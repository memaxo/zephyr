use crate::chain::governance::proposal::{Proposal, ProposalStatus};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: u64,
    pub vote_type: VoteType,
    pub timestamp: i64,
    pub signature: Vec<u8>,
}

pub struct Voting {
    pub votes: HashMap<u64, Vec<Vote>>,
    pub total_voting_power: u64,
    pub quorum: u64,
    pub threshold: u64,
}

impl Voting {
    pub fn new(total_voting_power: u64, quorum: u64, threshold: u64) -> Self {
        Voting {
            votes: HashMap::new(),
            total_voting_power,
            quorum,
            threshold,
        }
    }

    pub fn cast_vote(&mut self, vote: Vote) -> Result<(), String> {
        // Check if the vote is valid and the voter has enough voting power
        if !self.is_valid_vote(&vote) {
            return Err("Invalid vote".to_string());
        }

        // Add the vote to the corresponding proposal's vote list
        self.votes.entry(vote.proposal_id).or_default().push(vote);
        Ok(())
    }

    fn is_valid_vote(&self, vote: &Vote) -> bool {
        // Check if the vote signature is valid
        // Check if the voter has enough voting power
        // Implement the necessary validation logic here
        true
    }

    pub fn get_vote_counts(&self, proposal_id: u64) -> (u64, u64, u64) {
        let votes = self.votes.get(&proposal_id).unwrap_or(&Vec::new());
        let mut yes_count = 0;
        let mut no_count = 0;
        let mut abstain_count = 0;

        for vote in votes {
            match vote.vote_type {
                VoteType::Yes => yes_count += 1,
                VoteType::No => no_count += 1,
                VoteType::Abstain => abstain_count += 1,
            }
        }

        (yes_count, no_count, abstain_count)
    }

    pub fn get_voting_result(&self, proposal: &Proposal) -> ProposalStatus {
        let (yes_count, no_count, _) = self.get_vote_counts(proposal.id);
        let total_votes = yes_count + no_count;

        if total_votes < self.quorum {
            return ProposalStatus::Defeated;
        }

        let yes_ratio = (yes_count as f64) / (total_votes as f64);
        if yes_ratio >= (self.threshold as f64) / 100.0 {
            ProposalStatus::Succeeded
        } else {
            ProposalStatus::Defeated
        }
    }
}
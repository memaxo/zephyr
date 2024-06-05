use crate::chain::quantum_voting::errors::VotingError;
use crate::chain::quantum_voting::types::Candidate;
use log::{debug, info};
use std::collections::HashMap;

pub struct VoteTallying {}

impl VoteTallying {
    pub fn tally_votes(
        &self,
        votes: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<Candidate>, VotingError> {
        let mut vote_counts = HashMap::new();
        for (candidate_id, _) in votes {
            *vote_counts.entry(candidate_id).or_insert(0) += 1;
        }

        let num_representatives = self.voting_state.get_num_representatives()?;
        let mut representatives = Vec::new();
        for (candidate_id, vote_count) in vote_counts {
            if representatives.len() < num_representatives {
                let candidate = self.voting_state.get_candidate(candidate_id)?;
                representatives.push(candidate);
            } else {
                break;
            }
        }

        Ok(representatives)
    }
}

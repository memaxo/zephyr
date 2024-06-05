use crate::chain::quantum_voting::errors::VotingError;
use crate::chain::quantum_voting::types::{Candidate, VotingStatus};
use log::{debug, info};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct VotingState {
    voting_status: RwLock<VotingStatus>,
    candidates: RwLock<Vec<Candidate>>,
    num_representatives: usize,
    voting_duration: u64,
    vote_counts: RwLock<HashMap<String, usize>>,
}

impl VotingState {
    pub fn new() -> Self {
        VotingState {
            voting_status: RwLock::new(VotingStatus::NotStarted),
            candidates: RwLock::new(Vec::new()),
            num_representatives: 0,
            voting_duration: 0,
            vote_counts: RwLock::new(HashMap::new()),
        }
    }

    pub fn initialize_voting(
        &self,
        candidates: Vec<Candidate>,
        num_representatives: usize,
        voting_duration: u64,
    ) -> Result<(), VotingError> {
        debug!("Initializing voting state");

        let mut voting_status = self.voting_status.write().unwrap();
        if *voting_status != VotingStatus::NotStarted {
            return Err(VotingError::VotingAlreadyInitialized);
        }

        *voting_status = VotingStatus::InProgress;
        *self.candidates.write().unwrap() = candidates;
        self.num_representatives = num_representatives;
        self.voting_duration = voting_duration;
        self.vote_counts.write().unwrap().clear();

        info!("Voting state initialized successfully");
        Ok(())
    }

    pub fn is_voting_ongoing(&self) -> bool {
        *self.voting_status.read().unwrap() == VotingStatus::InProgress
    }

    pub fn get_candidates(&self) -> Vec<Candidate> {
        self.candidates.read().unwrap().clone()
    }

    pub fn get_num_representatives(&self) -> usize {
        self.num_representatives
    }

    pub fn get_voting_duration(&self) -> u64 {
        self.voting_duration
    }

    pub fn record_vote(&self, candidate_id: &str) -> Result<(), VotingError> {
        debug!("Recording vote for candidate: {}", candidate_id);

        if !self.is_voting_ongoing() {
            return Err(VotingError::VotingNotInProgress);
        }

        let mut vote_counts = self.vote_counts.write().unwrap();
        *vote_counts.entry(candidate_id.to_string()).or_insert(0) += 1;

        info!("Vote recorded successfully for candidate: {}", candidate_id);
        Ok(())
    }

    pub fn conclude_voting(&self) -> Result<(), VotingError> {
        debug!("Concluding voting");

        let mut voting_status = self.voting_status.write().unwrap();
        if *voting_status != VotingStatus::InProgress {
            return Err(VotingError::VotingNotInProgress);
        }

        *voting_status = VotingStatus::Concluded;

        info!("Voting concluded successfully");
        Ok(())
    }

    pub fn get_vote_counts(&self) -> HashMap<String, usize> {
        self.vote_counts.read().unwrap().clone()
    }
}

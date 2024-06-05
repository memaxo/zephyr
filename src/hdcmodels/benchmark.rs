use crate::error::{BenchmarkError, Result};
use crate::governance::AccuracyBenchmarkProposal;
use crate::state::StateManager;
use std::sync::Arc;

pub struct AccuracyBenchmark {
    state_manager: Arc<StateManager>,
}

impl AccuracyBenchmark {
    pub fn new(state_manager: Arc<StateManager>) -> Self {
        AccuracyBenchmark { state_manager }
    }

    pub fn get_current_accuracy_benchmark(&self) -> Result<f64> {
        let state = self.state_manager.get_state()?;
        let current_benchmark = state.get_accuracy_benchmark();

        match current_benchmark {
            Some(benchmark) => Ok(benchmark),
            None => Err(BenchmarkError::AccuracyBenchmarkNotFound.into()),
        }
    }

    pub fn update_accuracy_benchmark(&self, proposal: AccuracyBenchmarkProposal) -> Result<()> {
        let mut state = self.state_manager.get_state()?;

        // Verify the proposal has enough votes
        let total_votes = state.get_total_votes();
        let required_votes = (total_votes as f64 * 0.66) as u64; // Require 2/3 majority

        if proposal.votes < required_votes {
            return Err(BenchmarkError::InsufficientVotes.into());
        }

        // Update the accuracy benchmark
        state.set_accuracy_benchmark(proposal.new_benchmark);

        // Save the updated state
        self.state_manager.set_state(state)?;

        Ok(())
    }
}

use crate::chain::quantum_voting::errors::VotingError;
use crate::chain::quantum_voting::types::{QuantumState, TomographyResult};
use crate::crypto::quantum::{QuantumStateTomography, QuantumStateVerifier};
use log::{debug, error, info, trace};

pub struct QuantumStateVerification {
    tomography: QuantumStateTomography,
    verifier: QuantumStateVerifier,
}

impl crate::chain::quantum_voting::traits::QuantumStateVerification for QuantumStateVerification {
    pub fn new() -> Self {
        QuantumStateVerification {
            tomography: QuantumStateTomography::new(),
            verifier: QuantumStateVerifier::new(),
        }
    }

    fn verify_vote_state(&self, vote_state: &QuantumState) -> Result<bool, VotingError> {
        debug!("Verifying vote quantum state");

        // Perform quantum state tomography on the vote state
        let tomography_result = self.perform_tomography(vote_state)?;

        // Analyze the tomography result to verify the integrity and validity of the vote state
        let is_valid = self.analyze_tomography_result(&tomography_result)?;

        if is_valid {
            info!("Vote quantum state verified successfully");
        } else {
            error!("Vote quantum state verification failed");
        }

        Ok(is_valid)
    }

    fn verify_candidate_state(
        &self,
        candidate_state: &QuantumState,
    ) -> Result<bool, VotingError> {
        debug!("Verifying candidate quantum state");

        // Perform quantum state tomography on the candidate state
        let tomography_result = self.perform_tomography(candidate_state)?;

        // Analyze the tomography result to verify the integrity and validity of the candidate state
        let is_valid = self.analyze_tomography_result(&tomography_result)?;

        if is_valid {
            info!("Candidate quantum state verified successfully");
        } else {
            error!("Candidate quantum state verification failed");
        }

        Ok(is_valid)
    }

    fn perform_tomography(
        &self,
        quantum_state: &QuantumState,
    ) -> Result<TomographyResult, VotingError> {
        debug!("Performing quantum state tomography");

        // Perform quantum state tomography on the given quantum state
        let tomography_result = self.tomography.perform(quantum_state)?;

        trace!("Tomography result: {:?}", tomography_result);
        Ok(tomography_result)
    }

    fn analyze_tomography_result(
        &self,
        tomography_result: &TomographyResult,
    ) -> Result<bool, VotingError> {
        debug!("Analyzing tomography result");

        // Use the quantum state verifier to analyze the tomography result
        let is_valid = self.verifier.verify(tomography_result)?;

        trace!("Quantum state validity: {}", is_valid);
        Ok(is_valid)
    }
}

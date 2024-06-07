use crate::chain::quantum_voting::errors::VotingError;
use crate::chain::quantum_voting::types::{Candidate, Vote};
use crate::crypto::quantum::{QuantumGate, QuantumRegister};

pub fn encode_vote(vote_register: &mut QuantumRegister, vote: &Vote) -> Result<(), VotingError> {
    for (i, bit) in vote.to_string().chars().enumerate() {
        if bit == '1' {
            vote_register.apply_gate(QuantumGate::X, i)?;
        }
    }
    Ok(())
}

pub fn encode_candidate(
    candidate_register: &mut QuantumRegister,
    candidate: &Candidate,
) -> Result<(), VotingError> {
    for (i, bit) in candidate.id.chars().enumerate() {
        if bit == '1' {
            candidate_register.apply_gate(QuantumGate::X, i)?;
        }
    }
    Ok(())
}

pub fn decode_vote_data(vote_measurement: &QuantumMeasurement) -> Result<Vote, VotingError> {
    let vote_str = vote_measurement
        .outcomes
        .iter()
        .map(|&bit| if bit { '1' } else { '0' })
        .collect::<String>();
    let vote = Vote::from_str(&vote_str)?;
    Ok(vote)
}

pub fn decode_candidate_data(
    candidate_measurement: &QuantumMeasurement,
) -> Result<Candidate, VotingError> {
    let candidate_str = candidate_measurement
        .outcomes
        .iter()
        .map(|&bit| if bit { '1' } else { '0' })
        .collect::<String>();
    let (name, party) = candidate_str.split_at(candidate_str.len() / 2);
    let candidate = Candidate {
        name: name.to_string(),
        party: party.to_string(),
    };
    Ok(candidate)
}

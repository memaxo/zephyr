use crate::error_handling::Error;
use std::fmt;

#[derive(Debug)]
pub enum QUPError {
    InvalidBlock(String),
    InvalidTransaction(String),
    InvalidSignature(String),
    InvalidUsefulWork(String),
    InsufficientStake(String),
    InsufficientDelegatedStake(String),
    InvalidConsensusMessage(String),
    // Add more QUP-specific error variants as needed
}

impl fmt::Display for QUPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QUPError::InvalidBlock(msg) => write!(f, "Invalid block: {}", msg),
            QUPError::InvalidTransaction(msg) => write!(f, "Invalid transaction: {}", msg),
            QUPError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            QUPError::InvalidUsefulWork(msg) => write!(f, "Invalid useful work: {}", msg),
            QUPError::InsufficientStake(msg) => write!(f, "Insufficient stake: {}", msg),
            QUPError::InsufficientDelegatedStake(msg) => {
                write!(f, "Insufficient delegated stake: {}", msg)
            }
            QUPError::InvalidConsensusMessage(msg) => {
                write!(f, "Invalid consensus message: {}", msg)
            } // Add more error variant formatting as needed
        }
    }
}

impl std::error::Error for QUPError {}

#[derive(Debug)]
pub enum UsefulWorkError {
    InvalidProblem(String),
    InvalidSolution(String),
    VerificationFailed(String),
    // Add more useful work-specific error variants as needed
}

impl fmt::Display for UsefulWorkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UsefulWorkError::InvalidProblem(msg) => {
                write!(f, "Invalid useful work problem: {}", msg)
            }
            UsefulWorkError::InvalidSolution(msg) => {
                write!(f, "Invalid useful work solution: {}", msg)
            }
            UsefulWorkError::VerificationFailed(msg) => {
                write!(f, "Useful work verification failed: {}", msg)
            } // Add more error variant formatting as needed
        }
    }
}

impl std::error::Error for UsefulWorkError {}

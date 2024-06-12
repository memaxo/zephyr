use crate::error_handling::Error;
use std::fmt;

fn format_error(f: &mut fmt::Formatter, error_type: &str, msg: &str) -> fmt::Result {
    write!(f, "{}: {}", error_type, msg)
}

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
            QUPError::InvalidBlock(msg) => format_error(f, "Invalid block", msg),
            QUPError::InvalidTransaction(msg) => format_error(f, "Invalid transaction", msg),
            QUPError::InvalidSignature(msg) => format_error(f, "Invalid signature", msg),
            QUPError::InvalidUsefulWork(msg) => format_error(f, "Invalid useful work", msg),
            QUPError::InsufficientStake(msg) => format_error(f, "Insufficient stake", msg),
            QUPError::InsufficientDelegatedStake(msg) => format_error(f, "Insufficient delegated stake", msg),
            QUPError::InvalidConsensusMessage(msg) => format_error(f, "Invalid consensus message", msg),
            // Add more error variant formatting as needed
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
            UsefulWorkError::InvalidProblem(msg) => format_error(f, "Invalid useful work problem", msg),
            UsefulWorkError::InvalidSolution(msg) => format_error(f, "Invalid useful work solution", msg),
            UsefulWorkError::VerificationFailed(msg) => format_error(f, "Useful work verification failed", msg),
            // Add more error variant formatting as needed
        }
    }
}

impl std::error::Error for UsefulWorkError {}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Insufficient similarity")]
    InsufficientSimilarity,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid useful work proof")]
    InvalidUsefulWorkProof,
    #[error("Missing useful work proof")]
    MissingUsefulWorkProof,
    #[error("Unexpected message")]
    UnexpectedMessage,
}

use std::fmt;

#[derive(Debug)]
pub enum MarketplaceError {
    InvalidTaskError(String),
    InvalidBidError(String),
    TaskAssignmentError(String),
    RewardDistributionError(String),
}

impl fmt::Display for MarketplaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketplaceError::InvalidTaskError(msg) => write!(f, "Invalid Task: {}", msg),
            MarketplaceError::InvalidBidError(msg) => write!(f, "Invalid Bid: {}", msg),
            MarketplaceError::TaskAssignmentError(msg) => write!(f, "Task Assignment Error: {}", msg),
            MarketplaceError::RewardDistributionError(msg) => write!(f, "Reward Distribution Error: {}", msg),
        }
    }
}

impl std::error::Error for MarketplaceError {}

use std::fmt;

#[derive(Debug)]
pub enum MarketplaceError {
    InvalidTaskError(String),
    InvalidBidError(String),
    TaskAssignmentError(String),
    RewardDistributionError(String),
    NetworkError(String),
}

impl MarketplaceError {
    pub fn code(&self) -> u32 {
        match self {
            MarketplaceError::InvalidTaskError(_) => 1001,
            MarketplaceError::InvalidBidError(_) => 1002,
            MarketplaceError::TaskAssignmentError(_) => 1003,
            MarketplaceError::RewardDistributionError(_) => 1004,
            MarketplaceError::NetworkError(_) => 1005,
        }
    }
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

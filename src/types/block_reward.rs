use serde::{Deserialize, Serialize};
use crate::types::address::Address;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockReward {
    pub amount: u64,
    pub recipient: Address,
}

impl BlockReward {
    pub fn new(amount: u64, recipient: Address) -> Self {
        BlockReward { amount, recipient }
    }
}
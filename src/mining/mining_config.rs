use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    block_reward: u64,
    mining_interval: u64,
    transaction_fee_percentage: f64,
    qup_useful_work_difficulty: u64,
    qup_voting_threshold: u64,
    qup_validator_reward_ratio: f64,
    qup_delegator_reward_ratio: f64,
}

impl Default for MiningConfig {
    fn default() -> Self {
        MiningConfig {
            block_reward: 10,
            mining_interval: 5_000,
            transaction_fee_percentage: 0.01,
            qup_useful_work_difficulty: 100,
            qup_voting_threshold: 50,
            qup_validator_reward_ratio: 0.7,
            qup_delegator_reward_ratio: 0.3,
        }
    }
}

impl MiningConfig {
    pub fn new() -> Self {
        MiningConfig::default()
    }

    pub fn block_reward(&self) -> u64 {
        self.block_reward
    }

    pub fn mining_interval(&self) -> u64 {
        self.mining_interval
    }

    pub fn transaction_fee_percentage(&self) -> f64 {
        self.transaction_fee_percentage
    }

    pub fn qup_useful_work_difficulty(&self) -> u64 {
        self.qup_useful_work_difficulty
    }

    pub fn qup_voting_threshold(&self) -> u64 {
        self.qup_voting_threshold
    }

    pub fn qup_validator_reward_ratio(&self) -> f64 {
        self.qup_validator_reward_ratio
    }

    pub fn qup_delegator_reward_ratio(&self) -> f64 {
        self.qup_delegator_reward_ratio
    }
}

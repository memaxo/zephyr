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
    pub useful_work_reward_multiplier: u64,
    pub model_training_reward_multiplier: u64,
    pub high_load_threshold: f64,
    pub low_load_threshold: f64,
    pub target_utility_points: u64,
    pub fn useful_work_reward_multiplier(&self) -> u64 {
        self.useful_work_reward_multiplier
    }

    pub fn model_training_reward_multiplier(&self) -> u64 {
        self.model_training_reward_multiplier
    }

    pub fn high_load_threshold(&self) -> f64 {
        self.high_load_threshold
    }

    pub fn low_load_threshold(&self) -> f64 {
        self.low_load_threshold
    }

    pub fn target_utility_points(&self) -> u64 {
        self.target_utility_points
    }
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
            useful_work_reward_multiplier: 10,
            model_training_reward_multiplier: 5,
            high_load_threshold: 0.8,
            low_load_threshold: 0.2,
            target_utility_points: 1000,
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

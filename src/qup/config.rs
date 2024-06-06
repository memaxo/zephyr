use crate::consensus::ConsensusConfig;
use crate::crypto::hash::Hasher;
use crate::network::NetworkConfig;
use std::sync::Arc;

pub struct QUPConfig {
    pub consensus_config: ConsensusConfig,
    pub network_config: NetworkConfig,
    pub block_reward: u64,
    pub validator_reward_ratio: f64,
    pub useful_work_config: UsefulWorkConfig,
    pub reward_scheme: RewardScheme,
    pub supports_quantum: bool,
}

impl QUPConfig {
    pub fn new(
        consensus_config: ConsensusConfig,
        network_config: NetworkConfig,
        block_reward: u64,
        validator_reward_ratio: f64,
        useful_work_config: UsefulWorkConfig,
        reward_scheme: RewardScheme,
    ) -> Self {
        QUPConfig {
            consensus_config,
            network_config,
            block_reward,
            validator_reward_ratio,
            useful_work_config,
            reward_scheme,
        }
    }

    pub fn hasher(&self) -> Arc<dyn Hasher> {
        self.consensus_config.hasher()
    }
}

pub struct UsefulWorkConfig {
    pub problem_difficulty: u64,
    pub solution_accuracy_threshold: f64,
    pub max_attempts: u32,
}

impl UsefulWorkConfig {
    pub fn new(problem_difficulty: u64, solution_accuracy_threshold: f64, max_attempts: u32) -> Self {
        UsefulWorkConfig {
            problem_difficulty,
            solution_accuracy_threshold,
            max_attempts,
        }
    }
}

pub enum RewardScheme {
    FixedReward(u64),
    ProportionalReward {
        base_reward: u64,
        difficulty_factor: u64,
    },
    PerformanceBasedReward {
        base_reward: u64,
        performance_factor: f64,
    },
}

impl RewardScheme {
    pub fn calculate_rewards(&self, block: &QUPBlock) -> Result<HashMap<Address, u64>, Error> {
        match self {
            RewardScheme::FixedReward(amount) => {
                // Distribute a fixed reward to validators and delegators
                let mut rewards = HashMap::new();
                let total_stake = block.total_stake();
                for (address, stake) in block.stakes() {
                    let reward = amount * stake / total_stake;
                    rewards.insert(address, reward);
                }
                Ok(rewards)
            }
            RewardScheme::ProportionalReward {
                base_reward,
                difficulty_factor,
            } => {
                // Distribute rewards proportionally based on block difficulty
                let mut rewards = HashMap::new();
                let total_stake = block.total_stake();
                let total_reward = base_reward + (block.difficulty() * difficulty_factor);
                for (address, stake) in block.stakes() {
                    let reward = total_reward * stake / total_stake;
                    rewards.insert(address, reward);
                }
                Ok(rewards)
            }
            RewardScheme::PerformanceBasedReward {
                base_reward,
                performance_factor,
            } => {
                // Distribute rewards based on validator performance
                let mut rewards = HashMap::new();
                let total_stake = block.total_stake();
                let total_reward = base_reward + (block.performance() * performance_factor) as u64;
                for (address, stake) in block.stakes() {
                    let reward = total_reward * stake / total_stake;
                    rewards.insert(address, reward);
                }
                Ok(rewards)
            }
        }
    }
}
use crate::consensus::ConsensusConfig;
use crate::network::NetworkConfig;
use std::sync::Arc;

/// Configuration for the Quantum Uplink Protocol (QUP).
pub struct QUPConfig {
    pub consensus_config: ConsensusConfig,
    pub network_config: NetworkConfig,
    pub block_reward: u64,
    pub validator_reward_ratio: f64,
    pub useful_work_config: UsefulWorkConfig,
    pub reward_scheme: RewardScheme,
}

impl QUPConfig {
    pub fn new(
        consensus_config: ConsensusConfig,
        network_config: NetworkConfig,
        block_reward: u64,
        validator_reward_ratio: f64,
        useful_work_config: UsefulWorkConfig,
        reward_scheme: RewardScheme,
    ) -> Self {
        QUPConfig {
            consensus_config,
            network_config,
            block_reward,
            validator_reward_ratio,
            useful_work_config,
            reward_scheme,
        }
    }

    pub fn hasher(&self) -> Arc<dyn Hasher> {
        self.consensus_config.hasher()
    }
}

/// Configuration parameters for useful work generation and verification.
pub struct UsefulWorkConfig {
    // Configuration parameters for useful work generation and verification
    // ...
}

/// Different reward schemes for distributing rewards to validators and delegators.
pub enum RewardScheme {
    // Different reward schemes for distributing rewards to validators and delegators
    // ...
}

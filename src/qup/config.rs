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
    // Configuration parameters for useful work generation and verification
    // ...
}

pub enum RewardScheme {
    // Different reward schemes for distributing rewards to validators and delegators
    // ...
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

use crate::qup::types::{NodeId, BlockHeight, StakeAmount, RewardAmount, PenaltyAmount};
use crate::qup::block::QUPBlock;
use crate::qup::state::QUPState;
use std::collections::HashMap;

pub struct RewardManager {
    pub reward_rate: f64,
    pub penalty_rate: f64,
    pub useful_work_reward_percentage: f64,
}

impl RewardManager {
    pub fn new(reward_rate: f64, penalty_rate: f64, useful_work_reward_percentage: f64) -> Self {
        RewardManager {
            reward_rate,
            penalty_rate,
            useful_work_reward_percentage,
        }
    }

    pub fn calculate_rewards(&self, block: &QUPBlock, state: &QUPState) -> HashMap<NodeId, RewardAmount> {
        let mut rewards = HashMap::new();

        // Calculate validator reward
        let validator_reward = self.calculate_validator_reward(block, state);
        rewards.insert(block.validator.clone(), validator_reward);

        // Calculate delegator rewards
        let delegator_rewards = self.calculate_delegator_rewards(block, state);
        for (delegator, reward) in delegator_rewards {
            rewards.insert(delegator, reward);
        }

        // Calculate useful work rewards
        if let Some(useful_work_solution) = &block.useful_work_solution {
            let useful_work_reward = self.calculate_useful_work_reward(block, state);
            rewards.insert(useful_work_solution.provider.clone(), useful_work_reward);
        }

        rewards
    }

    fn calculate_validator_reward(&self, block: &QUPBlock, state: &QUPState) -> RewardAmount {
        let total_stake = state.total_stake();
        let validator_stake = state.get_stake(&block.validator);
        let block_reward = self.calculate_block_reward(block.height);

        (block_reward * validator_stake) / total_stake
    }

    fn calculate_delegator_rewards(&self, block: &QUPBlock, state: &QUPState) -> HashMap<NodeId, RewardAmount> {
        let mut rewards = HashMap::new();
        let total_stake = state.total_stake();
        let block_reward = self.calculate_block_reward(block.height);

        for (delegator, stake) in state.get_delegators(&block.validator) {
            let reward = (block_reward * stake) / total_stake;
            rewards.insert(delegator.clone(), reward);
        }

        rewards
    }

    fn calculate_useful_work_reward(&self, block: &QUPBlock, state: &QUPState) -> RewardAmount {
        let total_stake = state.total_stake();
        let block_reward = self.calculate_block_reward(block.height);

        (block_reward * self.useful_work_reward_percentage) / 100.0
    }

    fn calculate_block_reward(&self, height: BlockHeight) -> RewardAmount {
        // Implement the logic to calculate the block reward based on the block height
        // This can be a fixed amount or a dynamic formula
        1000.0 // Example fixed block reward
    }

    pub fn slash(&self, node_id: &NodeId, amount: PenaltyAmount, state: &mut QUPState) {
        // Deduct the slashed amount from the node's stake
        let stake = state.get_stake_mut(node_id);
        *stake -= amount;
    }
}
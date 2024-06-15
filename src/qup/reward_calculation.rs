use crate::qup::config::QUPConfig;
use crate::qup::types::QUPBlockHeader;
use std::collections::HashMap;

pub fn calculate_total_reward(config: &QUPConfig, block_header: &QUPBlockHeader) -> u64 {
    match config.reward_scheme {
        RewardScheme::FixedReward(amount) => amount,
        RewardScheme::ProportionalReward {
            base_reward,
            difficulty_factor,
        } => base_reward + (block_header.difficulty as u64 * difficulty_factor), // Add more reward schemes as needed
    }
}

pub fn calculate_validator_reward(total_reward: u64, contribution: u64, total_contribution: u64, reward_ratio: f64) -> u64 {
    let contribution_ratio = contribution as f64 / total_contribution as f64;
    (total_reward as f64 * contribution_ratio * reward_ratio) as u64
}

pub fn calculate_delegator_rewards(state: &State, total_delegator_reward: u64) -> HashMap<Vec<u8>, u64> {
    let mut delegator_rewards = HashMap::new();
    // Calculate the reward for each delegator based on their stake and the total delegator reward
    let total_stake: u64 = state
        .delegators()
        .map(|(_, stake)| stake)
        .sum();

    for (delegator_address, stake) in state.delegators() {
        let reward = (stake as f64 / total_stake as f64 * total_delegator_reward as f64) as u64;
        delegator_rewards.insert(delegator_address.clone(), reward);
    }

    delegator_rewards
}

use crate::chain::state::State;
use crate::qup::config::{QUPConfig, RewardScheme};
use crate::qup::reward_distributor::RewardDistributor;
use crate::qup::types::QUPBlockHeader;
use std::collections::HashMap;

#[test]
fn test_calculate_total_reward() {
    let config = QUPConfig {
        reward_scheme: RewardScheme::FixedReward(100),
        ..Default::default()
    };
    let reward_distributor = RewardDistributor::new(config);
    let block_header = QUPBlockHeader::default();

    let total_reward = reward_distributor.calculate_total_reward(&block_header);
    assert_eq!(total_reward, 100);
}

#[test]
fn test_calculate_validator_reward() {
    let config = QUPConfig {
        validator_reward_ratio: 0.8,
        ..Default::default()
    };
    let reward_distributor = RewardDistributor::new(config);
    let total_reward = 1000;

    let validator_reward = reward_distributor.calculate_validator_reward(total_reward);
    assert_eq!(validator_reward, 800);
}

#[test]
fn test_distribute_rewards() {
    let mut state = State::new();
    let config = QUPConfig {
        validator_reward_ratio: 0.6,
        reward_scheme: RewardScheme::ProportionalReward {
            base_reward: 100,
            difficulty_factor: 10,
        },
        ..Default::default()
    };
    let reward_distributor = RewardDistributor::new(config);
    let block_header = QUPBlockHeader {
        difficulty: 5,
        ..Default::default()
    };

    // Set up test accounts
    let validator_address = vec![1u8; 20];
    let delegator1_address = vec![2u8; 20];
    let delegator2_address = vec![3u8; 20];
    state.set_account_balance(&validator_address, 1000);
    state.set_account_balance(&delegator1_address, 2000);
    state.set_account_balance(&delegator2_address, 3000);

    // Distribute rewards
    reward_distributor.distribute_rewards(&mut state, &block_header);

    // Check validator reward
    let validator_balance = state.get_account_balance(&validator_address).unwrap();
    assert_eq!(validator_balance, 1000 + 150); // 60% of 250 (base reward + difficulty factor)

    // Check delegator rewards
    let delegator_rewards: HashMap<_, _> = vec![
        (delegator1_address, 2000 + 40), // 40% of 100 (remaining reward)
        (delegator2_address, 3000 + 60), // 60% of 100 (remaining reward)
    ]
    .into_iter()
    .collect();

    for (address, expected_balance) in delegator_rewards {
        let balance = state.get_account_balance(&address).unwrap();
        assert_eq!(balance, expected_balance);
    }
}

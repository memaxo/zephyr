use crate::chain::state::{Account, Address};
use crate::crypto::hash::Hash;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::delegator::QUPDelegator;
use crate::qup::state::QUPState;
use crate::qup::transaction::Transaction;
use crate::qup::validator::QUPValidator;
use std::sync::Arc;

#[test]
fn test_apply_block() {
    let config = Arc::new(QUPConfig::default());
    let mut state = QUPState::new(config);

    // Create some accounts
    let account1 = Account {
        address: Address::from_slice(&[1; 20]),
        balance: 1000,
    };
    let account2 = Account {
        address: Address::from_slice(&[2; 20]),
        balance: 2000,
    };
    state.update_account(account1).unwrap();
    state.update_account(account2).unwrap();

    // Create a block with transactions
    let transactions = vec![
        Transaction {
            sender: account1.address,
            recipient: account2.address,
            amount: 500,
            fee: 10,
        },
        Transaction {
            sender: account2.address,
            recipient: account1.address,
            amount: 200,
            fee: 20,
        },
    ];
    let block = QUPBlock {
        height: 1,
        prev_block_hash: Hash::default(),
        transactions: transactions.clone(),
        ..Default::default()
    };

    // Apply the block to the state
    state.apply_block(&block).unwrap();

    // Check account balances
    let account1_after = state.get_account(&account1.address).unwrap();
    let account2_after = state.get_account(&account2.address).unwrap();
    assert_eq!(account1_after.balance, 1000 - 500 - 10 + 200);
    assert_eq!(account2_after.balance, 2000 + 500 - 200 - 20);
}

#[test]
fn test_calculate_rewards() {
    let config = Arc::new(QUPConfig::default());
    let mut state = QUPState::new(config);

    // Create some validators and delegators
    let validator1 = QUPValidator {
        address: Address::from_slice(&[1; 20]),
        stake: 1000,
        ..Default::default()
    };
    let validator2 = QUPValidator {
        address: Address::from_slice(&[2; 20]),
        stake: 2000,
        ..Default::default()
    };
    state.update_validator(validator1.clone());
    state.update_validator(validator2.clone());

    let delegator1 = QUPDelegator {
        address: Address::from_slice(&[3; 20]),
        stake: 500,
        ..Default::default()
    };
    let delegator2 = QUPDelegator {
        address: Address::from_slice(&[4; 20]),
        stake: 1000,
        ..Default::default()
    };
    state.update_delegator(delegator1.clone());
    state.update_delegator(delegator2.clone());

    // Create a block
    let block = QUPBlock {
        height: 1,
        prev_block_hash: Hash::default(),
        ..Default::default()
    };

    // Calculate rewards
    let rewards = state.calculate_rewards(&block).unwrap();

    // Check rewards
    let validator1_reward = rewards
        .get(&validator1.address)
        .cloned()
        .unwrap_or_default();
    let validator2_reward = rewards
        .get(&validator2.address)
        .cloned()
        .unwrap_or_default();
    let delegator1_reward = rewards
        .get(&delegator1.address)
        .cloned()
        .unwrap_or_default();
    let delegator2_reward = rewards
        .get(&delegator2.address)
        .cloned()
        .unwrap_or_default();

    assert_eq!(validator1_reward, 1000 * 50 / 3000);
    assert_eq!(validator2_reward, 2000 * 50 / 3000);
    assert_eq!(delegator1_reward, 500 * 50 / 1500);
    assert_eq!(delegator2_reward, 1000 * 50 / 1500);
}

use crate::chain::state::{Account, Address};
use crate::crypto::hash::Hash;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::crypto::QUPKeyPair;
use crate::qup::hdcomm::UsefulWork;
use crate::qup::state::QUPState;
use crate::qup::transaction::Transaction;
use crate::qup::validator::QUPValidator;
use std::sync::Arc;

#[test]
fn test_stake_and_unstake() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let key_pair = QUPKeyPair::generate();
    let validator_address = Address::from_slice(&[1u8; 20]);

    let mut validator = QUPValidator::new(validator_address, key_pair, config.clone(), state.clone());

    // Set up validator account
    let mut account = Account {
        address: validator_address,
        balance: 1000,
    };
    state.set_account(&account.address, account.clone());

    // Stake
    validator.stake(500);
    assert_eq!(validator.stake, 500);
    let updated_account = state.get_account(&account.address).unwrap();
    assert_eq!(updated_account.balance, 500);

    // Unstake
    validator.unstake(200);
    assert_eq!(validator.stake, 300);
    let updated_account = state.get_account(&account.address).unwrap();
    assert_eq!(updated_account.balance, 700);
}

#[test]
fn test_propose_and_validate_block() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let key_pair = QUPKeyPair::generate();
    let validator_address = Address::from_slice(&[1u8; 20]);

    let validator = QUPValidator::new(validator_address, key_pair, config, state.clone());

    // Propose a block
    let transactions = vec![Transaction {
        sender: validator_address,
        recipient: Address::from_slice(&[2u8; 20]),
        amount: 100,
        fee: 10,
        data: vec![],
    }];
    let block = validator.propose_block(transactions).unwrap();

    // Validate the block
    assert!(validator.validate_block(&block).unwrap());

    // Validate an invalid block
    let mut invalid_block = block.clone();
    invalid_block.transactions = vec![Transaction {
        sender: Address::from_slice(&[3u8; 20]),
        recipient: Address::from_slice(&[4u8; 20]),
        amount: 1000000,
        fee: 0,
        data: vec![],
    }];
    assert!(!validator.validate_block(&invalid_block).unwrap());
}

#[test]
fn test_useful_work() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let key_pair = QUPKeyPair::generate();
    let validator_address = Address::from_slice(&[1u8; 20]);

    let validator = QUPValidator::new(validator_address, key_pair, config, state.clone());

    // Propose a block with useful work
    let transactions = vec![Transaction {
        sender: validator_address,
        recipient: Address::from_slice(&[2u8; 20]),
        amount: 100,
        fee: 10,
        data: vec![],
    }];
    let block = validator.propose_block(transactions).unwrap();

    // Verify useful work
    if let Some(useful_work) = &block.useful_work {
        assert!(useful_work.verify().unwrap());
    } else {
        panic!("No useful work found");
    }
}

use crate::chain::state::{Account, Address};
use crate::qup::config::QUPConfig;
use crate::qup::delegator::QUPDelegator;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use std::sync::Arc;

#[test]
fn test_stake_and_unstake() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));

    let address = Address::from_slice(&[1; 20]);
    let mut account = Account {
        address: address.clone(),
        balance: 1000,
    };
    state.set_account(&address, account.clone());

    let mut delegator = QUPDelegator::new(address, config, state.clone());

    // Stake
    delegator.stake(500);
    assert_eq!(delegator.stake, 500);
    let updated_account = state.get_account(&address).unwrap();
    assert_eq!(updated_account.balance, 500);

    // Unstake
    delegator.unstake(200);
    assert_eq!(delegator.stake, 300);
    let updated_account = state.get_account(&address).unwrap();
    assert_eq!(updated_account.balance, 700);
}

#[test]
fn test_delegate_and_undelegate() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));

    let delegator_address = Address::from_slice(&[1; 20]);
    let validator_address = Address::from_slice(&[2; 20]);

    let mut account = Account {
        address: delegator_address.clone(),
        balance: 1000,
    };
    state.set_account(&delegator_address, account.clone());

    let mut delegator = QUPDelegator::new(delegator_address, config.clone(), state.clone());
    delegator.stake(1000);

    let mut validator = QUPValidator::new(validator_address, config.clone(), state.clone());

    // Delegate
    delegator
        .delegate(&validator_address, 500)
        .expect("Failed to delegate");
    assert_eq!(delegator.get_delegated_validator(), Some(validator_address));
    assert_eq!(validator.get_delegated_stake(), 500);

    // Undelegate
    delegator.undelegate(200).expect("Failed to undelegate");
    assert_eq!(validator.get_delegated_stake(), 300);
}

#[test]
fn test_claim_rewards() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));

    let delegator_address = Address::from_slice(&[1; 20]);
    let validator_address = Address::from_slice(&[2; 20]);

    let mut account = Account {
        address: delegator_address.clone(),
        balance: 1000,
    };
    state.set_account(&delegator_address, account.clone());

    let mut delegator = QUPDelegator::new(delegator_address, config.clone(), state.clone());
    delegator.stake(1000);

    let mut validator = QUPValidator::new(validator_address, config.clone(), state.clone());
    validator.reward_rate = 0.1; // 10% reward rate

    delegator
        .delegate(&validator_address, 500)
        .expect("Failed to delegate");

    let reward_amount = delegator.claim_rewards().expect("Failed to claim rewards");
    assert_eq!(reward_amount, 50); // 10% of 500

    let updated_account = state.get_account(&delegator_address).unwrap();
    assert_eq!(updated_account.balance, 1050);
}

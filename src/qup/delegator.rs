use crate::chain::state::Account;
use crate::qup::config::QUPConfig;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use std::sync::Arc;

pub struct QUPDelegator {
    pub address: Address,
    pub stake: u64,
    pub delegated_to: Option<Address>,
    pub config: Arc<QUPConfig>,
    pub state: Arc<QUPState>,
}

impl QUPDelegator {
    pub fn new(address: Address, config: Arc<QUPConfig>, state: Arc<QUPState>) -> Self {
        QUPDelegator {
            address,
            stake: 0,
            delegated_to: None,
            config,
            state,
        }
    }

    pub fn stake(&mut self, amount: u64) {
        // Deduct the staked amount from the delegator's account balance
        let mut account = self.state.get_account(&self.address).unwrap();
        account.balance -= amount;
        self.state.set_account(&self.address, account);

        // Update the delegator's stake
        self.stake += amount;
    }

    pub fn unstake(&mut self, amount: u64) {
        // Ensure the delegator has sufficient staked amount
        assert!(self.stake >= amount, "Insufficient staked amount");

        // Add the unstaked amount back to the delegator's account balance
        let mut account = self.state.get_account(&self.address).unwrap();
        account.balance += amount;
        self.state.set_account(&self.address, account);

        // Update the delegator's stake
        self.stake -= amount;
    }

    pub fn delegate(&mut self, validator: &Address, amount: u64) -> Result<(), Error> {
        // Ensure the delegator has sufficient stake
        if self.stake < amount {
            return Err(Error::InsufficientStake("Insufficient stake".to_string()));
        }

        // Update the delegator's delegated_to field
        self.delegated_to = Some(*validator);

        // Update the corresponding validator's delegated stake
        let mut validator_state = self.state.get_validator_state(validator)?;
        validator_state.delegated_stake += amount;
        self.state.set_validator_state(validator, validator_state)?;

        // Update the delegator's account balance
        let mut account = self.state.get_account(&self.address)?;
        account.balance -= amount;
        self.state.set_account(&self.address, account)?;

        Ok(())
    }

    pub fn undelegate(&mut self, amount: u64) -> Result<(), Error> {
        // Ensure the delegator has delegated to a validator
        let validator = self.delegated_to.ok_or(Error::NoDelegation("No delegation found".to_string()))?;

        // Ensure the delegator has sufficient delegated amount
        let mut validator_state = self.state.get_validator_state(&validator)?;
        if validator_state.delegated_stake < amount {
            return Err(Error::InsufficientDelegatedStake("Insufficient delegated amount".to_string()));
        }

        // Update the corresponding validator's delegated stake
        validator_state.delegated_stake -= amount;
        self.state.set_validator_state(&validator, validator_state)?;

        // Update the delegator's account balance
        let mut account = self.state.get_account(&self.address)?;
        account.balance += amount;
        self.state.set_account(&self.address, account)?;

        // Clear the delegator's delegated_to field if all tokens are undelegated
        if validator_state.delegated_stake == 0 {
            self.delegated_to = None;
        }

        Ok(())
    }

    pub fn claim_rewards(&mut self) -> Result<u64, Error> {
        // Ensure the delegator has delegated to a validator
        let validator = self.delegated_to.ok_or(Error::NoDelegation)?;

        // Calculate the rewards based on the delegator's stake and the validator's performance
        let validator_state = self.state.get_validator_state(&validator)?;
        let reward_amount = self.stake * validator_state.reward_rate;

        // Update the delegator's account balance
        let mut account = self.state.get_account(&self.address).unwrap();
        account.balance += reward_amount;
        self.state.set_account(&self.address, account);

        Ok(reward_amount)
    }

    pub fn get_delegated_validator(&self) -> Option<Address> {
        self.delegated_to
    }

    pub fn get_voting_power(&self) -> u64 {
        // Calculate the delegator's voting power based on their stake and the total stake
        let total_stake = self.state.get_total_stake();
        if total_stake == 0 {
            return 0;
        }

        let voting_power = (self.stake as f64 / total_stake as f64) * 100.0;
        voting_power.round() as u64
    }
}

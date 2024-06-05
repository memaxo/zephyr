use crate::chain::state::{Account, StateDB};
use crate::crypto::hash::Hash;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::delegator::QUPDelegator;
use crate::qup::qup_hdcmodels::QUPHDCModels;
use crate::qup::validator::QUPValidator;
use std::collections::HashMap;
use std::sync::Arc;

pub struct QUPState {
    pub state_db: StateDB,
    pub validators: HashMap<Address, QUPValidator>,
    pub delegators: HashMap<Address, QUPDelegator>,
    pub latest_block_hash: Hash,
    pub latest_block_height: u64,
    pub total_stake: u64,
    pub config: Arc<QUPConfig>,
    pub hdc_models: QUPHDCModels,
}

impl QUPState {
    pub fn new(config: Arc<QUPConfig>, hdc_models: QUPHDCModels) -> Self {
        QUPState {
            state_db: StateDB::new(),
            validators: HashMap::new(),
            delegators: HashMap::new(),
            latest_block_hash: Hash::default(),
            latest_block_height: 0,
            total_stake: 0,
            config,
            hdc_models,
        }
    }

    pub fn get_account(&self, address: &Address) -> Result<Account, Error> {
        self.state_db.get_account(address)
    }

    pub fn update_account(&mut self, account: Account) -> Result<(), Error> {
        self.state_db.update_account(account)
    }

    pub fn get_validator(&self, address: &Address) -> Option<&QUPValidator> {
        self.validators.get(address)
    }

    pub fn update_validator(&mut self, validator: QUPValidator) {
        let address = validator.address.clone();
        self.validators.insert(address, validator);
    }

    pub fn get_delegator(&self, address: &Address) -> Option<&QUPDelegator> {
        self.delegators.get(address)
    }

    pub fn update_delegator(&mut self, delegator: QUPDelegator) {
        let address = delegator.address.clone();
        self.delegators.insert(address, delegator);
    }

    pub fn update_hdc_model(&mut self, block: &QUPBlock) {
        // Encode the block using HDC encoding
        let encoded_block = self.hdc_models.encode_block(block);

        // Update the HDC model with the encoded block
        self.hdc_models.update_model(&encoded_block);
    }

    pub fn get_optimized_block(&self, block: &QUPBlock) -> QUPBlock {
        // Optimize the block using HDC optimization
        let optimized_block = self.hdc_models.optimize_block(block);

        optimized_block
    }

    pub fn apply_block(&mut self, block: &QUPBlock) -> Result<(), Error> {
        // Update account balances based on the block transactions
        for transaction in &block.transactions {
            let sender = self.get_account(&transaction.sender)?;
            let recipient = self.get_account(&transaction.recipient)?;

            let sender_balance = sender.balance - transaction.amount - transaction.fee;
            let recipient_balance = recipient.balance + transaction.amount;

            self.update_account(Account {
                address: sender.address,
                balance: sender_balance,
            })?;
            self.update_account(Account {
                address: recipient.address,
                balance: recipient_balance,
            })?;
        }

        // Update validator stakes based on the block
        for (address, validator) in &mut self.validators {
            let account = self.get_account(address)?;
            validator.stake = account.balance;
        }

        // Update delegator information based on the block
        for (address, delegator) in &mut self.delegators {
            let account = self.get_account(address)?;
            delegator.stake = account.balance;
        }

        // Update the latest block hash and height
        self.latest_block_hash = block.hash();
        self.latest_block_height = block.height;

        // Update the total stake
        self.total_stake = self.validators.values().map(|v| v.stake).sum();

        // Update the HDC model with the new block
        self.update_hdc_model(block);

        Ok(())
    }

    pub fn calculate_rewards(&self, block: &QUPBlock) -> Result<HashMap<Address, u64>, Error> {
        let mut rewards = HashMap::new();

        // Calculate rewards for validators
        let validator_reward =
            self.config.block_reward * self.config.validator_reward_percentage / 100;
        let validator_stakes: Vec<(Address, u64)> = self
            .validators
            .iter()
            .map(|(a, v)| (a.clone(), v.stake))
            .collect();
        let total_validator_stake: u64 = validator_stakes.iter().map(|(_, s)| s).sum();

        for (validator_address, validator_stake) in validator_stakes {
            let reward = validator_reward * validator_stake / total_validator_stake;
            rewards.insert(validator_address, reward);
        }

        // Calculate rewards for delegators
        let delegator_reward = self.config.block_reward - validator_reward;
        let delegator_stakes: Vec<(Address, u64)> = self
            .delegators
            .iter()
            .map(|(a, d)| (a.clone(), d.stake))
            .collect();
        let total_delegator_stake: u64 = delegator_stakes.iter().map(|(_, s)| s).sum();

        for (delegator_address, delegator_stake) in delegator_stakes {
            let reward = delegator_reward * delegator_stake / total_delegator_stake;
            rewards.insert(delegator_address, reward);
        }

        Ok(rewards)
    }

    pub fn finalize_block(&mut self, block: &QUPBlock) -> Result<(), Error> {
        // Apply the block to the state
        self.apply_block(block)?;

        // Calculate and distribute rewards
        let rewards = self.calculate_rewards(block)?;
        for (address, reward) in rewards {
            let mut account = self.get_account(&address)?;
            account.balance += reward;
            self.update_account(account)?;
        }

        // Update the latest block hash and height
        self.latest_block_hash = block.hash();
        self.latest_block_height = block.height;

        Ok(())
    }

    pub fn rollback(&mut self, height: u64) -> Result<(), Error> {
        // Revert the state changes made by the blocks after the specified height
        while self.latest_block_height > height {
            let block = self
                .state_db
                .get_block_by_height(self.latest_block_height)?;

            // Revert account balances
            for transaction in &block.transactions {
                let sender = self.get_account(&transaction.sender)?;
                let recipient = self.get_account(&transaction.recipient)?;

                let sender_balance = sender.balance + transaction.amount + transaction.fee;
                let recipient_balance = recipient.balance - transaction.amount;

                self.update_account(Account {
                    address: sender.address,
                    balance: sender_balance,
                })?;
                self.update_account(Account {
                    address: recipient.address,
                    balance: recipient_balance,
                })?;
            }

            // Revert validator stakes and delegator information
            for (address, _) in &self.validators {
                let account = self.get_account(address)?;
                self.update_validator(QUPValidator {
                    address: address.clone(),
                    stake: account.balance,
                    ..Default::default()
                });
            }

            for (address, _) in &self.delegators {
                let account = self.get_account(address)?;
                self.update_delegator(QUPDelegator {
                    address: address.clone(),
                    stake: account.balance,
                    ..Default::default()
                });
            }

            // Update the latest block hash and height
            self.latest_block_hash = block.prev_block_hash;
            self.latest_block_height = block.height - 1;
        }

        Ok(())
    }
}

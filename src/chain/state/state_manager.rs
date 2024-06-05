use crate::chain::state::account::Account;
use crate::chain::state::state_db::StateDB;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct StateManager {
    state_db: Arc<RwLock<StateDB>>,
    account_cache: RwLock<HashMap<String, Account>>,
    qup_state: Arc<RwLock<QUPState>>,
}

impl StateManager {
    pub fn new(state_db: StateDB, qup_state: Arc<RwLock<QUPState>>) -> Self {
        StateManager {
            state_db: Arc::new(RwLock::new(state_db)),
            account_cache: RwLock::new(HashMap::new()),
            qup_state,
        }
    }

    pub fn get_account(&self, address: &str) -> Option<Account> {
        // Check the account cache first
        if let Some(account) = self.account_cache.read().unwrap().get(address) {
            return Some(account.clone());
        }

        // If not found in cache, retrieve from the state database
        if let Some(account) = self.state_db.read().unwrap().get_account(address) {
            // Update the account cache
            self.account_cache
                .write()
                .unwrap()
                .insert(address.to_string(), account.clone());
            Some(account)
        } else {
            None
        }
    }

    pub fn update_account(&self, account: &Account) {
        // Update the account in the state database
        self.state_db.write().unwrap().set_account(account);

        // Update the account cache
        self.account_cache
            .write()
            .unwrap()
            .insert(account.address.clone(), account.clone());
    }

    pub fn remove_account(&self, address: &str) {
        // Remove the account from the state database
        self.state_db.write().unwrap().remove_account(address);

        // Remove the account from the cache
        self.account_cache.write().unwrap().remove(address);
    }

    pub fn account_exists(&self, address: &str) -> bool {
        // Check the account cache first
        if self.account_cache.read().unwrap().contains_key(address) {
            return true;
        }

        // If not found in cache, check the state database
        self.state_db.read().unwrap().account_exists(address)
    }

    pub fn get_state_root(&self) -> Vec<u8> {
        self.state_db.read().unwrap().get_state_root()
    }

    pub fn generate_state_proof(&self, address: &str) -> Option<Vec<Vec<u8>>> {
        self.state_db.read().unwrap().generate_state_proof(address)
    }

    pub fn verify_state_proof(&self, address: &str, account: &Account, proof: &[Vec<u8>]) -> bool {
        self.state_db
            .read()
            .unwrap()
            .verify_state_proof(address, account, proof)
    }

    pub fn get_qup_state(&self) -> Arc<RwLock<QUPState>> {
        self.qup_state.clone()
    }

    pub fn update_qup_state(&self, new_state: QUPState) {
        let mut state = self.qup_state.write().unwrap();
        *state = new_state;
    }

    pub fn get_validator_state(&self, validator_id: &str) -> Option<QUPValidator> {
        self.qup_state.read().unwrap().get_validator(validator_id)
    }

    pub fn update_validator_state(&self, validator: QUPValidator) {
        self.qup_state.write().unwrap().update_validator(validator);
    }

    pub fn commit(&self) {
        // Commit the state changes to the underlying storage
        // This can involve flushing the cache to the state database
        // and updating the QUP state

        // Commit the account changes to the state database
        let account_cache = self.account_cache.read().unwrap();
        for (address, account) in account_cache.iter() {
            self.state_db.write().unwrap().set_account(account);
        }

        // Commit the QUP state changes
        let qup_state = self.qup_state.read().unwrap();
        // Implement the logic to persist the QUP state changes

        // Clear the account cache after committing the changes
        self.account_cache.write().unwrap().clear();
    }
}

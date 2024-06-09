use crate::state::account::Account;
use crate::state::state_storage::StateStorage;
use std::sync::Arc;

pub struct StateManager {
    storage: Arc<dyn StateStorage>,
}

impl StateManager {
    pub fn new(storage: Arc<dyn StateStorage>) -> Self {
        // TODO: Implement
        unimplemented!()
    }

    pub fn get_account(&self, address: &str) -> Option<Account> {
        // TODO: Implement
        unimplemented!()
    }

    pub fn update_account(&mut self, account: &Account) {
        // TODO: Implement
        unimplemented!()
    }

    pub fn remove_account(&mut self, address: &str) {
        // TODO: Implement
        unimplemented!()
    }

    // TODO: Add other state management-related methods
}

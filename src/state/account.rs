use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub storage: HashMap<String, String>,
}

impl Account {
    pub fn new(address: String, balance: u64) -> Self {
        // TODO: Implement
        unimplemented!()
    }

    pub fn get_balance(&self) -> u64 {
        // TODO: Implement
        unimplemented!()
    }

    pub fn set_balance(&mut self, balance: u64) {
        // TODO: Implement
        unimplemented!()
    }

    // TODO: Add other account-related methods
}

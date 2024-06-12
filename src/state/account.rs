use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub storage: HashMap<String, String>,
}

impl Account {
    pub fn new(address: String, balance: u64) -> Self {
        Account {
            address,
            balance,
            nonce: 0,
            storage: HashMap::new(),
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn get_storage(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn set_storage(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn remove_storage(&mut self, key: &str) {
        self.storage.remove(key);
    }

    pub fn has_storage_key(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.balance == 0 && self.nonce == 0 && self.storage.is_empty()
    }
}

impl Default for Account {
    fn default() -> Self {
        Account {
            address: String::new(),
            balance: 0,
            nonce: 0,
            storage: HashMap::new(),
        }
    }
}

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Treasury {
    pub balance: u64,
    pub allocations: HashMap<String, u64>,
}

impl Treasury {
    pub fn new(initial_balance: u64) -> Self {
        Treasury {
            balance: initial_balance,
            allocations: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds in the treasury".to_string())
        }
    }

    pub fn allocate(&mut self, allocation_id: String, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            self.allocations.insert(allocation_id, amount);
            Ok(())
        } else {
            Err("Insufficient funds in the treasury for allocation".to_string())
        }
    }

    pub fn deallocate(&mut self, allocation_id: &str) -> Result<(), String> {
        if let Some(amount) = self.allocations.remove(allocation_id) {
            self.balance += amount;
            Ok(())
        } else {
            Err("Allocation not found".to_string())
        }
    }

    pub fn get_allocation(&self, allocation_id: &str) -> Option<u64> {
        self.allocations.get(allocation_id).cloned()
    }

    pub fn transfer(&mut self, recipient: String, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            // Implement the logic to transfer funds to the recipient
            Ok(())
        } else {
            Err("Insufficient funds in the treasury for transfer".to_string())
        }
    }
}
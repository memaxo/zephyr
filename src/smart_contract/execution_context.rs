use crate::smart_contract::types::{Value, TransactionContext};
use std::collections::{HashMap, VecDeque};

pub struct ExecutionContext {
    pub state: HashMap<String, Value>,
    pub transaction_stack: VecDeque<TransactionContext>,
    pub gas_used: u64,
    pub gas_limit: u64,
}

impl ExecutionContext {
    pub fn new(gas_limit: u64) -> Self {
        ExecutionContext {
            state: HashMap::new(),
            transaction_stack: VecDeque::new(),
            gas_used: 0,
            gas_limit,
        }
    }

    pub fn start_transaction(&mut self) {
        self.transaction_stack.push_front(TransactionContext { changes: HashMap::new() });
    }

    pub fn commit_transaction(&mut self) {
        if let Some(tx) = self.transaction_stack.pop_front() {
            for (key, value) in tx.changes {
                self.state.insert(key, value);
            }
        }
    }

    pub fn rollback_transaction(&mut self) {
        self.transaction_stack.pop_front();
    }

    pub fn set_value(&mut self, key: String, value: Value) {
        if let Some(tx) = self.transaction_stack.front_mut() {
            tx.changes.insert(key, value);
        } else {
            self.state.insert(key, value);
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        for tx in &self.transaction_stack {
            if let Some(value) = tx.changes.get(key) {
                return Some(value);
            }
        }
        self.state.get(key)
    }

    pub fn has_value(&self, key: &str) -> bool {
        for tx in &self.transaction_stack {
            if tx.changes.contains_key(key) {
                return true;
            }
        }
        self.state.contains_key(key)
    }

    pub fn remove_value(&mut self, key: &str) {
        if let Some(tx) = self.transaction_stack.front_mut() {
            tx.changes.remove(key);
        } else {
            self.state.remove(key);
        }
    }

    pub fn clear_state(&mut self) {
        self.state.clear();
        self.transaction_stack.clear();
    }

    pub fn use_gas(&mut self, amount: u64) -> Result<(), String> {
        if self.gas_used + amount > self.gas_limit {
            return Err(format!("Insufficient gas. Used: {}, Limit: {}", self.gas_used, self.gas_limit));
        }
        self.gas_used += amount;
        Ok(())
    }

    pub fn reset_gas(&mut self) {
        self.gas_used = 0;
    }

    pub fn set_gas_limit(&mut self, limit: u64) {
        self.gas_limit = limit;
    }
}
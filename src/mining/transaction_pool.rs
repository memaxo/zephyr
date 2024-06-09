use crate::chain::transaction::Transaction;
use crate::storage::Storage;
use crate::state::account::Account;
use log::debug;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct TransactionPool {
    transactions: Mutex<HashSet<Transaction>>,
    capacity: usize,
}

impl TransactionPool {
    pub fn new(capacity: usize) -> Self {
        TransactionPool {
            transactions: Mutex::new(HashSet::new()),
            capacity,
        }
    }

    pub fn add_transaction(&self, transaction: Transaction, storage: &dyn Storage) -> bool {
        let mut transactions = self.transactions.lock().unwrap();
        // Ensure transactions have valid history proofs using storage
        if let Err(e) = transaction.verify_history_proof(storage) {
            error!("History proof verification failed for transaction: {}", e);
            return false;
        }
        if transactions.len() >= self.capacity {
            debug!("Transaction pool is full. Skipping transaction: {:?}", transaction);
            return false;
        }

        if transactions.insert(transaction) {
            debug!("Added transaction to the pool: {:?}", transaction);
            true
        } else {
            debug!("Transaction already exists in the pool: {:?}", transaction);
            false
        }
    }

    pub fn remove_transaction(&self, transaction: &Transaction) -> bool {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.remove(transaction)
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.iter().cloned().collect()
    }

    pub fn clear(&self) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.clear();
    }

    pub fn len(&self) -> usize {
        let transactions = self.transactions.lock().unwrap();
        transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        let transactions = self.transactions.lock().unwrap();
        transactions.is_empty()
    }

    pub fn contains(&self, transaction: &Transaction) -> bool {
        let transactions = self.transactions.lock().unwrap();
        transactions.contains(transaction)
    }
}

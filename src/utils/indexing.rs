use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::chain::{Blockchain, Transaction};

/// Indexes transactions by address for quick lookup.
pub struct TransactionIndex {
    by_sender: HashMap<String, HashSet<String>>,   // Maps sender address to transaction IDs
    by_receiver: HashMap<String, HashSet<String>>, // Maps receiver address to transaction IDs
}

impl TransactionIndex {
    pub fn new() -> Self {
        TransactionIndex {
            by_sender: HashMap::new(),
            by_receiver: HashMap::new(),
        }
    }

    /// Adds a transaction to the index.
    pub fn index_transaction(&mut self, transaction: &Transaction) {
        self.by_sender
            .entry(transaction.sender.clone())
            .or_insert_with(HashSet::new)
            .insert(transaction.transaction_id.clone());

        self.by_receiver
            .entry(transaction.receiver.clone())
            .or_insert_with(HashSet::new)
            .insert(transaction.transaction_id.clone());
    }

    /// Retrieves transaction IDs for a given address, either as sender or receiver.
    pub fn get_transactions_for_address(&self, address: &str) -> HashSet<String> {
        let mut transactions = HashSet::new();

        if let Some(sender_transactions) = self.by_sender.get(address) {
            transactions.extend(sender_transactions.iter().cloned());
        }

        if let Some(receiver_transactions) = self.by_receiver.get(address) {
            transactions.extend(receiver_transactions.iter().cloned());
        }

        transactions
    }
}

/// Manages balance calculations for addresses, caching results for efficiency.
pub struct BalanceManager {
    balances: HashMap<String, f64>, // Cached balances for each address
}

impl BalanceManager {
    pub fn new() -> Self {
        BalanceManager {
            balances: HashMap::new(),
        }
    }

    /// Updates the balance for a given address based on a transaction.
    pub fn update_balance(&mut self, transaction: &Transaction) {
        *self.balances.entry(transaction.sender.clone()).or_insert(0.0) -= transaction.amount;
        *self.balances.entry(transaction.receiver.clone()).or_insert(0.0) += transaction.amount;
    }

    /// Retrieves the balance for a given address.
    pub fn get_balance(&self, address: &str) -> f64 {
        self.balances.get(address).copied().unwrap_or(0.0)
    }
}

/// Integrates indexing and balance management into the blockchain.
pub struct BlockchainIndexer {
    pub transaction_index: TransactionIndex,
    pub balance_manager: BalanceManager,
}

impl BlockchainIndexer {
    pub fn new() -> Self {
        BlockchainIndexer {
            transaction_index: TransactionIndex::new(),
            balance_manager: BalanceManager::new(),
        }
    }

    /// Processes a new transaction, updating indexes and balances.
    pub fn process_transaction(&mut self, transaction: &Transaction) {
        self.transaction_index.index_transaction(transaction);
        self.balance_manager.update_balance(transaction);
    }
}
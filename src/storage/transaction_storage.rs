use crate::chain::transaction::Transaction;
use crate::chain::storage::database::Database;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionStorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
}

pub struct TransactionStorage {
    db: Box<dyn Database>,
}

impl TransactionStorage {
    pub fn new(db: Box<dyn Database>) -> Self {
        TransactionStorage { db }
    }

    // Implement transaction storage methods here
    // ...
}

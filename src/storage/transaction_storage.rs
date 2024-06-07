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

    pub fn save_transaction(&self, transaction: &Transaction) -> Result<(), TransactionStorageError> {
        let tx_id = transaction.id().to_string();
        let data = serde_json::to_vec(transaction)
            .map_err(|e| TransactionStorageError::DatabaseError(e.to_string()))?;
        self.db.put(&tx_id, &data)
            .map_err(|e| TransactionStorageError::DatabaseError(e.to_string()))
    }

    pub fn load_transaction(&self, tx_id: &str) -> Result<Transaction, TransactionStorageError> {
        match self.db.get(tx_id) {
            Some(data) => {
                let transaction: Transaction = serde_json::from_slice(&data)
                    .map_err(|e| TransactionStorageError::DatabaseError(e.to_string()))?;
                Ok(transaction)
            }
            None => Err(TransactionStorageError::TransactionNotFound(tx_id.to_string())),
        }
    }
}

use crate::chain::block::Block;
use crate::chain::transaction::Transaction;
use crate::storage::state_storage::StateStorageError;
use bincode;
use serde::{Deserialize, Serialize};

pub fn serialize_block(block: &Block) -> Result<Vec<u8>, StateStorageError> {
    bincode::serialize(block).map_err(|e| StateStorageError::DatabaseError(e.to_string()))
}

pub fn deserialize_block(data: &[u8]) -> Result<Block, StateStorageError> {
    bincode::deserialize(data).map_err(|e| StateStorageError::DatabaseError(e.to_string()))
}

pub fn serialize_transaction(transaction: &Transaction) -> Result<Vec<u8>, StateStorageError> {
    bincode::serialize(transaction).map_err(|e| StateStorageError::DatabaseError(e.to_string()))
}

pub fn deserialize_transaction(data: &[u8]) -> Result<Transaction, StateStorageError> {
    bincode::deserialize(data).map_err(|e| StateStorageError::DatabaseError(e.to_string()))
}

// Add other encoding and decoding functions here

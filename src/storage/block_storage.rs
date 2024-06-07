use crate::chain::block::Block;
use crate::chain::storage::database::Database;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockStorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Block not found: {0}")]
    BlockNotFound(String),
}

pub struct BlockStorage {
    db: Box<dyn Database>,
}

impl BlockStorage {
    pub fn new(db: Box<dyn Database>) -> Self {
        BlockStorage { db }
    }

    // Implement block storage methods here
    // ...
}

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

    pub async fn load_block(&self, hash: &str) -> Result<Block, BlockStorageError> {
        let block_data = self.db.get(hash).map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
        let block: Block = bincode::deserialize(&block_data).map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
        Ok(block)
    }

    pub async fn save_block(&self, block: &Block) -> Result<(), BlockStorageError> {
        let block_data = bincode::serialize(block).map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
        self.db.put(&block.hash, &block_data).map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

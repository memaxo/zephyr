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

    pub fn add_block(&self, block: &Block) -> Result<(), BlockStorageError> {
        let block_hash = block.hash.clone();
        let block_data = serde_json::to_vec(block)
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        self.db
            .put(block_hash.as_bytes(), &block_data)
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> Result<Option<Block>, BlockStorageError> {
        let block_data = self
            .db
            .get(block_hash.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        match block_data {
            Some(data) => {
                let block = serde_json::from_slice(&data)
                    .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockStorageError> {
        let height_key = format!("height:{}", height);
        let block_hash = self
            .db
            .get(height_key.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        match block_hash {
            Some(hash) => {
                let block_data = self
                    .db
                    .get(&hash)
                    .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

                match block_data {
                    Some(data) => {
                        let block = serde_json::from_slice(&data)
                            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
                        Ok(Some(block))
                    }
                    None => Err(BlockStorageError::BlockNotFound(format!(
                        "Block not found for height: {}",
                        height
                    ))),
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_latest_block(&self) -> Result<Option<Block>, BlockStorageError> {
        let latest_height = self.get_latest_block_height()?;
        match latest_height {
            Some(height) => self.get_block_by_height(height),
            None => Ok(None),
        }
    }

    pub fn get_latest_block_height(&self) -> Result<Option<u64>, BlockStorageError> {
        let latest_height_key = "latest_height";
        let latest_height = self
            .db
            .get(latest_height_key.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        match latest_height {
            Some(height_data) => {
                let height = String::from_utf8_lossy(&height_data)
                    .parse()
                    .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
                Ok(Some(height))
            }
            None => Ok(None),
        }
    }

    pub fn update_latest_block_height(&self, height: u64) -> Result<(), BlockStorageError> {
        let latest_height_key = "latest_height";
        let height_data = height.to_string();

        self.db
            .put(latest_height_key.as_bytes(), height_data.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
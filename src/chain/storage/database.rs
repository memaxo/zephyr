use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

pub trait Database: Send + Sync {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DatabaseError>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), DatabaseError>;
    fn delete(&self, key: &[u8]) -> Result<(), DatabaseError>;
    fn iterate(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, DatabaseError>;
    fn iterate_with_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, DatabaseError>;
}

pub struct MemoryDatabase {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemoryDatabase {
    pub fn new() -> Self {
        MemoryDatabase {
            data: HashMap::new(),
        }
    }
}

impl Database for MemoryDatabase {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DatabaseError> {
        Ok(self.data.get(key).cloned())
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), DatabaseError> {
        self.data.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    fn delete(&self, key: &[u8]) -> Result<(), DatabaseError> {
        self.data.remove(key);
        Ok(())
    }

    fn iterate(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, DatabaseError> {
        Ok(self.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    fn iterate_with_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, DatabaseError> {
        Ok(self
            .data
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }
}
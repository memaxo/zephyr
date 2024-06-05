use crate::chain::block::Block;
use crate::network::p2p::Message::State;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnapshotError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub block: Block,
    pub state: State,
}

impl Snapshot {
    pub fn new(block: Block, state: State) -> Self {
        Snapshot { block, state }
    }

    pub fn save(&self, path: &Path) -> Result<(), SnapshotError> {
        let serialized = serde_json::to_vec(&self)
            .map_err(|e| SnapshotError::SerializationError(e.to_string()))?;

        let mut file = File::create(path)?;
        file.write_all(&serialized)?;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, SnapshotError> {
        let file = File::open(path)?;
        let snapshot = serde_json::from_reader(file)
            .map_err(|e| SnapshotError::DeserializationError(e.to_string()))?;

        Ok(snapshot)
    }

    pub fn delete(path: &Path) -> Result<(), SnapshotError> {
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }
}
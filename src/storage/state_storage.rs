use crate::chain::state::ChainState;
use crate::chain::storage::database::Database;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateStorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("State not found: {0}")]
    StateNotFound(String),
}

pub struct StateStorage {
    db: Box<dyn Database>,
}

impl StateStorage {
    pub fn new(db: Box<dyn Database>) -> Self {
        StateStorage { db }
    }

    pub fn save_state(&self, state_id: &str, state: &ChainState) -> Result<(), StateStorageError> {
        let data = serde_json::to_vec(state)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
        self.db.put(state_id, &data)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))
    }

    pub fn load_state(&self, state_id: &str) -> Result<ChainState, StateStorageError> {
        match self.db.get(state_id) {
            Some(data) => {
                let state: ChainState = serde_json::from_slice(&data)
                    .map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
                Ok(state)
            }
            None => Err(StateStorageError::StateNotFound(state_id.to_string())),
        }
    }

    pub fn load_state(&self, state_id: &str) -> Result<ChainState, StateStorageError> {
        match self.db.get(state_id) {
            Some(data) => {
                let state: ChainState = serde_json::from_slice(&data)
                    .map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
                Ok(state)
            }
            None => Err(StateStorageError::StateNotFound(state_id.to_string())),
        }
    }

    pub fn save_state(&self, state_id: &str, state: &ChainState) -> Result<(), StateStorageError> {
        let data = serde_json::to_vec(state)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
        self.db.put(state_id, &data)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))
    }
}

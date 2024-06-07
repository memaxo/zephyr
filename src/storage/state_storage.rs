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

    pub fn load_state(&self, state_id: &str) -> Result<ChainState, StateStorageError> {
        // Implement the logic to load the state from the database
        // This is a placeholder implementation
        Err(StateStorageError::StateNotFound(state_id.to_string()))
    }

    pub fn save_state(&self, state_id: &str, state: &ChainState) -> Result<(), StateStorageError> {
        // Implement the logic to save the state to the database
        // This is a placeholder implementation
        Ok(())
    }
}

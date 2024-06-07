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

    // Implement state storage methods here
    // ...
}

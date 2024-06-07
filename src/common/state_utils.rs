use crate::chain::state::ChainState;
use crate::storage::state_storage::{StateStorage, StateStorageError};
use std::sync::{Arc, Mutex};

pub fn load_state(storage: &StateStorage, state_id: &str) -> Result<ChainState, StateStorageError> {
    // Implement the logic to load the state from the storage
    // This is a placeholder implementation
    Err(StateStorageError::StateNotFound(state_id.to_string()))
}

pub fn save_state(storage: &StateStorage, state: &ChainState) -> Result<(), StateStorageError> {
    // Implement the logic to save the state to the storage
    // This is a placeholder implementation
    Ok(())
}

pub fn update_account_balance(
    state: &mut ChainState,
    account_id: &str,
    amount: f64,
) -> Result<(), String> {
    // Implement the logic to update the account balance in the state
    // This is a placeholder implementation
    Ok(())
}

pub fn remove_account(state: &mut ChainState, account_id: &str) -> Result<(), String> {
    if state.accounts.remove(account_id).is_some() {
        Ok(())
    } else {
        Err(format!("Account {} not found", account_id))
    }
}

pub fn account_exists(state: &ChainState, account_id: &str) -> bool {
    state.accounts.contains_key(account_id)
}

// Add other common state operations here

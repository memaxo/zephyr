use crate::state::account::Account;
use crate::state::block::Block;
use crate::state::state_manager::StateManager;
use crate::state::transaction::Transaction;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateTransitionError {
    #[error("Sender account not found: {0}")]
    SenderAccountNotFound(String),
    #[error("Receiver account not found: {0}")]
    ReceiverAccountNotFound(String),
    #[error("Insufficient balance. Sender: {0}, Balance: {1}, Transaction amount: {2}")]
    InsufficientBalance(String, u64, u64),
    #[error("Invalid transaction nonce. Expected: {0}, Actual: {1}")]
    InvalidTransactionNonce(u64, u64),
    #[error("State update error: {0}")]
    StateUpdateError(String),
    #[error("Block validation failed: {0}")]
    BlockValidationFailed(String),
    #[error("Sender and receiver cannot be the same address: {0}")]
    SenderReceiverSameAddress(String),
    #[error("State update failed due to an inconsistent state")]
    InconsistentStateError,
}

pub struct StateTransition {
    state_manager: Arc<StateManager>,
    qup_state: Arc<QUPState>,
    qup_crypto: Arc<QUPCrypto>,
}

impl StateTransition {
    pub fn new(state_manager: Arc<StateManager>, qup_state: Arc<QUPState>, qup_crypto: Arc<QUPCrypto>) -> Self {
        StateTransition { state_manager, qup_state, qup_crypto }
    }

    pub fn apply(&self, transaction: &Transaction) -> Result<(), StateTransitionError> {
        let sender_address = &transaction.sender;
        let receiver_address = &transaction.receiver;

        if sender_address == receiver_address {
            return Err(StateTransitionError::SenderReceiverSameAddress(
                sender_address.clone(),
            ));
        }

        let mut sender_account = self
            .state_manager
            .get_account(sender_address)
            .ok_or_else(|| StateTransitionError::SenderAccountNotFound(sender_address.clone()))?;

        let mut receiver_account = self
            .state_manager
            .get_account(receiver_address)
            .ok_or_else(|| {
                StateTransitionError::ReceiverAccountNotFound(receiver_address.clone())
            })?;

        self.validate_transaction(&sender_account, transaction)?;

        sender_account.balance -= transaction.amount;
        receiver_account.balance += transaction.amount;
        sender_account.nonce += 1;

        self.state_manager
            .update_accounts(&[&sender_account, &receiver_account])
            .map_err(|_| StateTransitionError::InconsistentStateError)?;

        // Apply QUP-specific state changes
        self.apply_qup_state_changes(transaction)?;
        // Revert QUP-specific state changes
        self.revert_qup_state_changes(transaction)?;
        // Validate QUP-specific state changes
        self.validate_qup_state_changes(block)?;
        // Revert QUP-specific state changes
        self.revert_qup_block_state_changes(block)?;
        Ok(())
    }

    fn revert_qup_block_state_changes(&self, block: &Block) -> Result<(), StateTransitionError> {
        // Implement the logic to revert QUP-specific state changes for a block
        Ok(())
    }

    fn validate_qup_state_changes(&self, block: &Block) -> Result<(), StateTransitionError> {
        // Implement the logic to validate QUP-specific state changes
        Ok(())
    }

    fn revert_qup_state_changes(&self, transaction: &Transaction) -> Result<(), StateTransitionError> {
        // Implement the logic to revert QUP-specific state changes
        Ok(())
    }

    fn apply_qup_state_changes(&self, transaction: &Transaction) -> Result<(), StateTransitionError> {
        // Implement the logic to apply QUP-specific state changes
        Ok(())
    }

    fn validate_transaction(
        &self,
        sender_account: &Account,
        transaction: &Transaction,
    ) -> Result<(), StateTransitionError> {
        if sender_account.balance < transaction.amount {
            return Err(StateTransitionError::InsufficientBalance(
                sender_account.address.clone(),
                sender_account.balance,
                transaction.amount,
            ));
        }

        if transaction.nonce != sender_account.nonce {
            return Err(StateTransitionError::InvalidTransactionNonce(
                sender_account.nonce,
                transaction.nonce,
            ));
        }

        Ok(())
    }

    pub fn apply_block(&self, block: &Block) -> Result<(), StateTransitionError> {
        crossbeam_utils::thread::scope(|s| {
            let results: Vec<Result<(), StateTransitionError>> = block
                .transactions
                .par_iter()
                .map(|transaction| {
                    s.spawn(|_| {
                        self.apply(transaction).map_err(|e| {
                            StateTransitionError::StateUpdateError(format!(
                                "Failed to apply transaction: {}",
                                e
                            ))
                        })
                    })
                })
                .collect();

            // Check if any errors occurred during parallel execution
            for result in results {
                result?;
            }

            Ok(())
        })
        .unwrap();
            self.apply(transaction).map_err(|e| {
                StateTransitionError::StateUpdateError(format!(
                    "Failed to apply transaction: {}",
                    e
                ))
            })
        })
        .unwrap();

        let state_root = self.state_manager.get_state_root();
        if block.header.state_root != state_root {
            // Revert the applied transactions if the state root doesn't match
            self.revert_block(block).map_err(|_| {
                StateTransitionError::BlockValidationFailed("State root mismatch".to_string())
            })?;
            return Err(StateTransitionError::BlockValidationFailed(
                "State root mismatch".to_string(),
            ));
        }

        Ok(())
    }

    pub fn revert_block(&self, block: &Block) -> Result<(), StateTransitionError> {
        crossbeam_utils::thread::scope(|s| {
            let results: Vec<Result<(), StateTransitionError>> = block
                .transactions
                .par_iter()
                .rev()
                .map(|transaction| {
                    s.spawn(|_| {
                        self.revert(transaction).map_err(|e| {
                            StateTransitionError::StateUpdateError(format!(
                                "Failed to revert transaction: {}",
                                e
                            ))
                        })
                    })
                })
                .collect();

            // Check if any errors occurred during parallel execution
            for result in results {
                result?;
            }

            Ok(())
        })
        .unwrap();
            self.revert(transaction).map_err(|e| {
                StateTransitionError::StateUpdateError(format!(
                    "Failed to revert transaction: {}",
                    e
                ))
            })
        })
        .unwrap();

        Ok(())
    }


    fn revert(&self, transaction: &Transaction) -> Result<(), StateTransitionError> {
        let sender_address = &transaction.sender;
        let receiver_address = &transaction.receiver;

        let mut sender_account = self
            .state_manager
            .get_account(sender_address)
            .ok_or_else(|| StateTransitionError::SenderAccountNotFound(sender_address.clone()))?;

        let mut receiver_account = self
            .state_manager
            .get_account(receiver_address)
            .ok_or_else(|| {
                StateTransitionError::ReceiverAccountNotFound(receiver_address.clone())
            })?;

        sender_account.balance += transaction.amount;
        receiver_account.balance -= transaction.amount;
        sender_account.nonce -= 1;

        self.state_manager
            .update_accounts(&[&sender_account, &receiver_account])
            .map_err(|_| StateTransitionError::InconsistentStateError)?;

        Ok(())
    }
}

// Separate module for state transition-related functionality
pub(crate) mod state_transition_utils {
    use super::*;

    pub fn validate_block_transactions(
        state_transition: &StateTransition,
        block: &Block,
    ) -> Result<(), StateTransitionError> {
        for transaction in &block.transactions {
            let sender_address = &transaction.sender;
            let sender_account = state_transition
                .state_manager
                .get_account(sender_address)
                .ok_or_else(|| {
                    StateTransitionError::SenderAccountNotFound(sender_address.clone())
                })?;

            state_transition.validate_transaction(&sender_account, transaction)?;
        }

        Ok(())
    }

    // Add more utility functions as needed
}

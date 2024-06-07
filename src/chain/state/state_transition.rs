use crate::chain::block::Block;
use crate::chain::state::account::Account;
use crate::chain::state::state_manager::StateManager;
use crate::chain::transaction::Transaction;
use crate::qup::crypto::QUPCrypto;
use crate::qup::state::QUPState;
use std::sync::Arc;

pub struct StateTransition {
    state_manager: Arc<StateManager>,
    qup_state: Arc<QUPState>,
    qup_crypto: Arc<QUPCrypto>,
}

impl StateTransition {
    pub fn new(
        state_manager: Arc<StateManager>,
        qup_state: Arc<QUPState>,
        qup_crypto: Arc<QUPCrypto>,
    ) -> Self {
        StateTransition {
            state_manager,
            qup_state,
            qup_crypto,
        }
    }

    pub fn apply(&self, transaction: &Transaction) -> Result<(), String> {
        // Retrieve the sender and receiver accounts from the QUPState
        let sender_address = &transaction.sender;
        let receiver_address = &transaction.receiver;

        let mut sender_account = self
            .qup_state
            .get_account(sender_address)
            .ok_or_else(|| format!("Sender account not found: {}", sender_address))?;

        let mut receiver_account = self
            .qup_state
            .get_account(receiver_address)
            .unwrap_or_else(|| Account::new(receiver_address.clone(), 0));

        // Validate the transaction using post-quantum cryptography
        self.validate_transaction(&sender_account, transaction)?;

        // Apply the state transition
        sender_account.balance -= transaction.amount;
        receiver_account.balance += transaction.amount;
        sender_account.nonce += 1;

        // Update the accounts in the QUPState
        self.qup_state.update_account(&sender_account);
        self.qup_state.update_account(&receiver_account);

        // Update the accounts in the local state
        self.state_manager.update_account(&sender_account);
        self.state_manager.update_account(&receiver_account);

        // Apply QUP-specific state changes
        self.qup_state.apply_state_changes(transaction)?;

        // Revert QUP-specific state changes
        self.qup_state.revert_state_changes(transaction)?;

        Ok(())
    }

    fn validate_transaction(
        &self,
        sender_account: &Account,
        transaction: &Transaction,
    ) -> Result<(), String> {
        // Check if the sender has sufficient balance
        if sender_account.balance < transaction.amount {
            return Err(format!(
                "Insufficient balance. Sender: {}, Balance: {}, Transaction amount: {}",
                sender_account.address, sender_account.balance, transaction.amount
            ));
        }

        // Check if the transaction nonce matches the sender's nonce
        if transaction.nonce != sender_account.nonce {
            return Err(format!(
                "Invalid transaction nonce. Expected: {}, Actual: {}",
                sender_account.nonce, transaction.nonce
            ));
        }

        // Verify the transaction signature using post-quantum cryptography
        if !self.qup_crypto.verify_transaction_signature(transaction)? {
            return Err("Invalid transaction signature".to_string());
        }

        // Add more validation checks as needed

        Ok(())
    }

    pub fn apply_block(&self, block: &Block) -> Result<(), String> {
        // Apply state transitions for each transaction in the block
        for transaction in &block.transactions {
            self.apply(transaction)?;
        }

        // Update the state root in the block header
        let state_root = self.state_manager.get_state_root();
        block.header.state_root = state_root;

        Ok(())
    }

    pub fn revert_block(&self, block: &Block) -> Result<(), String> {
        // Revert state transitions for each transaction in the block (in reverse order)
        for transaction in block.transactions.iter().rev() {
            self.revert(transaction)?;
        }

        Ok(())
    }

    fn revert(&self, transaction: &Transaction) -> Result<(), String> {
        // Retrieve the sender and receiver accounts from the QUPState
        let sender_address = &transaction.sender;
        let receiver_address = &transaction.receiver;

        let mut sender_account = self
            .qup_state
            .get_account(sender_address)
            .ok_or_else(|| format!("Sender account not found: {}", sender_address))?;

        let mut receiver_account = self
            .qup_state
            .get_account(receiver_address)
            .ok_or_else(|| format!("Receiver account not found: {}", receiver_address))?;

        // Revert the state transition
        sender_account.balance += transaction.amount;
        receiver_account.balance -= transaction.amount;
        sender_account.nonce -= 1;

        // Update the accounts in the QUPState
        self.qup_state.update_account(&sender_account);
        self.qup_state.update_account(&receiver_account);

        // Update the accounts in the local state
        self.state_manager.update_account(&sender_account);
        self.state_manager.update_account(&receiver_account);

        Ok(())
    }
}

use crate::chain::transaction::Transaction;
use crate::qup::crypto::QUPCrypto;
use crate::qup::state::QUPState;
use crate::chain::validation::ValidationError;
use anyhow::Result;

pub fn validate_transaction(
    transaction: &Transaction,
    qup_crypto: &QUPCrypto,
    qup_state: &QUPState,
) -> Result<(), ValidationError> {
    // Check if the transaction amount is positive (excluding zero)
    if transaction.amount <= 0 {
        return Err(ValidationError::InvalidTransactionAmount);
    }

    // Verify the transaction's post-quantum signature
    qup_crypto
        .verify_transaction_signature(transaction)
        .map_err(ValidationError::TransactionValidationError)?;

    // Check if the sender has sufficient account balance
    let sender_account = qup_state
        .get_account(&transaction.sender)
        .ok_or(ValidationError::InsufficientAccountBalance)?;
    if sender_account.balance < transaction.amount {
        return Err(ValidationError::InsufficientAccountBalance);
    }

    // Verify the transaction's nonce
    if transaction.nonce != sender_account.nonce + 1 {
        return Err(ValidationError::InvalidNonce);
    }

    // Verify the transaction's post-quantum signature
    qup_crypto
        .verify_transaction_signature(transaction)
        .map_err(ValidationError::TransactionValidationError)?;

    // Send TransactionValidated message upon successful validation
    let validated_message = Message::TransactionValidated(transaction.clone());
    // Assuming you have a function to send messages to the Useful Work Node
    send_message_to_useful_work_node(validated_message);

    Ok(())
}

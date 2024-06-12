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

    // Verify the zero-knowledge proof
    let proof_inputs = [
        transaction.sender.as_bytes(),
        transaction.receiver.as_bytes(),
        transaction.amount.to_le_bytes().to_vec(),
        transaction.encrypted_details.clone(),
        transaction.timestamp.to_le_bytes().to_vec(), // Include timestamp for uniqueness
    ]
    .concat();
    zkp_crate::verify_proof(&transaction.proof.proof_hash, &proof_inputs)
        .map_err(ValidationError::ZKProofVerificationFailed)?;

    // Check if the sender has sufficient account balance
    let sender_account = qup_state
        .get_account(&transaction.sender)
        .ok_or(ValidationError::InsufficientAccountBalance)?;
    if sender_account.balance < transaction.amount {
        return Err(ValidationError::InsufficientAccountBalance);
    }

    // Perform additional transaction validation checks specific to the Zephyr project
    // ...

    Ok(())
}

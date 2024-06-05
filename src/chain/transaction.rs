use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, KeyInit, Nonce,
};
use log::{debug, error, info, trace};
use rand::{rngs::OsRng, RngCore};
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey, VerifyOnly};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use zeroize::Zeroize;

use crate::chain::crypto::{
    PostQuantumKeyPair, PostQuantumPublicKey, PostQuantumSignature, QUPCrypto,
};
use crate::key_management::{KeyManagementError, KeyManager};
use crate::logging::LoggingError;
use crate::qup::state::QUPState;
use crate::zkp_crate::{generate_proof, verify_proof, ZKProofError};

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(#[from] secp256k1::Error),
    #[error("Zero-knowledge proof error: {0}")]
    ZKProofError(String),
    #[error("Post-quantum signature error: {0}")]
    PostQuantumSignatureError(String),
    #[error("Key management error: {0}")]
    KeyManagementError(#[from] KeyManagementError),
    #[error("Logging error: {0}")]
    LoggingError(#[from] LoggingError),
    #[error("State update error: {0}")]
    StateUpdateError(String),
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("QUP state update error: {0}")]
    QUPStateUpdateError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub nonce: u64,
    #[serde(skip)]
    pub signature: Vec<u8>,
    pub proof: Proof,
    #[serde(skip)]
    pub encrypted_details: Vec<u8>,
    pub post_quantum_signature: Option<PostQuantumSignature>,
    pub useful_work_solution: Option<UsefulWorkSolution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub proof_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsefulWorkSolution {
    pub problem_type: String,
    pub solution: Vec<u8>,
}

impl Transaction {
    /// Encrypts transaction details.
    pub fn encrypt_details(&mut self, key: &[u8]) -> Result<(), TransactionError> {
        let key = Aes256Gcm::new_from_slice(key).map_err(|_| {
            TransactionError::EncryptionError("Invalid key length for encryption".to_string())
        })?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let plaintext = serde_json::to_vec(&self).map_err(TransactionError::SerializationError)?;
        let ciphertext = key
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| TransactionError::EncryptionError(format!("Encryption failed: {}", e)))?;
        self.encrypted_details = ciphertext;
        Ok(())
    }

    /// Decrypts transaction details.
    pub fn decrypt_details(&self, key: &[u8]) -> Result<Self, TransactionError> {
        let key = Aes256Gcm::new_from_slice(key).map_err(|_| {
            TransactionError::DecryptionError("Invalid key length for decryption".to_string())
        })?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let decrypted_details = key
            .decrypt(nonce, &self.encrypted_details)
            .map_err(|e| TransactionError::DecryptionError(format!("Decryption failed: {}", e)))?;
        serde_json::from_slice(&decrypted_details).map_err(TransactionError::SerializationError)
    }

    /// Batch verifies transaction signatures using the provided public keys.
    /// Returns `Ok(true)` if all signatures are valid, `Ok(false)` if any are invalid, or an error if verification fails.
    pub fn verify_signatures(
        transactions: &[Transaction],
        public_keys: &[PublicKey],
        qup_crypto: &QUPCrypto,
    ) -> Result<bool, TransactionError> {
        let mut valid = true;
        for (tx, pubkey) in transactions.iter().zip(public_keys) {
            if let Err(e) = tx.verify_signature(pubkey, qup_crypto) {
                error!("Signature verification failed for transaction: {}", e);
                valid = false;
            }
        }
        Ok(valid)
    }

    /// Signs the transaction with the sender's private key and generates a zero-knowledge proof.
    pub fn sign_and_generate_proof(
        &mut self,
        key_manager: &KeyManager,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), TransactionError> {
        info!("Signing transaction: {} -> {}", self.sender, self.receiver);
        let private_key = key_manager.get_private_key(&self.sender)?;
        self.sign(&private_key)?;

        let post_quantum_keypair = key_manager.get_post_quantum_keypair(&self.sender)?;
        self.sign_with_post_quantum_key(&post_quantum_keypair, qup_crypto)?;

        let proof_bytes = generate_proof(
            &self.sender,
            &self.receiver,
            self.amount,
            &self.encrypted_details,
        )
        .map_err(|e| {
            TransactionError::ZKProofError(format!(
                "Failed to generate zero-knowledge proof: {}",
                e
            ))
        })?;
        self.proof = Proof {
            proof_hash: hex::encode(proof_bytes),
        };
        Ok(())
    }

    /// Verifies the transaction's signature.
    pub fn verify_signature(
        &self,
        public_key: &PublicKey,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), TransactionError> {
        qup_crypto
            .verify_signature(public_key, &self.signature, &self.calculate_hash())
            .map_err(TransactionError::Secp256k1Error)
    }

    /// Signs the transaction with the sender's private key.
    pub fn sign(&mut self, private_key: &SecretKey) -> Result<(), TransactionError> {
        let secp = Secp256k1::signing_only();
        let message = Message::from_slice(&self.calculate_hash()).map_err(|_| {
            TransactionError::Secp256k1Error("Failed to create message from hash".to_string())
        })?;
        let (sig, _) = secp.sign_ecdsa(&message, private_key);
        self.signature = sig.serialize_compact().to_vec();
        Ok(())
    }

    /// Signs the transaction with the sender's post-quantum private key.
    pub fn sign_with_post_quantum_key(
        &mut self,
        post_quantum_keypair: &PostQuantumKeyPair,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), TransactionError> {
        info!(
            "Signing transaction with post-quantum key: {} -> {}",
            self.sender, self.receiver
        );
        let message = self.calculate_hash();
        let signature = qup_crypto
            .sign_with_post_quantum_key(post_quantum_keypair, &message)
            .map_err(|e| {
                TransactionError::PostQuantumSignatureError(format!(
                    "Failed to sign with post-quantum key: {}",
                    e
                ))
            })?;
        self.post_quantum_signature = Some(signature);
        Ok(())
    }

    /// Verifies the transaction's post-quantum signature.
    pub fn verify_post_quantum_signature(
        &self,
        post_quantum_public_key: &PostQuantumPublicKey,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), TransactionError> {
        if let Some(signature) = &self.post_quantum_signature {
            let message = self.calculate_hash();
            qup_crypto
                .verify_post_quantum_signature(post_quantum_public_key, &message, signature)
                .map_err(|e| {
                    TransactionError::PostQuantumSignatureError(format!(
                        "Post-quantum signature verification failed: {}",
                        e
                    ))
                })
        } else {
            Err(TransactionError::PostQuantumSignatureError(
                "Post-quantum signature not found".to_string(),
            ))
        }
    }

    /// Applies the transaction to the given state.
    pub fn apply_to_state(&self, state: &mut Arc<RwLock<State>>) -> Result<(), TransactionError> {
        let mut state_lock = state.write().unwrap();

        // Validate the transaction
        self.validate(&state_lock)?;

        // Update the sender's account
        let sender_account = state_lock.get_account_mut(&self.sender).ok_or_else(|| {
            TransactionError::InvalidTransaction(format!(
                "Sender account not found: {}",
                self.sender
            ))
        })?;
        if sender_account.balance < self.amount {
            return Err(TransactionError::InvalidTransaction(format!(
                "Insufficient balance for sender: {}",
                self.sender
            )));
        }
        if sender_account.nonce != self.nonce {
            return Err(TransactionError::InvalidTransaction(format!(
                "Invalid nonce for sender: {}",
                self.sender
            )));
        }
        sender_account.balance -= self.amount;
        sender_account.nonce += 1;

        // Update the receiver's account
        let receiver_account = state_lock
            .get_account_mut(&self.receiver)
            .unwrap_or_else(|| state_lock.create_account(&self.receiver));
        receiver_account.balance += self.amount;

        Ok(())
    }

    /// Validates the transaction against the current state.
    pub fn validate(&self, state: &State) -> Result<(), TransactionError> {
        // Check if the sender account exists
        if !state.account_exists(&self.sender) {
            return Err(TransactionError::InvalidTransaction(format!(
                "Sender account not found: {}",
                self.sender
            )));
        }

        // Check if the transaction amount is positive
        if self.amount <= 0.0 {
            return Err(TransactionError::InvalidTransaction(
                "Transaction amount must be positive".to_string(),
            ));
        }

        // Check if the sender has sufficient balance
        let sender_account = state.get_account(&self.sender).ok_or_else(|| {
            TransactionError::InvalidTransaction(format!(
                "Sender account not found: {}",
                self.sender
            ))
        })?;
        if sender_account.balance < self.amount {
            return Err(TransactionError::InvalidTransaction(format!(
                "Insufficient balance for sender: {}",
                self.sender
            )));
        }

        // Check if the nonce is valid
        if sender_account.nonce != self.nonce {
            return Err(TransactionError::InvalidTransaction(format!(
                "Invalid nonce for sender: {}",
                self.sender
            )));
        }

        // Verify the transaction signature
        self.verify_signature(&self.sender_public_key()?)?;

        // Verify the post-quantum signature
        self.verify_post_quantum_signature(&self.sender_post_quantum_public_key()?)?;

        // Verify the zero-knowledge proof
        let proof_data = [
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            &self.amount.to_be_bytes(),
            self.sp_key.expose_secret(),
        ]
        .concat();
        verify_proof(&self.proof.proof_hash, &proof_data).map_err(|e| {
            TransactionError::ZKProofError(format!("Failed to verify zero-knowledge proof: {}", e))
        })?;

        Ok(())
    }

    /// Applies the transaction to the given QUP state.
    pub fn apply_to_qup_state(
        &self,
        qup_state: &mut Arc<RwLock<QUPState>>,
    ) -> Result<(), TransactionError> {
        let mut qup_state_lock = qup_state.write().unwrap();

        // Validate the transaction
        self.validate_for_qup(&qup_state_lock)?;

        // Update the sender's account in the QUP state
        let sender_account = qup_state_lock
            .get_account_mut(&self.sender)
            .ok_or_else(|| {
                TransactionError::QUPStateUpdateError(format!(
                    "Sender account not found: {}",
                    self.sender
                ))
            })?;
        if sender_account.balance < self.amount {
            return Err(TransactionError::QUPStateUpdateError(format!(
                "Insufficient balance for sender: {}",
                self.sender
            )));
        }
        sender_account.balance -= self.amount;

        // Update the receiver's account in the QUP state
        let receiver_account = qup_state_lock
            .get_account_mut(&self.receiver)
            .unwrap_or_else(|| qup_state_lock.create_account(&self.receiver));
        receiver_account.balance += self.amount;

        // Update the validator's stake if the transaction includes a useful work solution
        if let Some(solution) = &self.useful_work_solution {
            let validator = qup_state_lock
                .get_validator_mut(&self.sender)
                .ok_or_else(|| {
                    TransactionError::QUPStateUpdateError(format!(
                        "Validator not found: {}",
                        self.sender
                    ))
                })?;
            validator.update_useful_work_score(solution);
        }

        Ok(())
    }

    /// Validates the transaction against the current QUP state.
    pub fn validate_for_qup(&self, qup_state: &QUPState) -> Result<(), TransactionError> {
        // Check if the sender account exists in the QUP state
        if !qup_state.account_exists(&self.sender) {
            return Err(TransactionError::QUPStateUpdateError(format!(
                "Sender account not found: {}",
                self.sender
            )));
        }

        // Check if the transaction amount is positive
        if self.amount <= 0.0 {
            return Err(TransactionError::QUPStateUpdateError(
                "Transaction amount must be positive".to_string(),
            ));
        }

        // Check if the sender has sufficient balance in the QUP state
        let sender_account = qup_state.get_account(&self.sender).ok_or_else(|| {
            TransactionError::QUPStateUpdateError(format!(
                "Sender account not found: {}",
                self.sender
            ))
        })?;
        if sender_account.balance < self.amount {
            return Err(TransactionError::QUPStateUpdateError(format!(
                "Insufficient balance for sender: {}",
                self.sender
            )));
        }

        // Verify the transaction signature
        self.verify_signature(&self.sender_public_key()?)?;

        // Verify the post-quantum signature
        self.verify_post_quantum_signature(&self.sender_post_quantum_public_key()?)?;

        // Verify the zero-knowledge proof
        let proof_data = [
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            &self.amount.to_be_bytes(),
            self.sp_key.expose_secret(),
        ]
        .concat();
        verify_proof(&self.proof.proof_hash, &proof_data).map_err(|e| {
            TransactionError::ZKProofError(format!("Failed to verify zero-knowledge proof: {}", e))
        })?;

        // Verify the useful work solution if present
        if let Some(solution) = &self.useful_work_solution {
            qup_state.verify_useful_work(solution).map_err(|e| {
                TransactionError::QUPStateUpdateError(format!(
                    "Failed to verify useful work solution: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }
}

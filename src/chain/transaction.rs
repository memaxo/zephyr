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
use anyhow::{Context, Result};
use zeroize::Zeroize;

use crate::chain::crypto::{
    PostQuantumKeyPair, PostQuantumPublicKey, PostQuantumSignature, QUPCrypto,
};
use crate::chain::common::{TransactionCommon, TransactionFields};
use crate::key_management::{KeyManagementError, KeyManager};
use crate::logging::LoggingError;
use crate::qup::state::QUPState;
use crate::zkp_crate::{generate_proof, verify_proof, ZKProofError};


#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub common: TransactionFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryProof {
    pub proof_data: Vec<u8>,
    pub proof_hash: String,
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

impl TransactionCommon for Transaction {
    fn encrypt_details(&mut self, key: &[u8]) -> Result<()> {
        let key = Aes256Gcm::new_from_slice(key)
            .context("Invalid key length for encryption")?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let plaintext = serde_json::to_vec(&self)
            .context("Failed to serialize transaction details")?;
        let ciphertext = key
            .encrypt(nonce, plaintext.as_ref())
            .context("Encryption failed")?;
        self.common.encrypted_details = ciphertext;
        Ok(())
    }

    fn decrypt_details(&self, key: &[u8]) -> Result<Self> {
        let key = Aes256Gcm::new_from_slice(key)
            .context("Invalid key length for decryption")?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let decrypted_details = key
            .decrypt(nonce, &self.common.encrypted_details)
            .context("Decryption failed")?;
        serde_json::from_slice(&decrypted_details)
            .context("Failed to deserialize transaction details")
    }

    fn verify_signature(&self, public_key: &PublicKey, qup_crypto: &QUPCrypto) -> Result<()> {
        let transaction_data = bincode::serialize(&self).context("Failed to serialize transaction")?;
        if qup_crypto.verify_transaction_signature(&transaction_data, &self.common.signature, &public_key.serialize()) {
            Ok(())
        } else {
            anyhow::bail!("Failed to verify transaction signature")
        }
    }

    fn sign(&mut self, private_key: &SecretKey) -> Result<()> {
        let secp = Secp256k1::signing_only();
        let message = Message::from_slice(&self.calculate_hash())
            .context("Failed to create message from hash")?;
        let (sig, _) = secp.sign_ecdsa(&message, private_key);
        self.common.signature = sig.serialize_compact().to_vec();
        Ok(())
    }

    fn sign_with_post_quantum_key(&mut self, post_quantum_keypair: &PostQuantumKeyPair, qup_crypto: &QUPCrypto) -> Result<()> {
        let message = self.calculate_hash();
        let signature = qup_crypto
            .sign_with_post_quantum_key(post_quantum_keypair, &message)
            .context("Failed to sign with post-quantum key")?;
        self.common.post_quantum_signature = Some(signature);
        Ok(())
    }

    fn verify_post_quantum_signature(&self, post_quantum_public_key: &PostQuantumPublicKey, qup_crypto: &QUPCrypto) -> Result<()> {
        if let Some(signature) = &self.common.post_quantum_signature {
            let message = self.calculate_hash();
            qup_crypto
                .verify_post_quantum_signature(post_quantum_public_key, &message, signature)
                .context("Post-quantum signature verification failed")
        } else {
            anyhow::bail!("Post-quantum signature not found")
        }
    }

    fn apply_to_state(&self, state: &mut Arc<RwLock<State>>) -> Result<()> {
        let mut state_lock = state.write().unwrap();

        // Validate the transaction
        self.validate(&state_lock)?;

        // Update the sender's account
        let sender_account = state_lock
            .get_account_mut(&self.common.sender)
            .context(format!("Sender account not found: {}", self.common.sender))?;
        if sender_account.balance < self.common.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.common.sender));
        }
        if sender_account.nonce != self.common.nonce {
            anyhow::bail!(format!("Invalid nonce for sender: {}", self.common.sender));
        }
        sender_account.balance -= self.common.amount;
        sender_account.nonce += 1;

        // Update the receiver's account
        let receiver_account = state_lock
            .get_account_mut(&self.common.receiver)
            .unwrap_or_else(|| state_lock.create_account(&self.common.receiver));
        receiver_account.balance += self.common.amount;

        Ok(())
    }

    fn validate(&self, state: &State) -> Result<()> {
        use crate::chain::validation::transaction_validator::validate_transaction;
        validate_transaction(self, state)
    }
}

impl Transaction {
    /// Encrypts transaction details.
    pub fn encrypt_details(&mut self, key: &[u8]) -> Result<()> {
        let key = Aes256Gcm::new_from_slice(key)
            .context("Invalid key length for encryption")?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let plaintext = serde_json::to_vec(&self)
            .context("Failed to serialize transaction details")?;
        let ciphertext = key
            .encrypt(nonce, plaintext.as_ref())
            .context("Encryption failed")?;
        self.encrypted_details = ciphertext;
        // Verify the history proof if present
        if let Some(proof) = &self.history_proof {
            qup_state
                .verify_history_proof(proof)
                .context("Failed to verify history proof")?;
        }

        Ok(())
    }

    /// Verifies the transaction's history proof.
    pub fn verify_history_proof(
        &self,
        qup_state: &QUPState,
    ) -> Result<()> {
        if let Some(proof) = &self.history_proof {
            qup_state
                .verify_history_proof(proof)
                .context("History proof verification failed")
        } else {
            anyhow::bail!("History proof not found")
        }
    }

    /// Decrypts transaction details.
    pub fn decrypt_details(&self, key: &[u8]) -> Result<Self> {
        let key = Aes256Gcm::new_from_slice(key)
            .context("Invalid key length for decryption")?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let decrypted_details = key
            .decrypt(nonce, &self.encrypted_details)
            .context("Decryption failed")?;
        serde_json::from_slice(&decrypted_details)
            .context("Failed to deserialize transaction details")
    }

    /// Batch verifies transaction signatures using the provided public keys.
    /// Returns `Ok(true)` if all signatures are valid, `Ok(false)` if any are invalid, or an error if verification fails.
    pub fn verify_signatures(
        transactions: &[Transaction],
        public_keys: &[PublicKey],
        qup_crypto: &QUPCrypto,
    ) -> Result<bool> {
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
        qup_crypto: &QUPCrypto,
        history_proof: Option<HistoryProof>,
    ) -> Result<()> {
        self.history_proof = history_proof;
        info!("Signing transaction: {} -> {}", self.sender, self.receiver);
        let private_key = key_manager
            .get_private_key(&self.sender)
            .context("Failed to get private key for sender")?;
        self.sign(&private_key)?;

        let post_quantum_keypair = key_manager
            .get_post_quantum_keypair(&self.sender)
            .context("Failed to get post-quantum keypair for sender")?;
        self.sign_with_post_quantum_key(&post_quantum_keypair, qup_crypto)?;

        let proof_bytes = generate_proof(
            &self.sender,
            &self.receiver,
            self.amount,
            &self.encrypted_details,
        )
        .context("Failed to generate zero-knowledge proof")?;
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
    ) -> Result<()> {
        qup_crypto
            .verify_signature(public_key, &self.signature, &self.calculate_hash())
            .context("Failed to verify transaction signature")
    }

    /// Signs the transaction with the sender's private key.
    pub fn sign(&mut self, private_key: &SecretKey) -> Result<()> {
        let secp = Secp256k1::signing_only();
        let message = Message::from_slice(&self.calculate_hash())
            .context("Failed to create message from hash")?;
        let (sig, _) = secp.sign_ecdsa(&message, private_key);
        self.signature = sig.serialize_compact().to_vec();
        Ok(())
    }

    /// Signs the transaction with the sender's post-quantum private key.
    pub fn sign_with_post_quantum_key(
        &mut self,
        post_quantum_keypair: &PostQuantumKeyPair,
        qup_crypto: &QUPCrypto,
    ) -> Result<()> {
        info!(
            "Signing transaction with post-quantum key: {} -> {}",
            self.sender, self.receiver
        );
        let message = self.calculate_hash();
        let signature = qup_crypto
            .sign_with_post_quantum_key(post_quantum_keypair, &message)
            .context("Failed to sign with post-quantum key")?;
        self.post_quantum_signature = Some(signature);
        Ok(())
    }

    /// Verifies the transaction's post-quantum signature.
    pub fn verify_post_quantum_signature(
        &self,
        post_quantum_public_key: &PostQuantumPublicKey,
        qup_crypto: &QUPCrypto,
    ) -> Result<()> {
        if let Some(signature) = &self.post_quantum_signature {
            let message = self.calculate_hash();
            qup_crypto
                .verify_post_quantum_signature(post_quantum_public_key, &message, signature)
                .context("Post-quantum signature verification failed")
        } else {
            anyhow::bail!("Post-quantum signature not found")
        }
    }

    /// Applies the transaction to the given state.
    pub fn apply_to_state(&self, state: &mut Arc<RwLock<State>>) -> Result<()> {
        let mut state_lock = state.write().unwrap();

        // Validate the transaction
        self.validate(&state_lock)?;

        // Update the sender's account
        let sender_account = state_lock
            .get_account_mut(&self.sender)
            .context(format!("Sender account not found: {}", self.sender))?;
        if sender_account.balance < self.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.sender));
        }
        if sender_account.nonce != self.nonce {
            anyhow::bail!(format!("Invalid nonce for sender: {}", self.sender));
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
    pub fn validate(&self, state: &State) -> Result<()> {
        // Check if the sender account exists
        if !state.account_exists(&self.sender) {
            anyhow::bail!(format!("Sender account not found: {}", self.sender));
        }

        // Check if the transaction amount is positive
        if self.amount <= 0.0 {
            anyhow::bail!("Transaction amount must be positive");
        }

        // Check if the sender has sufficient balance
        let sender_account = state
            .get_account(&self.sender)
            .context(format!("Sender account not found: {}", self.sender))?;
        if sender_account.balance < self.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.sender));
        }

        // Check if the nonce is valid
        if sender_account.nonce != self.nonce {
            anyhow::bail!(format!("Invalid nonce for sender: {}", self.sender));
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
        verify_proof(&self.proof.proof_hash, &proof_data)
            .context("Failed to verify zero-knowledge proof")?;

        Ok(())
    }

    /// Applies the transaction to the given QUP state.
    pub fn apply_to_qup_state(
        &self,
        qup_state: &mut Arc<RwLock<QUPState>>,
    ) -> Result<()> {
        let mut qup_state_lock = qup_state.write().unwrap();

        // Validate the transaction
        self.validate_for_qup(&qup_state_lock)?;

        // Update the sender's account in the QUP state
        let sender_account = qup_state_lock
            .get_account_mut(&self.sender)
            .context(format!("Sender account not found: {}", self.sender))?;
        if sender_account.balance < self.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.sender));
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
                .context(format!("Validator not found: {}", self.sender))?;
            validator.update_useful_work_score(solution);
        }

        Ok(())
    }

    /// Validates the transaction against the current QUP state.
    pub fn validate_for_qup(&self, qup_state: &QUPState) -> Result<()> {
        // Check if the sender account exists in the QUP state
        if !qup_state.account_exists(&self.sender) {
            anyhow::bail!(format!("Sender account not found: {}", self.sender));
        }

        // Check if the transaction amount is positive
        if self.amount <= 0.0 {
            anyhow::bail!("Transaction amount must be positive");
        }

        // Check if the sender has sufficient balance in the QUP state
        let sender_account = qup_state
            .get_account(&self.sender)
            .context(format!("Sender account not found: {}", self.sender))?;
        if sender_account.balance < self.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.sender));
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
        verify_proof(&self.proof.proof_hash, &proof_data)
            .context("Failed to verify zero-knowledge proof")?;

        // Verify the useful work solution if present
        if let Some(solution) = &self.useful_work_solution {
            qup_state
                .verify_useful_work(solution)
                .context("Failed to verify useful work solution")?;
        }

        Ok(())
    }
}

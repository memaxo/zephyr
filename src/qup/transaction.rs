use crate::chain::common::TransactionCommon;

impl TransactionCommon for QUPTransaction {
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
        qup_crypto
            .verify_signature(public_key, &self.common.signature, &self.calculate_hash())
            .context("Failed to verify transaction signature")
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
        // Check if the sender account exists
        if !state.account_exists(&self.common.sender) {
            anyhow::bail!(format!("Sender account not found: {}", self.common.sender));
        }

        // Check if the transaction amount is positive
        if self.common.amount <= 0.0 {
            anyhow::bail!("Transaction amount must be positive");
        }

        // Check if the sender has sufficient balance
        let sender_account = state
            .get_account(&self.common.sender)
            .context(format!("Sender account not found: {}", self.common.sender))?;
        if sender_account.balance < self.common.amount {
            anyhow::bail!(format!("Insufficient balance for sender: {}", self.common.sender));
        }

        // Check if the nonce is valid
        if sender_account.nonce != self.common.nonce {
            anyhow::bail!(format!("Invalid nonce for sender: {}", self.common.sender));
        }

        // Verify the transaction signature
        self.verify_signature(&self.sender_public_key()?)?;

        // Verify the post-quantum signature
        self.verify_post_quantum_signature(&self.sender_post_quantum_public_key()?)?;

        // Verify the zero-knowledge proof
        let proof_data = [
            self.common.sender.as_bytes(),
            self.common.receiver.as_bytes(),
            &self.common.amount.to_be_bytes(),
            self.sp_key.expose_secret(),
        ]
        .concat();
        verify_proof(&self.common.proof.proof_hash, &proof_data)
            .context("Failed to verify zero-knowledge proof")?;

        Ok(())
    }
}

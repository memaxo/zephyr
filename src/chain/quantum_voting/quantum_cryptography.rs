use crate::chain::quantum_voting::errors::VotingError;
use crate::chain::quantum_voting::types::{EncryptedVote, QuantumKey, Vote};
use crate::crypto::quantum::{QuantumDigitalSignature, QuantumEncryption, QuantumHash};
use log::{debug, error, info, trace};

pub struct QuantumCryptography {
    encryption: QuantumEncryption,
    digital_signature: QuantumDigitalSignature,
    hash: QuantumHash,
}

impl crate::chain::quantum_voting::traits::QuantumCryptography for QuantumCryptography {
    pub fn new() -> Self {
        QuantumCryptography {
            encryption: QuantumEncryption::new(),
            digital_signature: QuantumDigitalSignature::new(),
            hash: QuantumHash::new(),
        }
    }

    fn encrypt_vote(
        &self,
        vote: &Vote,
        quantum_key: &QuantumKey,
    ) -> Result<EncryptedVote, VotingError> {
        if self.node_config.supports_quantum_features() {
            if !self.feature_flags.enable_quantum_cryptography {
                return Err(VotingError::QuantumFeatureDisabled);
            }
            debug!("Encrypting vote");

            // Perform quantum encryption on the vote using the provided quantum key
            let encrypted_vote = self.encryption.encrypt(vote, quantum_key)?;

            info!("Vote encrypted successfully");
            Ok(encrypted_vote)
        } else {
            // Fallback to classical encryption
            let classical_cryptography = ClassicalCryptography::new();
            classical_cryptography.encrypt_vote(vote, quantum_key)
        }
    }

    fn decrypt_vote(
        &self,
        encrypted_vote: &EncryptedVote,
        quantum_key: &QuantumKey,
    ) -> Result<Vote, VotingError> {
        debug!("Decrypting vote");

        // Perform quantum decryption on the encrypted vote using the provided quantum key
        let decrypted_vote = self.encryption.decrypt(encrypted_vote, quantum_key)?;

        info!("Vote decrypted successfully");
        Ok(decrypted_vote)
    }

    fn sign_vote(
        &self,
        vote: &Vote,
        quantum_key: &QuantumKey,
    ) -> Result<QuantumDigitalSignature, VotingError> {
        debug!("Signing vote");

        // Generate a digital signature for the vote using the provided quantum key
        let signature = self.digital_signature.sign(vote, quantum_key)?;

        info!("Vote signed successfully");
        Ok(signature)
    }

    fn verify_vote_signature(
        &self,
        vote: &Vote,
        signature: &QuantumDigitalSignature,
        quantum_key: &QuantumKey,
    ) -> Result<bool, VotingError> {
        debug!("Verifying vote signature");

        // Verify the digital signature of the vote using the provided quantum key
        let is_valid = self
            .digital_signature
            .verify(vote, signature, quantum_key)?;

        info!("Vote signature verified: {}", is_valid);
        Ok(is_valid)
    }

    fn hash_vote(&self, vote: &Vote) -> Result<QuantumHash, VotingError> {
        debug!("Hashing vote");

        // Compute the quantum hash of the vote
        let vote_hash = self.hash.compute(vote)?;

        trace!("Vote hash: {:?}", vote_hash);
        Ok(vote_hash)
    }
}

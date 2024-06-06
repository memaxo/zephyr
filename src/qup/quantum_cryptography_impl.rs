use crate::qup::traits::QuantumCryptography;
use aes_gcm_siv::{
    aead::{Aead, NewAead},
    Aes256GcmSiv, Key, Nonce,
};
use blake3::Hasher as Blake3Hasher;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

pub struct QuantumCryptographyImpl;

impl QuantumCryptography for QuantumCryptographyImpl {
    fn encrypt_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<EncryptedVote, VotingError> {
        let key = Key::from_slice(&quantum_key.public_key);
        let cipher = Aes256GcmSiv::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // Use a unique nonce for each encryption
        let ciphertext = cipher.encrypt(nonce, vote.to_bytes().as_ref())
            .map_err(|_| VotingError::EncryptionFailed)?;
        Ok(EncryptedVote { ciphertext })
    }

    fn decrypt_vote(&self, encrypted_vote: &EncryptedVote, quantum_key: &QuantumKey) -> Result<Vote, VotingError> {
        let key = Key::from_slice(&quantum_key.secret_key);
        let cipher = Aes256GcmSiv::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // Use the same nonce as used during encryption
        let plaintext = cipher.decrypt(nonce, encrypted_vote.ciphertext.as_ref())
            .map_err(|_| VotingError::DecryptionFailed)?;
        Vote::from_bytes(&plaintext)
    }

    fn sign_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<QuantumDigitalSignature, VotingError> {
        let secret_key = SecretKey::from_bytes(&quantum_key.secret_key)
            .map_err(|_| VotingError::InvalidKey)?;
        let keypair = Keypair { secret: secret_key, public: PublicKey::from(&secret_key) };
        let signature = keypair.sign(&vote.to_bytes());
        Ok(QuantumDigitalSignature { signature: signature.to_bytes().to_vec() })
    }

    fn verify_vote_signature(&self, vote: &Vote, signature: &QuantumDigitalSignature, quantum_key: &QuantumKey) -> Result<bool, VotingError> {
        let public_key = PublicKey::from_bytes(&quantum_key.public_key)
            .map_err(|_| VotingError::InvalidKey)?;
        let signature = Signature::from_bytes(&signature.signature)
            .map_err(|_| VotingError::InvalidSignature)?;
        Ok(public_key.verify(&vote.to_bytes(), &signature).is_ok())
    }

    fn hash_vote(&self, vote: &Vote) -> Result<QuantumHash, VotingError> {
        let mut hasher = Blake3Hasher::new();
        hasher.update(&vote.to_bytes());
        Ok(QuantumHash { hash: hasher.finalize().as_bytes().to_vec() })
    }
}

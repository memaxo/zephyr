use crate::qup::traits::QuantumCryptography;

pub struct QuantumCryptographyImpl;

impl QuantumCryptography for QuantumCryptographyImpl {
    fn encrypt_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<EncryptedVote, VotingError> {
        let public_key = KyberPublicKey::from_bytes(&quantum_key.public_key)?;
        let ciphertext = public_key.encrypt(&vote.to_bytes())?;
        Ok(EncryptedVote { ciphertext: ciphertext.to_bytes() })
    }

    fn decrypt_vote(&self, encrypted_vote: &EncryptedVote, quantum_key: &QuantumKey) -> Result<Vote, VotingError> {
        let secret_key = KyberSecretKey::from_bytes(&quantum_key.secret_key)?;
        let plaintext = secret_key.decrypt(&Ciphertext::from_bytes(&encrypted_vote.ciphertext)?)?;
        Vote::from_bytes(&plaintext)
    }

    fn sign_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<QuantumDigitalSignature, VotingError> {
        let secret_key = SecretKey::from_bytes(&quantum_key.secret_key)?;
        let signature = secret_key.sign(&vote.to_bytes())?;
        Ok(QuantumDigitalSignature { signature: signature.to_bytes() })
    }

    fn verify_vote_signature(&self, vote: &Vote, signature: &QuantumDigitalSignature, quantum_key: &QuantumKey) -> Result<bool, VotingError> {
        let public_key = PublicKey::from_bytes(&quantum_key.public_key)?;
        let dilithium_signature = DilithiumSignature::from_bytes(&signature.signature)?;
        Ok(public_key.verify(&vote.to_bytes(), &dilithium_signature).is_ok())
    }

    fn hash_vote(&self, vote: &Vote) -> Result<QuantumHash, VotingError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&vote.to_bytes());
        Ok(QuantumHash { hash: hasher.finalize().as_bytes().to_vec() })
    }
}

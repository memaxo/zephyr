use crate::qup::crypto::crypto_trait::CryptoOperations;

pub struct ClassicalCryptographyImpl;

impl CryptoOperations for ClassicalCryptographyImpl {
    type Key = ClassicalKey;
    type EncryptedVote = EncryptedVote;
    type DigitalSignature = ClassicalDigitalSignature;
    type Hash = ClassicalHash;
    type Error = VotingError;
    fn encrypt_vote(&self, vote: &Vote, classical_key: &Self::Key) -> Result<Self::EncryptedVote, Self::Error> {
        let key = Key::from_slice(&classical_key.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&classical_key.nonce);
        let ciphertext = cipher.encrypt(nonce, vote.to_bytes().as_ref())?;
        Ok(EncryptedVote { ciphertext })
    }

    fn decrypt_vote(&self, encrypted_vote: &Self::EncryptedVote, classical_key: &Self::Key) -> Result<Vote, Self::Error> {
        let key = Key::from_slice(&classical_key.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&classical_key.nonce);
        let plaintext = cipher.decrypt(nonce, encrypted_vote.ciphertext.as_ref())?;
        Vote::from_bytes(&plaintext)
    }

    fn sign_vote(&self, vote: &Vote, classical_key: &Self::Key) -> Result<Self::DigitalSignature, Self::Error> {
        let secret_key = SecretKey::from_bytes(&classical_key.signing_key)?;
        let keypair = Keypair { secret: secret_key, public: PublicKey::from(&secret_key) };
        let signature = keypair.sign(&vote.to_bytes());
        Ok(ClassicalDigitalSignature { signature: signature.to_bytes().to_vec() })
    }

    fn verify_vote_signature(&self, vote: &Vote, signature: &Self::DigitalSignature, classical_key: &Self::Key) -> Result<bool, Self::Error> {
        let public_key = PublicKey::from_bytes(&classical_key.verification_key)?;
        let ed25519_signature = Ed25519Signature::from_bytes(&signature.signature)?;
        Ok(public_key.verify(&vote.to_bytes(), &ed25519_signature).is_ok())
    }

    fn hash_vote(&self, vote: &Vote) -> Result<Self::Hash, Self::Error> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&vote.to_bytes());
        Ok(ClassicalHash { hash: hasher.finalize().as_bytes().to_vec() })
    }
}

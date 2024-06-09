pub trait CryptoOperations {
    type Key;
    type EncryptedVote;
    type DigitalSignature;
    type Hash;
    type Error;

    fn encrypt_vote(&self, vote: &Vote, key: &Self::Key) -> Result<Self::EncryptedVote, Self::Error>;
    fn decrypt_vote(&self, encrypted_vote: &Self::EncryptedVote, key: &Self::Key) -> Result<Vote, Self::Error>;
    fn sign_vote(&self, vote: &Vote, key: &Self::Key) -> Result<Self::DigitalSignature, Self::Error>;
    fn verify_vote_signature(&self, vote: &Vote, signature: &Self::DigitalSignature, key: &Self::Key) -> Result<bool, Self::Error>;
    fn hash_vote(&self, vote: &Vote) -> Result<Self::Hash, Self::Error>;
}

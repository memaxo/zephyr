pub trait QuantumCryptography {
    fn encrypt_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<EncryptedVote, VotingError>;
    fn decrypt_vote(&self, encrypted_vote: &EncryptedVote, quantum_key: &QuantumKey) -> Result<Vote, VotingError>;
    fn sign_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<QuantumDigitalSignature, VotingError>;
    fn verify_vote_signature(&self, vote: &Vote, signature: &QuantumDigitalSignature, quantum_key: &QuantumKey) -> Result<bool, VotingError>;
    fn hash_vote(&self, vote: &Vote) -> Result<QuantumHash, VotingError>;
}

pub trait ClassicalCryptography {
    fn encrypt_vote(&self, vote: &Vote, classical_key: &ClassicalKey) -> Result<EncryptedVote, VotingError>;
    fn decrypt_vote(&self, encrypted_vote: &EncryptedVote, classical_key: &ClassicalKey) -> Result<Vote, VotingError>;
    fn sign_vote(&self, vote: &Vote, classical_key: &ClassicalKey) -> Result<ClassicalDigitalSignature, VotingError>;
    fn verify_vote_signature(&self, vote: &Vote, signature: &ClassicalDigitalSignature, classical_key: &ClassicalKey) -> Result<bool, VotingError>;
    fn hash_vote(&self, vote: &Vote) -> Result<ClassicalHash, VotingError>;
}
pub trait QuantumComputationProvider {
    fn perform_useful_work(&self, problem: &UsefulWorkProblem) -> UsefulWorkSolution;
    fn validate_useful_work(&self, problem: &UsefulWorkProblem, solution: &UsefulWorkSolution) -> bool;
}

pub trait QuantumKeyManagement {
    fn generate_key_pair(&self) -> (QuantumPublicKey, QuantumPrivateKey);
    fn sign_message(&self, message: &[u8], private_key: &QuantumPrivateKey) -> QuantumSignature;
    fn verify_signature(&self, message: &[u8], signature: &QuantumSignature, public_key: &QuantumPublicKey) -> bool;
}

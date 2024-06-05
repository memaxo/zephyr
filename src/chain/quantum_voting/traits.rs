pub trait QuantumCryptography {
    fn encrypt_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<EncryptedVote, VotingError>;
    fn decrypt_vote(&self, encrypted_vote: &EncryptedVote, quantum_key: &QuantumKey) -> Result<Vote, VotingError>;
    fn sign_vote(&self, vote: &Vote, quantum_key: &QuantumKey) -> Result<QuantumDigitalSignature, VotingError>;
    fn verify_vote_signature(&self, vote: &Vote, signature: &QuantumDigitalSignature, quantum_key: &QuantumKey) -> Result<bool, VotingError>;
    fn hash_vote(&self, vote: &Vote) -> Result<QuantumHash, VotingError>;
}

pub trait QuantumKeyDistribution {
    fn generate_quantum_keys(&self, num_candidates: usize) -> Result<Vec<QuantumKey>, VotingError>;
    fn distribute_candidate_keys(&self, candidates: &[Candidate], quantum_keys: &[QuantumKey]) -> Result<(), VotingError>;
    fn distribute_voter_keys(&self, num_voters: usize, quantum_keys: &[QuantumKey]) -> Result<Vec<QuantumKey>, VotingError>;
}

pub trait QuantumStatePreparation {
    fn prepare_vote_state(&self, vote: &Vote) -> Result<QuantumState, VotingError>;
    fn prepare_candidate_state(&self, candidate: &Candidate) -> Result<QuantumState, VotingError>;
}

pub trait QuantumStateVerification {
    fn verify_vote_state(&self, vote_state: &QuantumState) -> Result<bool, VotingError>;
    fn verify_candidate_state(&self, candidate_state: &QuantumState) -> Result<bool, VotingError>;
}

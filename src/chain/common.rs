pub trait BlockCommon {
    fn calculate_hash(&self) -> String;
    fn verify_signature(&self, qup_crypto: &QUPCrypto, qup_state: &QUPState) -> Result<(), BlockError>;
    fn validate(&self, qup_consensus: &QUPConsensus, qup_state: &QUPState) -> Result<(), BlockError>;
    fn apply(&self, state: &mut QUPState) -> Result<(), Error>;
}

pub trait TransactionCommon {
    fn encrypt_details(&mut self, key: &[u8]) -> Result<()>;
    fn decrypt_details(&self, key: &[u8]) -> Result<Self>;
    fn verify_signature(&self, public_key: &PublicKey, qup_crypto: &QUPCrypto) -> Result<()>;
    fn sign(&mut self, private_key: &SecretKey) -> Result<()>;
    fn sign_with_post_quantum_key(&mut self, post_quantum_keypair: &PostQuantumKeyPair, qup_crypto: &QUPCrypto) -> Result<()>;
    fn verify_post_quantum_signature(&self, post_quantum_public_key: &PostQuantumPublicKey, qup_crypto: &QUPCrypto) -> Result<()>;
    fn apply_to_state(&self, state: &mut Arc<RwLock<State>>) -> Result<()>;
    fn validate(&self, state: &State) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockFields {
    pub timestamp: u64,
    pub transactions: Vec<Arc<Transaction>>,
    pub previous_hash: String,
    pub hash: String,
    pub validator_signature: Option<PostQuantumSignature>,
    pub useful_work: Option<UsefulWork>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionFields {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub proof: Proof,
    pub encrypted_details: Vec<u8>,
    pub post_quantum_signature: Option<PostQuantumSignature>,
    pub useful_work_solution: Option<UsefulWorkSolution>,
    pub history_proof: Option<HistoryProof>,
}

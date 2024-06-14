use pqcrypto::hash::sha3_256;
use pqcrypto::hash::Hash;

pub struct QuantumSafeHash {
    hasher: sha3_256::Hasher,
}

impl QuantumSafeHash {
    pub fn new() -> Self {
        Self {
            hasher: sha3_256::Hasher::new(),
        }
    }

    pub fn hash(&mut self, data: &[u8]) -> Vec<u8> {
        self.hasher.update(data);
        self.hasher.finalize().to_vec()
    }
}

use pqcrypto_picnic::picnic_l1_fs::{self, PublicKey, SecretKey, sign, verify};

pub struct QuantumSafeHash {
    hasher: picnic_l1_fs::Hasher,
}

impl QuantumSafeHash {
    pub fn new() -> Self {
        QuantumSafeHash {
            hasher: picnic_l1_fs::Hasher::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

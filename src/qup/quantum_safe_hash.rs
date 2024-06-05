use blake2::{Blake2b, Blake2s};
use sha3::{Digest, Sha3_256, Sha3_512};

pub enum QuantumSafeHash {
    Sha3_256(Sha3_256),
    Sha3_512(Sha3_512),
    Blake2b(Blake2b),
    Blake2s(Blake2s),
}

impl QuantumSafeHash {
    pub fn new_sha3_256() -> Self {
        QuantumSafeHash::Sha3_256(Sha3_256::new())
    }

    pub fn new_sha3_512() -> Self {
        QuantumSafeHash::Sha3_512(Sha3_512::new())
    }

    pub fn new_blake2b() -> Self {
        QuantumSafeHash::Blake2b(Blake2b::new())
    }

    pub fn new_blake2s() -> Self {
        QuantumSafeHash::Blake2s(Blake2s::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        match self {
            QuantumSafeHash::Sha3_256(hasher) => hasher.update(data),
            QuantumSafeHash::Sha3_512(hasher) => hasher.update(data),
            QuantumSafeHash::Blake2b(hasher) => hasher.update(data),
            QuantumSafeHash::Blake2s(hasher) => hasher.update(data),
        }
    }

    pub fn finalize(self) -> Vec<u8> {
        match self {
            QuantumSafeHash::Sha3_256(hasher) => hasher.finalize().to_vec(),
            QuantumSafeHash::Sha3_512(hasher) => hasher.finalize().to_vec(),
            QuantumSafeHash::Blake2b(hasher) => hasher.finalize().as_bytes().to_vec(),
            QuantumSafeHash::Blake2s(hasher) => hasher.finalize().as_bytes().to_vec(),
        }
    }
}

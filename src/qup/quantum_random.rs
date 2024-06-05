use rand::Rng;
use rand_core::{CryptoRng, OsRng, RngCore};

pub struct QuantumRandom {
    rng: OsRng,
}

impl QuantumRandom {
    pub fn new() -> Self {
        QuantumRandom { rng: OsRng }
    }

    pub fn generate_random_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        self.rng.fill_bytes(&mut bytes);
        bytes
    }

    pub fn generate_random_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn generate_random_range(&mut self, min: u64, max: u64) -> u64 {
        self.rng.gen_range(min..=max)
    }

    // Add more random number generation methods as needed
}

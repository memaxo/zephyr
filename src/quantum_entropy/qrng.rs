use crate::quantum_entropy::config::QRNGConfig;
use quantumrandom::{RandomNumberGenerator, RandomNumberGeneratorError};

pub struct QuantumEntropy {
    qrng: RandomNumberGenerator,
}

impl QuantumEntropy {
    pub fn new(config: &QRNGConfig) -> Result<Self, QRNGError> {
        let mut qrng = RandomNumberGenerator::new();
        qrng.set_url(&config.server_address);
        qrng.set_api_key(&config.api_key);

        Ok(QuantumEntropy { qrng })
    }

    pub fn generate_random_bytes(&self, num_bytes: usize) -> Result<Vec<u8>, QRNGError> {
        self.qrng
            .generate_random_bytes(num_bytes)
            .map_err(QRNGError::RandomNumberGeneratorError)
    }

    pub fn generate_seed(&self, seed_len: usize) -> Result<Vec<u8>, QRNGError> {
        self.generate_random_bytes(seed_len)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QRNGError {
    #[error("Random Number Generator error: {0}")]
    RandomNumberGeneratorError(#[from] RandomNumberGeneratorError),
}
use crate::qup::quantum_random::QuantumRandom;
use pqcrypto_kyber::kyber1024::{keypair as kyber_keypair, PublicKey as KyberPublicKey, SecretKey as KyberSecretKey};
use pqcrypto_dilithium::dilithium5::{keypair as dilithium_keypair, PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey};
use std::collections::HashMap;

pub struct KeyManagement {
    quantum_random: QuantumRandom,
    keys: HashMap<String, KeyPair>,
}

impl KeyManagement {
    pub fn new() -> Self {
        KeyManagement {
            quantum_random: QuantumRandom::new(),
            keys: HashMap::new(),
        }
    }

    pub fn generate_authentication_key(&self) -> Vec<u8> {
        self.quantum_random.generate_random_bytes(32).await.expect("Failed to generate authentication key")
    }

    pub fn generate_keypair(&mut self, key_id: &str, key_type: KeyType) {
        let keypair = match key_type {
            KeyType::Kyber => {
                let (public_key, secret_key) = kyber_keypair();
                KeyPair::Kyber(public_key, secret_key)
            }
            KeyType::Dilithium => {
                let (public_key, secret_key) = dilithium_keypair();
                KeyPair::Dilithium(public_key, secret_key)
            }
        };
        self.keys.insert(key_id.to_string(), keypair);
    }

    pub fn get_keypair(&self, key_id: &str) -> Option<&KeyPair> {
        self.keys.get(key_id)
    }
}

pub enum KeyType {
    Kyber,
    Dilithium,
}

pub enum KeyPair {
    Kyber(KyberPublicKey, KyberSecretKey),
    Dilithium(DilithiumPublicKey, DilithiumSecretKey),
}

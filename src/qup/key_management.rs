use crate::qup::quantum_random::QuantumRandom;
use pqcrypto_kyber::kyber1024::{keypair as kyber_keypair, PublicKey as KyberPublicKey, SecretKey as KyberSecretKey};
use pqcrypto_dilithium::dilithium5::{keypair as dilithium_keypair, PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey};
use std::collections::HashMap;

pub struct KeyManagement {
    quantum_random: QuantumRandom,
    kyber_keys: HashMap<String, (KyberPublicKey, KyberSecretKey)>,
    dilithium_keys: HashMap<String, (DilithiumPublicKey, DilithiumSecretKey)>,
}

impl KeyManagement {
    pub fn new() -> Self {
        KeyManagement {
            quantum_random: QuantumRandom::new(),
            kyber_keys: HashMap::new(),
            dilithium_keys: HashMap::new(),
        }
    }

    pub fn generate_authentication_key(&self) -> Vec<u8> {
        self.quantum_random.generate_random_bytes(32).await.expect("Failed to generate authentication key")
    }

    pub fn generate_kyber_keys(&mut self, key_id: &str) {
        let (public_key, secret_key) = kyber_keypair();
        self.kyber_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn generate_dilithium_keys(&mut self, key_id: &str) {
        let (public_key, secret_key) = dilithium_keypair();
        self.dilithium_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn get_kyber_keys(&self, key_id: &str) -> Option<&(KyberPublicKey, KyberSecretKey)> {
        self.kyber_keys.get(key_id)
    }

    pub fn get_dilithium_keys(&self, key_id: &str) -> Option<&(DilithiumPublicKey, DilithiumSecretKey)> {
        self.dilithium_keys.get(key_id)
    }
}

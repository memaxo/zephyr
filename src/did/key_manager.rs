use rand::rngs::OsRng;
use rand::RngCore;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::did::did::DID;

pub struct KeyManager {
    storage: Arc<Mutex<HashMap<String, KeyPair>>>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn generate_keypair() -> (PublicKey, PrivateKey) {
        let mut rng = OsRng;
        let mut private_key = [0u8; 32];
        rng.fill_bytes(&mut private_key);
        let public_key = private_key; // Placeholder for actual public key generation
        (PublicKey(public_key.to_vec()), PrivateKey(private_key.to_vec()))
    }

    pub fn store_keypair(&self, did: &DID, keypair: KeyPair) {
        let mut storage = self.storage.lock().unwrap();
        storage.insert(did.to_string(), keypair);
    }

    pub fn retrieve_keypair(&self, did: &DID) -> Option<KeyPair> {
        let storage = self.storage.lock().unwrap();
        storage.get(&did.to_string()).cloned()
    }
}

#[derive(Clone)]
pub struct PublicKey(Vec<u8>);

#[derive(Clone)]
pub struct PrivateKey(Vec<u8>);

#[derive(Clone)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl KeyPair {
    pub fn new(public_key: PublicKey, private_key: PrivateKey) -> Self {
        Self { public_key, private_key }
    }
}

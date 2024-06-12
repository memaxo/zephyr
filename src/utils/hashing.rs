use pqcrypto_picnic::picnic_l1_fs::Hasher;

pub fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
use sha2::{Digest, Sha256};

pub fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

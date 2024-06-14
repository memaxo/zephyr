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
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn consistent_hash(transaction_hash: u64, num_shards: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    transaction_hash.hash(&mut hasher);
    hasher.finish() % num_shards
}

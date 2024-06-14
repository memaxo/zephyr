use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use sled::{Db, IVec};

#[derive(Serialize, Deserialize, Clone)]
pub struct Storage {
    data: HashMap<String, String>,
    root_hash: String,
    db: Db,
}

impl Storage {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).expect("Failed to open database");
        let data = HashMap::new();
        let root_hash = Self::calculate_root_hash(&data);
        Storage { data, root_hash, db }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());
        self.update_root_hash();
        self.db.insert(key, IVec::from(value.as_bytes())).expect("Failed to write to database");
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.update_root_hash();
        self.db.remove(key).expect("Failed to delete from database");
    }

    pub fn get_root_hash(&self) -> &str {
        &self.root_hash
    }

    fn update_root_hash(&mut self) {
        self.root_hash = Self::calculate_root_hash(&self.data);
    }

    fn calculate_root_hash(data: &HashMap<String, String>) -> String {
        let mut hasher = Sha256::new();
        for (key, value) in data {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    pub fn prune(&mut self) {
        // Placeholder for state trie pruning logic
        // Implement actual pruning logic to remove old or unused state entries
    }

    pub fn apply_storage_rent(&mut self) {
        // Placeholder for storage rent logic
        // Implement actual logic to charge rent for storage usage
    }
}

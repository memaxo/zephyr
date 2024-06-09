use crate::state::account::Account;
use crate::state::state_storage::StateStorage;
use crate::chain::state::merkle_trie::MerkleTrie;
use crate::chain::state::ChainState;
use bincode::serialize;
use lru_cache::LruCache;
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct StateDB {
    db: DB,
    account_trie: MerkleTrie,
    account_cache: LruCache<String, Account>,
}

impl StateDB {
    pub fn new<P: AsRef<Path>>(path: P, chain_state: ChainState) -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, path).expect("Failed to open database");
        let account_cache = LruCache::new(1000); // Adjust the cache size as needed
        StateDB {
            db,
            account_trie: MerkleTrie::new(),
            chain_state,
            account_cache,
        }
    }

    pub fn get_account(&mut self, address: &str) -> Option<Account> {
        if let Some(account) = self.account_cache.get(address) {
            return Some(account.clone());
        }


        let account_data = self.db.get(address.as_bytes()).ok()??;
        let account = deserialize_account(&account_data);
        if let Some(account) = account.clone() {
            self.account_cache.put(address.to_string(), account);
        }
        account
    }

    pub fn set_account(&mut self, account: &Account) {
        let account_data = serialize_account(account);
        self.db
            .put(account.address.as_bytes(), &account_data)
            .expect("Failed to set account");
        self.account_cache
            .put(account.address.clone(), account.clone());
        self.update_account_trie(account);
    }

    pub fn remove_account(&mut self, address: &str) {
        self.db
            .delete(address.as_bytes())
            .expect("Failed to remove account");
        self.account_cache.pop(address);
        self.account_trie
            .remove(address.as_bytes())
            .expect("Failed to remove account from trie");
    }

    pub fn account_exists(&self, address: &str) -> bool {
        if self.account_cache.contains(address) {
            return true;
        }
        self.db.get(address.as_bytes()).ok().is_some()
    }

    pub fn get_state_root(&self) -> Vec<u8> {
        self.account_trie.root_hash().unwrap_or_default()
    }

    pub fn generate_state_proof(&self, address: &str) -> Option<Vec<Vec<u8>>> {
        self.account_trie.generate_proof(address.as_bytes())
    }

    pub fn verify_state_proof(&self, address: &str, account: &Account, proof: &[Vec<u8>]) -> bool {
        let state_root = self.get_state_root();
        let account_data = serialize_account(account);
        self.account_trie
            .verify_proof(&state_root, address.as_bytes(), &account_data, proof)
            .unwrap_or(false)
    }

    fn update_account_trie(&mut self, account: &Account) {
        let account_data = serialize_account(account);
        self.account_trie
            .insert(account.address.as_bytes(), &account_data);
    }
}

fn serialize_account(account: &Account) -> Vec<u8> {
    bincode::serialize(account).expect("Failed to serialize account")
}

fn deserialize_account(data: &[u8]) -> Option<Account> {
    bincode::deserialize(data).ok()
}
impl StateStorage for StateDB {
    fn get_account(&self, address: &str) -> Option<Account> {
        // TODO: Implement
        unimplemented!()
    }

    fn set_account(&mut self, account: &Account) {
        // TODO: Implement
        unimplemented!()
    }

    fn remove_account(&mut self, address: &str) {
        // TODO: Implement
        unimplemented!()
    }

    fn account_exists(&self, address: &str) -> bool {
        // TODO: Implement
        unimplemented!()
    }
}

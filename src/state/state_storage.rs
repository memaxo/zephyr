use crate::state::account::Account;

pub trait StateStorage {
    fn get_account(&self, address: &str) -> Option<Account>;
    fn set_account(&mut self, account: &Account);
    fn remove_account(&mut self, address: &str);
    fn account_exists(&self, address: &str) -> bool;
    // TODO: Add other state storage-related methods
}

pub mod block_storage;
pub mod state_storage;
pub mod transaction_storage;
pub mod contract_storage;

use crate::chain::state::ChainState;
use crate::chain::block::Block;
use crate::chain::transaction::Transaction;
use crate::storage::state_storage::StateStorageError;
use crate::storage::block_storage::BlockStorageError;
use crate::storage::transaction_storage::TransactionStorageError;

pub trait Storage {
    fn save_block(&self, block: &Block) -> Result<(), BlockStorageError>;
    fn load_block(&self, block_id: &str) -> Result<Block, BlockStorageError>;

    fn save_transaction(&self, transaction: &Transaction) -> Result<(), TransactionStorageError>;
    fn load_transaction(&self, transaction_id: &str) -> Result<Transaction, TransactionStorageError>;

    fn save_state(&self, state_id: &str, state: &ChainState) -> Result<(), StateStorageError>;
    fn load_state(&self, state_id: &str) -> Result<ChainState, StateStorageError>;
}

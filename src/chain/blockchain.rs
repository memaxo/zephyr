use chrono::{DateTime, Duration, Utc};
use log::{debug, error, info, trace, warn};
use parking_lot::RwLock;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::fs;

use crate::chain::block::Block;
use crate::chain::state::ChainState;
use crate::chain::state_transition::StateTransition;
use crate::chain::storage::BlockchainStorage;
use crate::chain::transaction::Transaction;
use crate::qup::config::QUPConfig;
use crate::qup::consensus::QUPConsensus;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use crate::secure_storage::SecureStorage;
use crate::zkp_crate;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Blockchain is empty")]
    EmptyBlockchain,
    #[error("Invalid block hash at index {0}. Expected: {1}, Found: {2}")]
    InvalidBlockHash(usize, String, String),
    #[error("Invalid previous hash at index {0}. Expected: {1}, Found: {2}")]
    InvalidPreviousHash(usize, String, String),
    #[error("Double-spending detected in block at index {0}")]
    DoubleSpending(usize),
    #[error("Zero-knowledge proof verification failed for one or more transactions in block at index {0}")]
    ZKPVerificationFailed(usize),
    #[error("Blockchain storage error: {0}")]
    StorageError(#[from] BlockchainStorageError),
    #[error("Secure storage error: {0}")]
    SecureStorageError(#[from] SecureStorageError),
    #[error("State update error: {0}")]
    StateUpdateError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct Blockchain {
    chain: Arc<RwLock<Vec<Arc<Block>>>>,
    storage: Arc<BlockchainStorage>,
    secure_storage: Arc<SecureStorage>,
    state: Arc<RwLock<ChainState>>,
    state_mutex: Arc<Mutex<()>>,
    state_transition: Arc<StateTransition>,
    qup_config: Arc<QUPConfig>,
    qup_consensus: Arc<QUPConsensus>,
    qup_state: Arc<QUPState>,
}

impl Blockchain {
    pub fn new(
        storage: Arc<BlockchainStorage>,
        secure_storage: Arc<SecureStorage>,
        qup_config: Arc<QUPConfig>,
        qup_consensus: Arc<QUPConsensus>,
        qup_state: Arc<QUPState>,
    ) -> Self {
        let chain = Arc::new(RwLock::new(Vec::new()));
        let state = Arc::new(RwLock::new(ChainState::new()));
        let state_manager = Arc::new(StateManager::new());
        let state_transition = Arc::new(StateTransition::new(state_manager));

        Blockchain {
            chain,
            storage,
            secure_storage,
            state,
            state_mutex: Arc::new(Mutex::new(())),
            state_transition,
            qup_config,
            qup_consensus,
            qup_state,
        }
    }

    pub async fn commit(&self) -> Result<(), BlockchainError> {
        let _state_lock = self.state_mutex.lock().unwrap();

        // Commit the state changes to the underlying storage
        let state = self.state.read();
        let state_data = serde_json::to_vec(&*state)?;
        fs::write("state.json", state_data).await.map_err(BlockchainError::IoError)?;

        // Update the state trie and persist it to storage
        let state_trie_data = state.trie.serialize();
        fs::write("state_trie.dat", state_trie_data).await.map_err(BlockchainError::IoError)?;

        debug!("State committed successfully");
        Ok(())
    }

    pub async fn revert_block(&self, block: &Block) -> Result<(), BlockchainError> {
        let _state_lock = self.state_mutex.lock().unwrap();

        // Revert the state changes made by the block
        self.state_transition.revert_block(block)?;

        // Remove the block from the chain
        let mut chain = self.chain.write();
        if let Some(index) = chain.iter().position(|b| b.hash == block.hash) {
            chain.remove(index);
        }

        // Persist the updated state and chain data
        self.commit().await?;

        debug!("Block reverted successfully");
        Ok(())
    }

    pub async fn add_block(&self, block: Block) -> Result<(), BlockchainError> {
        let _state_lock = self.state_mutex.lock().unwrap();

        let mut chain = self.chain.write();
        chain.push(Arc::new(block.clone()));

        self.storage.save_block(&block).await?;
        self.state_transition.apply_block(&block)?;

        debug!("Block added to the blockchain");
        Ok(())
    }

    pub async fn validate_chain(&self) -> Result<(), BlockchainError> {
        let chain = self.chain.read();
        let mut spent_transactions: HashSet<String> = HashSet::new();

        if chain.is_empty() {
            return Err(BlockchainError::EmptyBlockchain);
        }

        chain.par_iter().enumerate().try_for_each(|(i, block)| {
            if i == 0 {
                // Validate the genesis block's previous hash
                if block.previous_hash != self.qup_config.genesis_block_prev_hash() {
                    return Err(BlockchainError::InvalidPreviousHash(
                        i,
                        self.qup_config.genesis_block_prev_hash().to_string(),
                        block.previous_hash.clone(),
                    ));
                }
            } else {
                let previous_block = &chain[i - 1];
                if block.previous_hash != previous_block.hash {
                    return Err(BlockchainError::InvalidPreviousHash(
                        i,
                        previous_block.hash.clone(),
                        block.previous_hash.clone(),
                    ));
                }
            }

            let calculated_hash = block.calculate_hash();
            if block.hash != calculated_hash {
                return Err(BlockchainError::InvalidBlockHash(
                    i,
                    calculated_hash,
                    block.hash.clone(),
                ));
            }

            let transaction_hashes: HashSet<String> = block
                .transactions
                .par_iter()
                .map(|tx| tx.calculate_hash())
                .collect();

            if transaction_hashes
                .par_iter()
                .any(|hash| spent_transactions.contains(hash))
            {
                return Err(BlockchainError::DoubleSpending(i));
            }

            spent_transactions.extend(transaction_hashes);

            // Additional transaction validation checks can be added here
            // ...

            let verification_results = block
                .transactions
                .par_iter()
                .map(|transaction| transaction.verify_signature())
                .collect::<Vec<_>>();

            if !verification_results.into_iter().all(|result| result) {
                return Err(BlockchainError::ZKPVerificationFailed(i));
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn get_current_block_id(&self) -> Option<usize> {
        let chain = self.chain.read();
        if chain.is_empty() {
            None
        } else {
            Some(chain.len() - 1)
        }
    }

    pub async fn register_validator(
        &self,
        validator: &QUPValidator,
    ) -> Result<(), BlockchainError> {
        let public_key = validator.public_key().to_bytes();
        let validator_id = validator.address().to_string();
        self.secure_storage
            .save_validator_key(&validator_id, &public_key)
            .await?;
        self.qup_state.register_validator(validator).await?;
        debug!("Validator registered: {}", validator_id);
        Ok(())
    }

    pub async fn get_block_by_height(&self, height: u64) -> Option<Block> {
        self.qup_state.get_block_by_height(height).await
    }

    pub async fn get_latest_block(&self) -> Option<Block> {
        self.qup_state.get_latest_block().await
    }

    pub fn get_chain_length(&self) -> u64 {
        let chain = self.chain.read();
        chain.len() as u64
    }
}

Here's the updated blockchain.rs file with the suggested improvements:
rustCopy codeuse chrono::{DateTime, Duration, Utc};
use log::{debug, info};
use parking_lot::RwLock;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use thiserror::Error;

use crate::chain::block::Block;
use crate::chain::state::ChainState;
use crate::chain::state_transition::StateTransition;
use crate::chain::storage::BlockchainStorage;
use crate::chain::transaction::Transaction;
use crate::qup::config::QUPConfig;
use crate::qup::consensus::QUPConsensus;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use crate::secure_storage::SecureStorage;
use crate::zkp_crate;

#[derive(Error, Debug)]
pub enum BlockchainError {
    // ...
}

pub struct Blockchain {
    chain: Arc<RwLock<Vec<Arc<Block>>>>,
    storage: Arc<BlockchainStorage>,
    secure_storage: Arc<SecureStorage>,
    state: Arc<RwLock<ChainState>>,
    state_mutex: Arc<Mutex<()>>,
    state_transition: Arc<StateTransition>,
    qup_config: Arc<QUPConfig>,
    qup_consensus: Arc<QUPConsensus>,
    qup_state: Arc<QUPState>,
}

impl Blockchain {
    pub fn new(
        storage: Arc<BlockchainStorage>,
        secure_storage: Arc<SecureStorage>,
        qup_config: Arc<QUPConfig>,
        qup_consensus: Arc<QUPConsensus>,
        qup_state: Arc<QUPState>,
    ) -> Self {
        // ...
        Blockchain {
            // ...
            qup_state,
        }
    }

    // ...

    pub async fn validate_chain(&self) -> Result<(), BlockchainError> {
        let chain = self.chain.read();
        let mut spent_transactions: HashSet<String> = HashSet::new();

        if chain.is_empty() {
            return Err(BlockchainError::EmptyBlockchain);
        }

        chain.par_iter().enumerate().try_for_each(|(i, block)| {
            if i > 0 {
                let previous_block = &chain[i - 1];
                if block.previous_hash != previous_block.hash {
                    return Err(BlockchainError::InvalidPreviousHash(
                        i,
                        previous_block.hash.clone(),
                        block.previous_hash.clone(),
                    ));
                }
            }

            let calculated_hash = block.calculate_hash();
            if block.hash != calculated_hash {
                return Err(BlockchainError::InvalidBlockHash(
                    i,
                    calculated_hash,
                    block.hash.clone(),
                ));
            }

            let transaction_hashes: HashSet<String> = block
                .transactions
                .par_iter()
                .map(|tx| tx.calculate_hash())
                .collect();

            if transaction_hashes
                .par_iter()
                .any(|hash| spent_transactions.contains(hash))
            {
                return Err(BlockchainError::DoubleSpending(i));
            }

            spent_transactions.extend(transaction_hashes);

            let verification_results = block
                .transactions
                .par_iter()
                .map(|transaction| transaction.verify_signature())
                .collect::<Vec<_>>();

            if !verification_results.into_iter().all(|result| result) {
                return Err(BlockchainError::ZKPVerificationFailed(i));
            }

            Ok(())
        })?;

        Ok(())
    }

    pub async fn register_validator(&self, validator: &QUPValidator) -> Result<(), BlockchainError> {
        let public_key = validator.public_key().to_bytes();
        let validator_id = validator.address().to_string();
        self.secure_storage
            .save_validator_key(&validator_id, &public_key)
            .await?;
        self.qup_state.register_validator(validator).await?;
        debug!("Validator registered: {}", validator_id);
        Ok(())
    }

    pub async fn get_block_by_height(&self, height: u64) -> Option<Block> {
        self.qup_state.get_block_by_height(height).await
    }

    pub async fn get_latest_block(&self) -> Option<Block> {
        self.qup_state.get_latest_block().await
    }
}

// Separate module for blockchain-related functionality
pub(crate) mod blockchain_utils {
    use super::*;

    pub async fn prune_blockchain(blockchain: &Blockchain, max_blocks: usize) -> Result<(), BlockchainError> {
        let mut chain = blockchain.chain.write();

        if chain.len() > max_blocks {
            let prune_count = chain.len() - max_blocks;
            chain.drain(0..prune_count);
            debug!("Pruned {} blocks from the blockchain", prune_count);

            // Persist the updated chain data
            blockchain.commit().await?;
        }

        Ok(())
    }

    // Add more utility functions as needed
}
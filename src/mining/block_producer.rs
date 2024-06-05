use crate::chain::block::Block;
use crate::chain::blockchain::Blockchain;
use crate::chain::transaction::Transaction;
use crate::mining::mining_config::MiningConfig;
use log::debug;
use std::sync::{Arc, Mutex};

pub struct BlockProducer {
    blockchain: Arc<Mutex<Blockchain>>,
    config: MiningConfig,
}

impl BlockProducer {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>, config: MiningConfig) -> Self {
        BlockProducer { blockchain, config }
    }

    pub fn create_block_template(&self, transactions: Vec<Transaction>) -> Block {
        let mut blockchain = self.blockchain.lock().unwrap();
        let previous_block_hash = blockchain.get_latest_block_hash();
        let block_number = blockchain.get_latest_block_number() + 1;
        let timestamp = chrono::Utc::now().timestamp();

        let block = Block {
            block_number,
            previous_block_hash,
            timestamp,
            transactions,
            nonce: 0,
            block_reward: self.config.block_reward(),
            difficulty: self.calculate_difficulty(),
            // Add other block fields as needed
        };

        debug!("Created block template: {:?}", block);
        block
    }

    fn calculate_difficulty(&self) -> u64 {
        let mut blockchain = self.blockchain.lock().unwrap();
        let latest_block = blockchain.get_latest_block();

        // Implement difficulty adjustment algorithm based on the mining config
        // and the current state of the blockchain
        // Example:
        let time_since_last_block = chrono::Utc::now().timestamp() - latest_block.timestamp;
        let target_block_time = self.config.target_block_time();

        if time_since_last_block < target_block_time {
            latest_block.difficulty + 1
        } else if time_since_last_block > target_block_time {
            latest_block.difficulty - 1
        } else {
            latest_block.difficulty
        }
    }
}
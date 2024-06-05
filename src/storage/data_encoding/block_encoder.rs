use crate::chain::block::Block;
use serde_json;

/// Encodes a block into a JSON string.
pub fn encode_block(block: &Block) -> Result<String, serde_json::Error> {
    serde_json::to_string(block)
}

/// Decodes a JSON string into a block.
pub fn decode_block(data: &str) -> Result<Block, serde_json::Error> {
    serde_json::from_str(data)
}
use crate::chain::block::Block;
use crate::hdcmodels::encoding::{encode_transactional_data, encode_smart_contract};
use serde_json;
use std::error::Error;

/// Struct for encoding and decoding blockchain blocks.
pub struct BlockEncoder;

impl BlockEncoder {
    /// Encodes a block into a JSON string.
    pub fn encode_block(block: &Block) -> Result<String, serde_json::Error> {
        serde_json::to_string(block)
    }

    /// Decodes a JSON string into a block.
    pub fn decode_block(data: &str) -> Result<Block, serde_json::Error> {
        serde_json::from_str(data)
    }

    /// Encodes block data into high-dimensional vectors.
    pub fn encode_block_data(block: &Block, dimension: usize) -> Result<Vec<f64>, Box<dyn Error>> {
        let mut encoded_data = Vec::new();

        // Encode transactions
        let transactions = block.transactions.iter().map(|tx| tx.as_ref().clone()).collect::<Vec<_>>();
        let encoded_transactions = encode_transactional_data(&transactions, dimension);
        encoded_data.extend(encoded_transactions);

        // Encode smart contracts
        for sc in &block.smart_contracts {
            let encoded_contract = encode_smart_contract(&sc.code, dimension, 3); // Example n-gram size
            encoded_data.extend(encoded_contract);
        }

        Ok(encoded_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::transaction::Transaction;
    use crate::chain::block::UsefulWork;
    use std::sync::Arc;

    #[test]
    fn test_encode_decode_block() {
        let block = Block {
            timestamp: 1234567890,
            transactions: vec![Arc::new(Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 10.0,
                nonce: 1,
                signature: vec![],
                proof: Proof { proof_hash: "hash".to_string() },
                encrypted_details: vec![],
                post_quantum_signature: None,
                useful_work_solution: None,
            })],
            previous_hash: "prev_hash".to_string(),
            hash: "hash".to_string(),
            difficulty: 1,
            nonce: 0,
            merkle_root: "merkle_root".to_string(),
            smart_contracts: vec![],
            hv_dimension: 128,
            state_root: "state_root".to_string(),
            validator_signature: None,
            useful_work: None,
        };

        let encoded = BlockEncoder::encode_block(&block).unwrap();
        let decoded: Block = BlockEncoder::decode_block(&encoded).unwrap();

        assert_eq!(block.timestamp, decoded.timestamp);
        assert_eq!(block.transactions.len(), decoded.transactions.len());
        assert_eq!(block.previous_hash, decoded.previous_hash);
        assert_eq!(block.hash, decoded.hash);
        assert_eq!(block.difficulty, decoded.difficulty);
        assert_eq!(block.nonce, decoded.nonce);
        assert_eq!(block.merkle_root, decoded.merkle_root);
        assert_eq!(block.smart_contracts.len(), decoded.smart_contracts.len());
        assert_eq!(block.hv_dimension, decoded.hv_dimension);
        assert_eq!(block.state_root, decoded.state_root);
    }

    #[test]
    fn test_encode_block_data() {
        let block = Block {
            timestamp: 1234567890,
            transactions: vec![Arc::new(Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 10.0,
                nonce: 1,
                signature: vec![],
                proof: Proof { proof_hash: "hash".to_string() },
                encrypted_details: vec![],
                post_quantum_signature: None,
                useful_work_solution: None,
            })],
            previous_hash: "prev_hash".to_string(),
            hash: "hash".to_string(),
            difficulty: 1,
            nonce: 0,
            merkle_root: "merkle_root".to_string(),
            smart_contracts: vec![],
            hv_dimension: 128,
            state_root: "state_root".to_string(),
            validator_signature: None,
            useful_work: None,
        };

        let encoded_data = BlockEncoder::encode_block_data(&block, 128).unwrap();
        assert!(!encoded_data.is_empty());
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;
    use crate::chain::{Block, Transaction};
    use std::collections::VecDeque;

    #[test]
    fn test_calculate_hash() {
        let block = Block {
            timestamp: "2023-01-01 00:00:00".to_string(),
            transactions: Vec::new(),
            previous_hash: "000000".to_string(),
            hash: "".to_string(),
            difficulty: 0,
            nonce: 0,
            merkle_root: "".to_string(),
        };
        let hash = Block::calculate_hash(&block);
        assert!(!hash.is_empty(), "The hash should not be empty");
    }

    #[test]
    fn test_mine_block() {
        let mut block = Block {
            timestamp: "2023-01-01 00:00:00".to_string(),
            transactions: Vec::new(),
            previous_hash: "000000".to_string(),
            hash: "".to_string(),
            difficulty: 1, // Low difficulty for testing
            nonce: 0,
            merkle_root: "".to_string(),
        };
        block.mine(&[]);
        assert!(block.hash.starts_with("0"), "The mined block hash should start with '0'");
    }

    #[test]
    fn test_validate_block() {
        let mut block = Block {
            timestamp: "2023-01-01 00:00:00".to_string(),
            transactions: Vec::new(),
            previous_hash: "000000".to_string(),
            hash: "".to_string(),
            difficulty: 1,
            nonce: 0,
            merkle_root: "".to_string(),
        };
        block.mine(&[]);
        let validation = block.validate(None);
        assert!(validation.is_ok(), "The block should be valid after mining");
    }

    #[test]
    fn test_calculate_merkle_root() {
        let transactions = vec![
            Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 10.0,
                signature: vec![],
                sp_key: vec![],
                encrypted_details: vec![],
                proof: Proof { proof_hash: "dummy_hash".to_string(), secret_key: "dummy_key".to_string() },
                transaction_id: "tx1".to_string(),
            },
            Transaction {
                sender: "Charlie".to_string(),
                receiver: "Dave".to_string(),
                amount: 20.0,
                signature: vec![],
                sp_key: vec![],
                encrypted_details: vec![],
                proof: Proof { proof_hash: "dummy_hash".to_string(), secret_key: "dummy_key".to_string() },
                transaction_id: "tx2".to_string(),
            },
        ];
        let merkle_root = Block::calculate_merkle_root(&transactions);
        assert!(!merkle_root.is_empty(), "The Merkle root should not be empty");
    }
}
    #[test]
    fn test_merkle_root_reflects_transaction_changes() {
        let transaction1 = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 10.0,
            signature: vec![],
            sp_key: vec![],
            encrypted_details: vec![],
            proof: Proof { proof_hash: "dummy_hash".to_string(), secret_key: "dummy_key".to_string() },
            transaction_id: "tx1".to_string(),
        };
        let transaction2 = Transaction {
            sender: "Charlie".to_string(),
            receiver: "Dave".to_string(),
            amount: 20.0,
            signature: vec![],
            sp_key: vec![],
            encrypted_details: vec![],
            proof: Proof { proof_hash: "dummy_hash".to_string(), secret_key: "dummy_key".to_string() },
            transaction_id: "tx2".to_string(),
        };
        let transactions = vec![transaction1.clone(), transaction2.clone()];
        let merkle_root_initial = Block::calculate_merkle_root(&transactions);

        // Add another transaction and recalculate Merkle root
        let transaction3 = Transaction {
            sender: "Eve".to_string(),
            receiver: "Frank".to_string(),
            amount: 30.0,
            signature: vec![],
            sp_key: vec![],
            encrypted_details: vec![],
            proof: Proof { proof_hash: "dummy_hash".to_string(), secret_key: "dummy_key".to_string() },
            transaction_id: "tx3".to_string(),
        };
        let transactions_updated = vec![transaction1, transaction2, transaction3];
        let merkle_root_updated = Block::calculate_merkle_root(&transactions_updated);

        assert_ne!(merkle_root_initial, merkle_root_updated, "Merkle root should change with transaction set updates");
    }
    #[test]
    fn test_difficulty_adjustment() {
        let mut blockchain = Blockchain::new();
        let initial_difficulty = blockchain.chain.last().unwrap().difficulty;

        // Simulate fast block creation to increase difficulty
        for _ in 0..12 {
            let mut block = Block::new_from_previous(&blockchain.chain.last().unwrap());
            block.timestamp = (block.timestamp.parse::<u64>().unwrap() + 50000).to_string(); // Simulate fast mining
            block.mine(&blockchain.chain);
            blockchain.chain.push(block);
        }
        let increased_difficulty = blockchain.chain.last().unwrap().difficulty;
        assert!(increased_difficulty > initial_difficulty, "Difficulty should increase after fast block creation");

        // Simulate slow block creation to decrease difficulty
        for _ in 0..12 {
            let mut block = Block::new_from_previous(&blockchain.chain.last().unwrap());
            block.timestamp = (block.timestamp.parse::<u64>().unwrap() + 120000).to_string(); // Simulate slow mining
            block.mine(&blockchain.chain);
            blockchain.chain.push(block);
        }
        let decreased_difficulty = blockchain.chain.last().unwrap().difficulty;
        assert!(decreased_difficulty < increased_difficulty, "Difficulty should decrease after slow block creation");
    }

use crate::chain::transaction::Transaction;
use crate::hdcmodels::encoding::encode_transactional_data;
use serde_json;
use std::error::Error;

/// Struct for encoding and decoding transactions.
pub struct TransactionEncoder;

impl TransactionEncoder {
    /// Encodes a transaction into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `transaction` - A reference to the transaction to be encoded.
    ///
    /// # Returns
    ///
    /// * `Result<String, serde_json::Error>` - A JSON string representation of the transaction or an error.
    pub fn encode_transaction(transaction: &Transaction) -> Result<String, serde_json::Error> {
        serde_json::to_string(transaction)
    }

    /// Decodes a JSON string into a transaction.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON string representation of the transaction.
    ///
    /// # Returns
    ///
    /// * `Result<Transaction, serde_json::Error>` - A transaction object or an error.
    pub fn decode_transaction(data: &str) -> Result<Transaction, serde_json::Error> {
        serde_json::from_str(data)
    }

    /// Encodes transaction data into high-dimensional vectors.
    ///
    /// # Arguments
    ///
    /// * `transactions` - A slice of transactions to be encoded.
    /// * `dimension` - The dimension of the high-dimensional vectors.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<f64>, Box<dyn Error>>` - A vector of high-dimensional vectors or an error.
    pub fn encode_transaction_data(transactions: &[Transaction], dimension: usize) -> Result<Vec<f64>, Box<dyn Error>> {
        let encoded_data = encode_transactional_data(transactions, dimension);
        Ok(encoded_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::transaction::Proof;

    #[test]
    fn test_encode_decode_transaction() {
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 10.0,
            nonce: 1,
            signature: vec![],
            proof: Proof { proof_hash: "hash".to_string() },
            encrypted_details: vec![],
            post_quantum_signature: None,
            useful_work_solution: None,
        };

        let encoded = TransactionEncoder::encode_transaction(&transaction).unwrap();
        let decoded: Transaction = TransactionEncoder::decode_transaction(&encoded).unwrap();

        assert_eq!(transaction.sender, decoded.sender);
        assert_eq!(transaction.receiver, decoded.receiver);
        assert_eq!(transaction.amount, decoded.amount);
        assert_eq!(transaction.nonce, decoded.nonce);
        assert_eq!(transaction.proof.proof_hash, decoded.proof.proof_hash);
    }

    #[test]
    fn test_encode_transaction_data() {
        let transactions = vec![
            Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 10.0,
                nonce: 1,
                signature: vec![],
                proof: Proof { proof_hash: "hash".to_string() },
                encrypted_details: vec![],
                post_quantum_signature: None,
                useful_work_solution: None,
            },
            Transaction {
                sender: "Charlie".to_string(),
                receiver: "Dave".to_string(),
                amount: 20.0,
                nonce: 2,
                signature: vec![],
                proof: Proof { proof_hash: "hash2".to_string() },
                encrypted_details: vec![],
                post_quantum_signature: None,
                useful_work_solution: None,
            },
        ];

        let encoded_data = TransactionEncoder::encode_transaction_data(&transactions, 128).unwrap();
        assert!(!encoded_data.is_empty());
    }
}

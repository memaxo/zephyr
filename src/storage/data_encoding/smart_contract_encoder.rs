use crate::hdcmodels::encoding::{encode_rust_code, encode_smart_contract};
use serde_json;
use std::error::Error;

/// Struct for encoding and decoding smart contract code and data.
pub struct SmartContractEncoder;

impl SmartContractEncoder {
    /// Encodes smart contract code into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `contract_code` - A reference to the smart contract code to be encoded.
    ///
    /// # Returns
    ///
    /// * `Result<String, serde_json::Error>` - A JSON string representation of the smart contract code or an error.
    pub fn encode_contract_code(contract_code: &str) -> Result<String, serde_json::Error> {
        serde_json::to_string(contract_code)
    }

    /// Decodes a JSON string into smart contract code.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON string representation of the smart contract code.
    ///
    /// # Returns
    ///
    /// * `Result<String, serde_json::Error>` - A smart contract code string or an error.
    pub fn decode_contract_code(data: &str) -> Result<String, serde_json::Error> {
        serde_json::from_str(data)
    }

    /// Encodes smart contract code into high-dimensional vectors.
    ///
    /// # Arguments
    ///
    /// * `contract_code` - A reference to the smart contract code to be encoded.
    /// * `dimension` - The dimension of the high-dimensional vectors.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<f64>, Box<dyn Error>>` - A vector of high-dimensional vectors or an error.
    pub fn encode_contract_code_data(contract_code: &str, dimension: usize) -> Result<Vec<f64>, Box<dyn Error>> {
        let encoded_data = encode_rust_code(contract_code, dimension);
        Ok(encoded_data)
    }

    /// Encodes smart contract data into high-dimensional vectors.
    ///
    /// # Arguments
    ///
    /// * `contract_data` - A reference to the smart contract data to be encoded.
    /// * `dimension` - The dimension of the high-dimensional vectors.
    /// * `n` - The n-gram size for encoding.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<f64>, Box<dyn Error>>` - A vector of high-dimensional vectors or an error.
    pub fn encode_contract_data(contract_data: &str, dimension: usize, n: usize) -> Result<Vec<f64>, Box<dyn Error>> {
        let encoded_data = encode_smart_contract(contract_data, dimension, n);
        Ok(encoded_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_contract_code() {
        let contract_code = r#"
        fn main() {
            println!("Hello, blockchain!");
        }
        "#;

        let encoded = SmartContractEncoder::encode_contract_code(contract_code).unwrap();
        let decoded: String = SmartContractEncoder::decode_contract_code(&encoded).unwrap();

        assert_eq!(contract_code.trim(), decoded.trim());
    }

    #[test]
    fn test_encode_contract_code_data() {
        let contract_code = r#"
        fn main() {
            println!("Hello, blockchain!");
        }
        "#;

        let encoded_data = SmartContractEncoder::encode_contract_code_data(contract_code, 128).unwrap();
        assert!(!encoded_data.is_empty());
    }

    #[test]
    fn test_encode_contract_data() {
        let contract_data = r#"
        {
            "name": "MySmartContract",
            "version": "1.0.0",
            "author": "Alice"
        }
        "#;

        let encoded_data = SmartContractEncoder::encode_contract_data(contract_data, 128, 3).unwrap();
        assert!(!encoded_data.is_empty());
    }
}

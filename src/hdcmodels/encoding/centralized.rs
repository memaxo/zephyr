use rand::Rng;
use crate::chain::state::State;
use serde_json;
use std::error::Error;

pub fn encode_transactional_data(data: &[Transaction], dimension: usize) -> Vec<f64> {
    let mut encoded_data = Vec::with_capacity(data.len() * dimension);
    for transaction in data {
        let transaction_vector = random_projection(&transaction.to_string(), dimension);
        encoded_data.extend(transaction_vector);
    }
    encoded_data
}

pub fn encode_smart_contract(contract: &str, dimension: usize, n: usize) -> Vec<f64> {
    let tokens = tokenize_smart_contract(contract, n);
    let token_vectors = tokens
        .iter()
        .map(|token| random_projection(token, dimension))
        .collect::<Vec<Vec<f64>>>();

    // Placeholder for combining token vectors using HDC operations
    // Replace this with the actual implementation
    token_vectors.iter().flatten().cloned().collect()
}

pub fn encode_rust_code(code: &str, dimension: usize) -> Vec<f64> {
    let tokens = tokenize_rust_code(code);
    let token_vectors = tokens
        .iter()
        .map(|token| random_projection(token, dimension))
        .collect::<Vec<Vec<f64>>>();

    // Placeholder for combining token vectors using HDC operations
    // Replace this with the actual implementation
    token_vectors.iter().flatten().cloned().collect()
}

pub fn encode_natural_language(text: &str, dimension: usize) -> Vec<f64> {
    let tokens = tokenize_natural_language(text);
    let token_vectors = tokens
        .iter()
        .map(|token| random_projection(token, dimension))
        .collect::<Vec<Vec<f64>>>();

    // Placeholder for combining token vectors using HDC operations
    // Replace this with the actual implementation
    token_vectors.iter().flatten().cloned().collect()
}

fn tokenize_rust_code(code: &str) -> Vec<String> {
    // Placeholder for Rust code tokenization logic
    // Replace this with the actual implementation
    vec![
        "fn".to_string(),
        "main".to_string(),
        "()".to_string(),
        "{}".to_string(),
    ]
}

fn tokenize_natural_language(text: &str) -> Vec<String> {
    // Placeholder for natural language tokenization logic
    // Replace this with the actual implementation
    text.split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn tokenize_smart_contract(contract: &str, n: usize) -> Vec<String> {
    // Placeholder for smart contract tokenization logic using N-grams
    // Replace this with the actual implementation
    vec![
        "token1".to_string(),
        "token2".to_string(),
        "token3".to_string(),
    ]
}

fn random_projection(token: &str, dimension: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..dimension).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

pub struct StateEncoder;

impl StateEncoder {
    pub fn encode_state(state: &State) -> Result<String, serde_json::Error> {
        serde_json::to_string(state)
    }

    pub fn decode_state(data: &str) -> Result<State, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn encode_state_data(state: &State, dimension: usize) -> Result<Vec<f64>, Box<dyn Error>> {
        let encoded_data = encode_state_data(state, dimension);
        Ok(encoded_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::state::Account;

    #[test]
    fn test_encode_decode_state() {
        let state = State {
            accounts: vec![
                Account {
                    address: "Alice".to_string(),
                    balance: 100.0,
                    nonce: 1,
                },
                Account {
                    address: "Bob".to_string(),
                    balance: 50.0,
                    nonce: 2,
                },
            ],
        };

        let encoded = StateEncoder::encode_state(&state).unwrap();
        let decoded: State = StateEncoder::decode_state(&encoded).unwrap();

        assert_eq!(state.accounts.len(), decoded.accounts.len());
        assert_eq!(state.accounts[0].address, decoded.accounts[0].address);
        assert_eq!(state.accounts[0].balance, decoded.accounts[0].balance);
        assert_eq!(state.accounts[0].nonce, decoded.accounts[0].nonce);
    }

    #[test]
    fn test_encode_state_data() {
        let state = State {
            accounts: vec![
                Account {
                    address: "Alice".to_string(),
                    balance: 100.0,
                    nonce: 1,
                },
                Account {
                    address: "Bob".to_string(),
                    balance: 50.0,
                    nonce: 2,
                },
            ],
        };

        let encoded_data = StateEncoder::encode_state_data(&state, 128).unwrap();
        assert!(!encoded_data.is_empty());
    }
}

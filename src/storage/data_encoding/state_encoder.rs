use crate::chain::state::State;
use crate::hdcmodels::encoding::encode_state_data;
use serde_json;
use std::error::Error;

/// Struct for encoding and decoding state data.
pub struct StateEncoder;

impl StateEncoder {
    /// Encodes state data into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `state` - A reference to the state to be encoded.
    ///
    /// # Returns
    ///
    /// * `Result<String, serde_json::Error>` - A JSON string representation of the state or an error.
    pub fn encode_state(state: &State) -> Result<String, serde_json::Error> {
        serde_json::to_string(state)
    }

    /// Decodes a JSON string into state data.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON string representation of the state.
    ///
    /// # Returns
    ///
    /// * `Result<State, serde_json::Error>` - A state object or an error.
    pub fn decode_state(data: &str) -> Result<State, serde_json::Error> {
        serde_json::from_str(data)
    }

    /// Encodes state data into high-dimensional vectors.
    ///
    /// # Arguments
    ///
    /// * `state` - A reference to the state to be encoded.
    /// * `dimension` - The dimension of the high-dimensional vectors.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<f64>, Box<dyn Error>>` - A vector of high-dimensional vectors or an error.
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

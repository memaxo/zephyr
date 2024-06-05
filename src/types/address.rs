use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub Vec<u8>);

impl Address {
    pub fn new(bytes: Vec<u8>) -> Self {
        Address(bytes)
    }

    pub fn from_hex(hex: &str) -> Result<Self, AddressError> {
        let bytes = hex::decode(hex).map_err(|_| AddressError::InvalidHexFormat)?;
        Ok(Address(bytes))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Debug)]
pub enum AddressError {
    InvalidHexFormat,
}
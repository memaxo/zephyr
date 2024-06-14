use rand::Rng;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DID {
    method: String,
    identifier: String,
}

impl DID {
    pub fn new(method: String, identifier: String) -> Self {
        Self { method, identifier }
    }

    pub fn from_str(did_str: &str) -> Result<Self, DIDError> {
        let parts: Vec<&str> = did_str.split(':').collect();
        if parts.len() != 3 || parts[0] != "did" {
            return Err(DIDError::InvalidFormat);
        }
        Ok(Self {
            method: parts[1].to_string(),
            identifier: parts[2].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("did:{}:{}", self.method, self.identifier)
    }

    pub fn is_valid(&self) -> bool {
        !self.method.is_empty() && !self.identifier.is_empty()
    }

    pub fn resolve(&self) -> Result<DIDDocument, DIDError> {
        // Placeholder for actual resolution logic
        Ok(DIDDocument::new(self.clone()))
    }

    pub fn generate_new() -> Self {
        let identifier: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        Self {
            method: "zephyr".to_string(),
            identifier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DIDDocument {
    did: DID,
    // Placeholder for actual DID document fields
}

impl DIDDocument {
    pub fn new(did: DID) -> Self {
        Self { did }
    }
}

#[derive(Debug)]
pub enum DIDError {
    InvalidFormat,
    ResolutionFailure,
}

impl fmt::Display for DIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIDError::InvalidFormat => write!(f, "Invalid DID format"),
            DIDError::ResolutionFailure => write!(f, "Failed to resolve DID"),
        }
    }
}

impl std::error::Error for DIDError {}

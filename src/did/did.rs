use rand::Rng;
use std::fmt;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::Address;
use web3::transports::Http;
use web3::Web3;

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

    pub fn resolve(&self, web3: &Web3<Http>, contract_address: Address) -> Result<DIDDocument, DIDError> {
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("../../contracts/DIDRegistry.json"),
        ).unwrap();

        let did_doc: String = contract.query("didDocuments", (self.to_string(),), None, Options::default(), None).wait().unwrap();
        DIDDocument::from_json(&did_doc).map_err(|_| DIDError::ResolutionFailure)
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

    pub fn register(&self, web3: &Web3<Http>, contract_address: Address, did_doc: DIDDocument) -> Result<(), DIDError> {
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("../../contracts/DIDRegistry.json"),
        ).unwrap();

        let result = contract.call("registerDID", (self.to_string(), did_doc.to_json()), None, Options::default()).wait();
        result.map_err(|_| DIDError::RegistrationFailure)
    }

    pub fn update(&self, web3: &Web3<Http>, contract_address: Address, did_doc: DIDDocument) -> Result<(), DIDError> {
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("../../contracts/DIDRegistry.json"),
        ).unwrap();

        let result = contract.call("updateDID", (self.to_string(), did_doc.to_json()), None, Options::default()).wait();
        result.map_err(|_| DIDError::UpdateFailure)
    }

    pub fn deactivate(&self, web3: &Web3<Http>, contract_address: Address) -> Result<(), DIDError> {
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("../../contracts/DIDRegistry.json"),
        ).unwrap();

        let result = contract.call("deactivateDID", (self.to_string(),), None, Options::default()).wait();
        result.map_err(|_| DIDError::DeactivationFailure)
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

    pub fn from_json(json_str: &str) -> Result<Self, DIDDocumentError> {
        serde_json::from_str(json_str).map_err(|_| DIDDocumentError::InvalidFormat)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "Invalid DID document".to_string())
    }
}

#[derive(Debug)]
pub enum DIDError {
    InvalidFormat,
    ResolutionFailure,
    RegistrationFailure,
    UpdateFailure,
    DeactivationFailure,
}

impl fmt::Display for DIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIDError::InvalidFormat => write!(f, "Invalid DID format"),
            DIDError::ResolutionFailure => write!(f, "Failed to resolve DID"),
            DIDError::RegistrationFailure => write!(f, "Failed to register DID"),
            DIDError::UpdateFailure => write!(f, "Failed to update DID"),
            DIDError::DeactivationFailure => write!(f, "Failed to deactivate DID"),
        }
    }
}

impl std::error::Error for DIDError {}

#[derive(Debug)]
pub enum DIDDocumentError {
    InvalidFormat,
}

impl fmt::Display for DIDDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIDDocumentError::InvalidFormat => write!(f, "Invalid DID document format"),
        }
    }
}

impl std::error::Error for DIDDocumentError {}

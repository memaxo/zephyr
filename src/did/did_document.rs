use serde::{Deserialize, Serialize};
use serde_json;
use crate::did::did::{DID, DIDError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub context: Vec<String>,  
    pub id: DID,                 
    pub controller: Option<Vec<DID>>,   
    pub alsoKnownAs: Option<Vec<String>>, 
    pub verificationMethod: Option<Vec<VerificationMethod>>, 
    pub authentication: Option<Vec<VerificationMethod>>,  
    pub assertionMethod: Option<Vec<VerificationMethod>>, 
    pub keyAgreement: Option<Vec<VerificationMethod>>, 
    pub capabilityInvocation: Option<Vec<VerificationMethod>>, 
    pub capabilityDelegation: Option<Vec<VerificationMethod>>, 
    pub service: Option<Vec<ServiceEndpoint>>,  
}

impl DIDDocument {
    pub fn new(
        id: DID,
        verificationMethod: Option<Vec<VerificationMethod>>,
        authentication: Option<Vec<VerificationMethod>>,
        assertionMethod: Option<Vec<VerificationMethod>>,
        keyAgreement: Option<Vec<VerificationMethod>>,
        capabilityInvocation: Option<Vec<VerificationMethod>>,
        capabilityDelegation: Option<Vec<VerificationMethod>>,
        service: Option<Vec<ServiceEndpoint>>,
    ) -> Self {
        Self {
            context: vec!["https://www.w3.org/ns/did/v1".to_string()],
            id,
            controller: None,
            alsoKnownAs: None,
            verificationMethod,
            authentication,
            assertionMethod,
            keyAgreement,
            capabilityInvocation,
            capabilityDelegation,
            service,
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, DIDDocumentError> {
        serde_json::from_str(json_str).map_err(|_| DIDDocumentError::InvalidFormat)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "Invalid DID document".to_string())
    }

    pub fn verify_signature(&self, signature: &str, data: &str) -> Result<bool, DIDDocumentError> {
        // Placeholder for actual signature verification logic
        Ok(true)
    }

    pub fn add_verification_method(&mut self, method: VerificationMethod) {
        self.public_key.push(method);
    }

    pub fn remove_verification_method(&mut self, id: &str) {
        self.public_key.retain(|method| method.id != id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    Ed25519VerificationKey2018 { id: String, public_key: String },
    EcdsaSecp256k1VerificationKey2019 { id: String, public_key: String },
    X25519KeyAgreementKey2019 { id: String, public_key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    id: String,
    type_: String,
    service_endpoint: String,
}

#[derive(Debug)]
pub enum DIDDocumentError {
    InvalidFormat,
    VerificationFailure,
}

impl std::fmt::Display for DIDDocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DIDDocumentError::InvalidFormat => write!(f, "Invalid DID document format"),
            DIDDocumentError::VerificationFailure => write!(f, "Failed to verify DID document"),
        }
    }
}

impl std::error::Error for DIDDocumentError {}

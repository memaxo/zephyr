use serde::{Deserialize, Serialize};
use crate::did::did::DID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub type_: Vec<String>,
    pub issuer: DID,
    pub issuanceDate: String,
    pub credentialSubject: CredentialSubject,
    pub proof: Proof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    pub id: DID,
    pub claims: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub type_: String,
    pub created: String,
    pub proofPurpose: String,
    pub verificationMethod: String,
    pub jws: String,
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        type_: Vec<String>,
        issuer: DID,
        issuanceDate: String,
        credentialSubject: CredentialSubject,
        proof: Proof,
    ) -> Self {
        Self {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id,
            type_,
            issuer,
            issuanceDate,
            credentialSubject,
            proof,
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, VerifiableCredentialError> {
        serde_json::from_str(json_str).map_err(|_| VerifiableCredentialError::InvalidFormat)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "Invalid Verifiable Credential".to_string())
    }

    pub fn verify(&self) -> Result<bool, VerifiableCredentialError> {
        // Placeholder for actual verification logic
        Ok(true)
    }
}

#[derive(Debug)]
pub enum VerifiableCredentialError {
    InvalidFormat,
    VerificationFailure,
}

impl std::fmt::Display for VerifiableCredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifiableCredentialError::InvalidFormat => write!(f, "Invalid Verifiable Credential format"),
            VerifiableCredentialError::VerificationFailure => write!(f, "Failed to verify Verifiable Credential"),
        }
    }
}

impl std::error::Error for VerifiableCredentialError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::did::did::{DID, DIDDocument};
    use crate::did::verifiable_credential::{VerifiableCredential, CredentialSubject, Proof};
    use crate::did::key_manager::KeyManager;
    use crate::did::qup::quantum_random::QuantumRandom;
    use crate::did::qup::quantum_safe_hash::QuantumSafeHash;

    #[test]
    fn test_did_creation() {
        let did = DID::generate_new();
        assert!(did.is_valid());
    }

    #[test]
    fn test_did_registration() {
        // Placeholder for actual test logic
    }

    #[test]
    fn test_did_resolution() {
        // Placeholder for actual test logic
    }

    #[test]
    fn test_verifiable_credential() {
        let did = DID::generate_new();
        let credential_subject = CredentialSubject {
            id: did.clone(),
            claims: serde_json::json!({"name": "Alice"}),
        };
        let proof = Proof {
            type_: "Ed25519Signature2018".to_string(),
            created: "2023-10-01T00:00:00Z".to_string(),
            proofPurpose: "assertionMethod".to_string(),
            verificationMethod: "did:example:123#key-1".to_string(),
            jws: "eyJhbGciOiJFZERTQSJ9..".to_string(),
        };
        let vc = VerifiableCredential::new(
            "http://example.edu/credentials/3732".to_string(),
            vec!["VerifiableCredential".to_string()],
            did,
            "2023-10-01T00:00:00Z".to_string(),
            credential_subject,
            proof,
        );
        assert!(vc.verify().is_ok());
    }
}

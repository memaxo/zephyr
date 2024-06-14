use crate::did::did::{DID, DIDDocument, DIDError};

pub trait DIDResolver {
    fn resolve(&self, did: &DID) -> Result<DIDDocument, DIDError>;
}

pub struct ZephyrChainResolver;

impl DIDResolver for ZephyrChainResolver {
    fn resolve(&self, did: &DID) -> Result<DIDDocument, DIDError> {
        // 1. Fetch DID Document from Zephyr Chain
        let did_doc_string = match self.blockchain_client.get_did_document(did.to_string()) {
            Ok(doc) => doc,
            Err(err) => {
                return Err(DIDError::ResolutionError(format!(
                    "Failed to fetch DID Document: {}",
                    err
                )))
            }
        };

        // 2. Parse DID Document from JSON
        let did_doc: DIDDocument = match serde_json::from_str(&did_doc_string) {
            Ok(doc) => doc,
            Err(err) => {
                return Err(DIDError::InvalidDIDDocument(format!(
                    "Failed to parse DID Document: {}",
                    err
                )))
            }
        };

        // 3. Additional Verification (Optional)
        // You might want to add extra checks here, like verifying the DID
        // document's signature or validating its structure more thoroughly.

        Ok(did_doc)
    }
}

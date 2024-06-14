use crate::did::did::{DID, DIDDocument, DIDError};
use web3::transports::Http;
use web3::Web3;
use web3::types::Address;

pub trait DIDResolver {
    fn resolve(&self, did: &DID) -> Result<DIDDocument, DIDError>;
}

pub struct ZephyrChainResolver {
    web3: Web3<Http>,
    contract_address: Address,
}

impl ZephyrChainResolver {
    pub fn new(web3: Web3<Http>, contract_address: Address) -> Self {
        Self { web3, contract_address }
    }
}

impl DIDResolver for ZephyrChainResolver {
    fn resolve(&self, did: &DID) -> Result<DIDDocument, DIDError> {
        did.resolve(&self.web3, self.contract_address)
    }
}

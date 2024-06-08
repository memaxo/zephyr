use crate::network::{Network, ProtocolMessage};
use crate::storage::{Storage, QUPStorage};
use crate::smart_contracts::{SmartContract, SmartContractExecutor};
use crate::qup::{QUPCrypto, QUPState, QUPBlock};

pub struct QUPIntegration {
    network: Network,
    storage: QUPStorage,
    smart_contract_executor: SmartContractExecutor,
    qup_crypto: QUPCrypto,
    qup_state: QUPState,
}

impl QUPIntegration {
    pub fn new(
        network: Network,
        storage: QUPStorage,
        smart_contract_executor: SmartContractExecutor,
        qup_crypto: QUPCrypto,
        qup_state: QUPState,
    ) -> Self {
        QUPIntegration {
            network,
            storage,
            smart_contract_executor,
            qup_crypto,
            qup_state,
        }
    }

    pub fn broadcast_block(&self, block: QUPBlock) -> Result<(), String> {
        let message = ProtocolMessage::Block(block);
        self.network.broadcast(message).map_err(|e| e.to_string())
    }

    pub fn store_block(&self, block: QUPBlock) -> Result<(), String> {
        self.storage.put_block(block).map_err(|e| e.to_string())
    }

    pub fn execute_smart_contract(&self, contract: SmartContract) -> Result<(), String> {
        self.smart_contract_executor.execute(contract).map_err(|e| e.to_string())
    }

    pub fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        self.qup_crypto.sign(data).map_err(|e| e.to_string())
    }

    pub fn verify_signature(&self, data: &[u8], signature: &[u8]) -> Result<bool, String> {
        self.qup_crypto.verify(data, signature).map_err(|e| e.to_string())
    }

    pub fn update_state(&mut self, new_state: QUPState) {
        self.qup_state = new_state;
    }
}

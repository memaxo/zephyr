use crate::network::{Network, ProtocolMessage};
use crate::storage::{Storage, QUPStorage};
use crate::smart_contracts::{SmartContract, SmartContractExecutor};
use crate::qup::{QUPCrypto, QUPState, QUPBlock};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

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

    // New methods for seamless communication and data exchange with external systems

    pub fn send_data_to_external_system(&self, address: &str, data: &[u8]) -> Result<(), String> {
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.write_all(data).map_err(|e| e.to_string())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn receive_data_from_external_system(&self, address: &str) -> Result<Vec<u8>, String> {
        match TcpListener::bind(address) {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            let mut buffer = Vec::new();
                            stream.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                            return Ok(buffer);
                        },
                        Err(e) => return Err(e.to_string()),
                    }
                }
                Err("No incoming connections".to_string())
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

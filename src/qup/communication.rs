use crate::qup::block::QUPBlock;
use crate::qup::crypto::{QUPKeyPair, encrypt_data, decrypt_data, sign_data, verify_signature, hash_data};
use crate::qup::state::QUPState;
use crate::network::{NetworkMessage, NetworkSender, NetworkReceiver, discover_peers, connect_to_peer, disconnect_from_peer};
use crate::error::ConsensusError;
use std::sync::Arc;

pub enum NodeType {
    Classical,
    Quantum,
}

pub struct CommunicationProtocol {
    pub node_type: NodeType,
    pub key_pair: QUPKeyPair,
    pub peers: Vec<String>,
    pub sender: NetworkSender,
    pub receiver: NetworkReceiver,
}

impl CommunicationProtocol {
    pub fn new(node_type: NodeType, key_pair: QUPKeyPair, sender: NetworkSender, receiver: NetworkReceiver) -> Self {
        CommunicationProtocol { 
            node_type, 
            key_pair,
            peers: Vec::new(),
            sender,
            receiver,
        }
    }

    pub fn discover_peers(&mut self) -> Result<(), ConsensusError> {
        let discovered_peers = discover_peers()?;
        self.peers.extend(discovered_peers);
        Ok(())
    }

    pub fn connect_to_peer(&mut self, peer: &str) -> Result<(), ConsensusError> {
        connect_to_peer(peer)?;
        if !self.peers.contains(&peer.to_string()) {
            self.peers.push(peer.to_string());
        }
        Ok(())
    }

    pub fn disconnect_from_peer(&mut self, peer: &str) -> Result<(), ConsensusError> {
        disconnect_from_peer(peer)?;
        self.peers.retain(|p| p != peer);
        Ok(())
    }

    pub async fn send_message(&self, message: NetworkMessage) -> Result<(), ConsensusError> {
        let serialized_message = bincode::serialize(&message)?;
        self.sender.send(serialized_message).await?;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<NetworkMessage, ConsensusError> {
        let serialized_message = self.receiver.receive().await?;
        let message: NetworkMessage = bincode::deserialize(&serialized_message)?;
        self.authenticate_message(&message)?;
        self.verify_message_integrity(&message)?;
        Ok(message)
    }

    pub fn send_proof(&self, proof: &[u8], recipient: &str) -> Result<(), ConsensusError> {
        let encrypted_proof = encrypt_data(proof, &self.key_pair)?;
        let signature = sign_data(&encrypted_proof, &self.key_pair)?;
        let message = NetworkMessage::Proof {
            proof: encrypted_proof,
            signature,
        };
        send_message(recipient, message)
    }

    pub fn receive_proof(&self, message: NetworkMessage) -> Result<Vec<u8>, ConsensusError> {
        if let NetworkMessage::Proof { proof, signature } = message {
            verify_signature(&proof, &signature, &self.key_pair)?;
            decrypt_data(&proof, &self.key_pair)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    pub fn send_result(&self, result: &[u8], recipient: &str) -> Result<(), ConsensusError> {
        let encrypted_result = encrypt_data(result, &self.key_pair)?;
        let signature = sign_data(&encrypted_result, &self.key_pair)?;
        let message = NetworkMessage::Result {
            result: encrypted_result,
            signature,
        };
        send_message(recipient, message)
    }

    pub fn receive_result(&self, message: NetworkMessage) -> Result<Vec<u8>, ConsensusError> {
        if let NetworkMessage::Result { result, signature } = message {
            verify_signature(&result, &signature, &self.key_pair)?;
            decrypt_data(&result, &self.key_pair)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    fn authenticate_message(&self, message: &NetworkMessage) -> Result<(), ConsensusError> {
        match message {
            NetworkMessage::Proof { proof, signature } => {
                let hashed_proof = hash_data(proof)?;
                verify_signature(&hashed_proof, signature, &self.key_pair)
            }
            NetworkMessage::Result { result, signature } => {
                let hashed_result = hash_data(result)?;
                verify_signature(&hashed_result, signature, &self.key_pair)
            }
            _ => Ok(()),
        }
    }

    fn verify_message_integrity(&self, message: &NetworkMessage) -> Result<(), ConsensusError> {
        match message {
            NetworkMessage::Proof { proof, .. } => {
                let decrypted_proof = decrypt_data(proof, &self.key_pair)?;
                let hashed_proof = hash_data(&decrypted_proof)?;
                if hashed_proof != *proof {
                    return Err(ConsensusError::MessageIntegrityError);
                }
                Ok(())
            }
            NetworkMessage::Result { result, .. } => {
                let decrypted_result = decrypt_data(result, &self.key_pair)?;
                let hashed_result = hash_data(&decrypted_result)?;
                if hashed_result != *result {
                    return Err(ConsensusError::MessageIntegrityError);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

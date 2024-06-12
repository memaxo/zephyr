use crate::error_handling::network_error::NetworkError;
use crate::network::handler::Handler;
use crate::network::p2p::message::Message;
use crate::network::p2p::peer::Peer;
use crate::network::protocol::{ProtocolMessage, HANDSHAKE_TIMEOUT, PING_INTERVAL};
use crate::network::tls::{PostQuantumTLSConnection, PostQuantumTLSConfig};
use crate::qup::crypto::PostQuantumCrypto;
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::oneshot;

pub struct Client {
    peer: Arc<Peer>,
    handler: Arc<dyn Handler>,
    pq_tls_connection: Option<PostQuantumTLSConnection>,
    crypto: Arc<PostQuantumCrypto>,
}

impl Client {
    pub fn new(peer_address: String, handler: Arc<dyn Handler>, crypto: Arc<PostQuantumCrypto>) -> Self {
        let peer = Arc::new(Peer::new(peer_address));
        Client {
            peer,
            handler,
            pq_tls_connection: None,
            crypto,
        }
    }

    pub async fn start(&mut self) -> Result<(), NetworkError> {
        info!("Connecting to peer: {}", self.peer.address);

        // Configure and establish the TLS connection using rustls
        let config = PostQuantumTLSConfig::new();
        let stream = TcpStream::connect(&self.peer.address).await.map_err(|e| {
            error!("Failed to connect to peer: {}", e);
            NetworkError::ConnectionError(format!("Failed to connect to peer: {}", e))
        })?;

        let mut pq_tls_connection = PostQuantumTLSConnection::new(stream, config).await.map_err(|e| {
            error!("TLS connection establishment failed: {}", e);
            NetworkError::ConnectionError(format!("TLS connection establishment failed: {}", e))
        })?;

        self.pq_tls_connection = Some(pq_tls_connection);
        info!("TLS connection established with peer: {}", self.peer.address);

        loop {
            // Receive messages using the quantum-resistant connection
            match self.pq_tls_connection.as_mut().unwrap().receive().await {
                Ok(message) => {
                    // Deserialize and handle the received message
                    match ProtocolMessage::deserialize(&message, &self.crypto) {
                        Ok(protocol_message) => {
                            match validate_protocol_message(&protocol_message) {
                                Ok(ProtocolMessage::Pong) => {
                                    // Received pong response, do nothing
                                }
                                Ok(ProtocolMessage::QKDKeyRequest) => {
                                    // Handle QKD key request
                                    if let Err(e) = self.handle_qkd_key_request().await {
                                        error!("Failed to handle QKD key request: {}", e);
                                    }
                                }
                                Ok(ProtocolMessage::QKDKeyResponse(key)) => {
                                    // Handle QKD key response
                                    if let Err(e) = self.handle_qkd_key_response(key).await {
                                        error!("Failed to handle QKD key response: {}", e);
                                    }
                                }
                                Ok(ProtocolMessage::QKDKeyConfirmation) => {
                                    // Handle QKD key confirmation
                                    if let Err(e) = self.handle_qkd_key_confirmation().await {
                                        error!("Failed to handle QKD key confirmation: {}", e);
                                    }
                                }
                                Ok(ProtocolMessage::QuantumStateDistribution(state)) => {
                                    // Handle quantum state distribution
                                    if let Err(e) = self.handle_quantum_state_distribution(state).await {
                                        error!("Failed to handle quantum state distribution: {}", e);
                                    }
                                }
                                Ok(ProtocolMessage::QuantumStateMeasurementResults(results)) => {
                                    // Handle quantum state measurement results
                                    if let Err(e) = self.handle_quantum_state_measurement_results(results).await {
                                        error!("Failed to handle quantum state measurement results: {}", e);
                                    }
                                }
                                Err(e) => {
                                    error!("Invalid protocol message: {}", e);
                                    break;
                                }
                                _ => {
                                    match Message::from_protocol_message(protocol_message) {
                                        Ok(zephyr_message) => {
                                            let (response_sender, response_receiver) = oneshot::channel();
                                            self.handler.handle_message(&self.peer, zephyr_message, Some(response_sender)).await;
                                            if let Ok(response) = response_receiver.await {
                                                // Serialize and send the response using the quantum-resistant connection
                                                match response.serialize(&self.crypto) {
                                                    Ok(serialized_response) => {
                                                        if let Err(e) = self.pq_tls_connection.as_mut().unwrap().send(&serialized_response).await {
                                                            error!("Failed to send response message: {}", e);
                                                        }
                                                    }
                                                    Err(e) => {
                                                        error!("Failed to serialize response message: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to convert protocol message to zephyr message: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize message: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to receive message: {}", e);
                }
            }
        }
    }

    async fn handle_qkd_key_request(&mut self) {
        // Perform QKD key request logic
        // ...
    }

    async fn handle_qkd_key_response(&mut self, key: QKDKey) {
        // Perform QKD key response logic
        // ...
    }

    async fn handle_qkd_key_confirmation(&mut self) {
        // Perform QKD key confirmation logic
        // ...
    }

    async fn handle_quantum_state_distribution(&mut self, state: QuantumState) {
        // Perform quantum state distribution logic
        // ...
    }

    async fn handle_quantum_state_measurement_results(&mut self, results: Vec<bool>) {
        // Perform quantum state measurement results logic
        // ...
    }
}

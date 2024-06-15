use crate::network::handler::Handler;
use crate::network::p2p::message::Message;
use crate::network::p2p::peer::Peer;
use crate::network::protocol::{
    ProtocolMessage, HANDSHAKE_TIMEOUT, MAX_MESSAGE_SIZE, PING_INTERVAL, PONG_TIMEOUT,
};
use crate::qup::crypto::PostQuantumCrypto;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{debug, error, info};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::sync::Arc;
use parking_lot::Mutex;
use crossbeam_utils::thread;
use tokio::sync::oneshot;

pub struct Server {
    address: String,
    handler: Arc<dyn Handler>,
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    crypto: Arc<PostQuantumCrypto>,
    rate_limiter: Arc<RateLimiter<String>>,
    blacklist: Arc<Mutex<HashSet<String>>>,
}

const RATE_LIMIT: u32 = 10; // 10 requests per second
const BLACKLIST_THRESHOLD: u32 = 100; // 100 requests per minute

impl Server {
    pub fn new(address: String, handler: Arc<dyn Handler>, crypto: Arc<PostQuantumCrypto>) -> Self {
        let rate_limiter = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(RATE_LIMIT))));
        let blacklist = Arc::new(Mutex::new(HashSet::new()));

        Server {
            rate_limiter,
            blacklist,
            address,
            handler,
            peers: Arc::new(Mutex::new(HashMap::new())),
            crypto,
        }

        // Implement error handling for model outputs
        let timeout_duration = Duration::from_secs(30);
        match tokio::time::timeout(timeout_duration, async {
            // Logic to wait for model outputs
        }).await {
            Ok(outputs) => {
                if !validate_model_outputs(&outputs) {
                    error!("Received invalid model outputs");
                    // Request verification from other nodes
                    // Penalize the node if necessary
                }
            }
            Err(e) => {
                error!("Timeout waiting for model outputs: {}", e);
                // Retry logic or flag the node as unreliable
            }
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind server");
        info!("Server listening on {}", self.address);

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let peer_address = addr.to_string();
                    debug!("New connection from {}", peer_address);

                    let blacklist = self.blacklist.clone();
                    let rate_limiter = self.rate_limiter.clone();

                    if blacklist.lock().unwrap().contains(&peer_address) {
                        error!("Connection attempt from blacklisted IP: {}", peer_address);
                        continue;
                    }

                    if !rate_limiter.check_key(&peer_address).is_ok() {
                        error!("Rate limit exceeded for IP: {}", peer_address);
                        let mut blacklist = blacklist.lock().unwrap();
                        *blacklist.entry(peer_address.clone()).or_insert(0) += 1;
                        if *blacklist.get(&peer_address).unwrap() > BLACKLIST_THRESHOLD {
                            blacklist.insert(peer_address.clone());
                            error!("IP blacklisted: {}", peer_address);
                        }
                        continue;
                    }
                    let handler = self.handler.clone();
                    let peers = self.peers.clone();
                    let crypto = self.crypto.clone();

                    tokio::spawn(async move {
                        match PostQuantumTLSConnection::new(stream, config).await {
                            Ok(mut pq_tls_connection) => {
                                if let Err(e) = handle_connection(pq_tls_connection, peer_address, handler, peers, crypto).await {
                                    error!("Error handling connection: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("TLS connection establishment failed: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    mut pq_tls_connection: PostQuantumTLSConnection,
    peer_address: String, 
    handler: Arc<dyn Handler>,
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    crypto: Arc<PostQuantumCrypto>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut peer = Peer::new(peer_address.clone());

    loop {
        // Receive messages using the quantum-resistant connection
        let message = match pq_tls_connection.receive().await {
            Ok(msg) => msg,
            Err(e) => {
                error!("Failed to receive message over post-quantum TLS: {}", e);
                break;
            }
        };

        // Deserialize and handle the received message
        let protocol_message = match ProtocolMessage::deserialize(&message, &crypto) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Failed to deserialize message: {}", e);
                break;
            }
        };

        match validate_protocol_message(&protocol_message) {
            Ok(ProtocolMessage::Ping) => {
                let pong_msg = ProtocolMessage::Pong;
                let serialized_pong = pong_msg.serialize(&crypto)?;
                pq_tls_connection
                    .send(&serialized_pong)
                    .await?;
            }
            Ok(ProtocolMessage::Pong) => {
                // Received pong response, do nothing
            }
            Ok(ProtocolMessage::QKDKeyRequest) => {
                // Handle QKD key request
                handle_qkd_key_request(&mut pq_tls_connection, &peer_address, &crypto).await?;
            }
            Ok(ProtocolMessage::QKDKeyResponse(key)) => {
                // Handle QKD key response
                handle_qkd_key_response(&mut pq_tls_connection, &peer_address, key, &crypto)
                    .await?;
            }
            Ok(ProtocolMessage::QKDKeyConfirmation) => {
                // Handle QKD key confirmation
                handle_qkd_key_confirmation(&mut pq_tls_connection, &peer_address, &crypto)
                    .await?;
            }
            Ok(ProtocolMessage::QuantumStateDistribution(state)) => {
                // Handle quantum state distribution
                store_quantum_state(state).await?;
                if is_worker_node() {
                    use_quantum_state_for_training(state).await?;
                }
            }
            Ok(ProtocolMessage::QuantumStateMeasurementResults(results)) => {
                // Handle quantum state measurement results
                aggregate_measurement_results(results).await?;
            }
            Err(e) => {
                error!("Invalid protocol message: {}", e);
                break;
            }
            Message::RequestModelOutputs(inputs) => {
                // Handle RequestModelOutputs
                let model_outputs = self.retrieve_model_outputs(&inputs).await;
                let response_message = Message::ResponseModelOutputs(model_outputs);
                if let Err(e) = pq_tls_connection.send(&response_message.serialize()?).await {
                    error!("Failed to send ResponseModelOutputs: {}", e);
                }
            }
            Message::ResponseModelOutputs(_) => {
                // No specific action required for the server
            }
            Message::QKDKeyRequest => {
                // Handle QKDKeyRequest
                let (private_key, public_key) = self.crypto.generate_qkd_key_pair().unwrap();
                self.qkd_private_keys.insert(peer_address.clone(), private_key);
                let response_message = Message::QKDKeyResponse(public_key);
                if let Err(e) = pq_tls_connection.send(&response_message.serialize()?).await {
                    error!("Failed to send QKDKeyResponse: {}", e);
                }
            }
            Message::QKDKeyResponse(public_key) => {
                // Handle QKDKeyResponse
                self.qkd_public_keys.insert(peer_address.clone(), public_key);
                let confirmation_message = Message::QKDKeyConfirmation;
                if let Err(e) = pq_tls_connection.send(&confirmation_message.serialize()?).await {
                    error!("Failed to send QKDKeyConfirmation: {}", e);
                }
            }
            Message::QKDKeyConfirmation => {
                // Handle QKDKeyConfirmation
                self.qkd_completed_peers.insert(peer_address.clone());
                info!("QKD key exchange completed with peer: {}", peer_address);
            }
            Message::VerificationGameChallenge(challenge) => {
                // Handle VerificationGameChallenge
                let response = self.handle_verification_game_challenge(challenge).await;
                let response_message = Message::VerificationGameResponse(response);
                if let Err(e) = pq_tls_connection.send(&response_message.serialize()?).await {
                    error!("Failed to send VerificationGameResponse: {}", e);
                }
            }
            Message::VerificationGameResponse(response) => {
                // Handle VerificationGameResponse
                self.handle_verification_game_response(response).await;
            }
            _ => {
                let zephyr_message = Message::from_protocol_message(protocol_message)?;
                let (response_sender, response_receiver) = oneshot::channel();
                handler
                    .handle_message(&peer, zephyr_message, Some(response_sender))
                    .await;
                if let Ok(response) = response_receiver.await {
                    // Serialize and send the response using the quantum-resistant connection
                    let serialized_response = response.serialize(&crypto)?;
                    pq_tls_connection  
                        .send(&serialized_response)
                        .await?;
                }
            }
        }
    }

    // Remove the peer from the list of connected peers
    peers.lock().unwrap().remove(&peer_address);

    // Update peer selection weights based on reputation
    self.update_peer_selection_weights().await;

    let mut blacklist = peers.lock().unwrap();
    *blacklist.entry(peer_address.clone()).or_insert(0) += 1;
    if *blacklist.get(&peer_address).unwrap() > BLACKLIST_THRESHOLD {
        blacklist.insert(peer_address.clone());
        error!("IP blacklisted: {}", peer_address);
    }

    Ok(())
}

async fn handle_qkd_key_request(
    quantum_connection: &mut QuantumResistantConnection,
    peer_address: &str,
    crypto: &QUPCrypto,
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new QKD key pair
    let (private_key, public_key) = crypto.generate_qkd_key_pair()?;

    // Serialize and encrypt the public key using the peer's public key
    let serialized_public_key = bincode::serialize(&public_key)?;
    let encrypted_public_key =
        crypto.encrypt(&serialized_public_key, &quantum_connection.peer_public_key)?;

    // Send the encrypted public key to the peer
    let qkd_key_response = ProtocolMessage::QKDKeyResponse {
        key: encrypted_public_key,
    };
    quantum_connection
        .send_message(&peer_address, &qkd_key_response.serialize(crypto)?)
        .await?;

    // Store the private key for future communication with the peer
    quantum_connection.qkd_private_key = Some(private_key);

    Ok(())
}

async fn handle_qkd_key_response(
    quantum_connection: &mut QuantumResistantConnection,
    peer_address: &str,
    key: Vec<u8>,
    crypto: &QUPCrypto,
) -> Result<(), Box<dyn std::error::Error>> {
    // Decrypt the received public key using the local private key
    let decrypted_public_key = crypto.decrypt(&key, &quantum_connection.local_private_key)?;
    let peer_public_key: QKDPublicKey = bincode::deserialize(&decrypted_public_key)?;

    // Store the peer's public key for future communication
    quantum_connection.peer_public_key = Some(peer_public_key);

    // Send the QKD key confirmation message to the peer
    let qkd_key_confirmation = ProtocolMessage::QKDKeyConfirmation;
    quantum_connection
        .send_message(&peer_address, &qkd_key_confirmation.serialize(crypto)?)
        .await?;

    Ok(())
}

async fn handle_qkd_key_confirmation(
    quantum_connection: &mut QuantumResistantConnection,
    peer_address: &str,
    crypto: &QUPCrypto,
) -> Result<(), Box<dyn std::error::Error>> {
    // QKD key exchange completed successfully
    info!("QKD key exchange completed with peer: {}", peer_address);

    // Perform any additional actions or setup required for secure communication
    // using the established QKD keys
    // ...

    Ok(())
}

async fn handle_quantum_state_distribution(
    quantum_connection: &mut QuantumResistantConnection,
    peer_address: &str,
    state: QuantumState,
    crypto: &QUPCrypto,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize and encrypt the quantum state using the QKD keys
    let serialized_state = bincode::serialize(&state)?;
    let encrypted_state = crypto.encrypt(
        &serialized_state,
        &quantum_connection.qkd_private_key.as_ref().unwrap(),
    )?;

    // Send the encrypted quantum state to the peer
    let quantum_state_distribution = ProtocolMessage::QuantumStateDistribution {
        state: encrypted_state,
    };
    quantum_connection
        .send_message(
            &peer_address,
            &quantum_state_distribution.serialize(crypto)?,
        )
        .await?;

    Ok(())
}

async fn handle_quantum_state_measurement_results(
    quantum_connection: &mut QuantumResistantConnection,
    peer_address: &str,
    results: Vec<bool>,
    crypto: &QUPCrypto,
) -> Result<(), Box<dyn std::error::Error>> {
    // Deserialize and decrypt the measurement results using the QKD keys
    let serialized_results = bincode::serialize(&results)?;
    let decrypted_results = crypto.decrypt(
        &serialized_results,
        &quantum_connection.qkd_private_key.as_ref().unwrap(),
    )?;
    let measurement_results: Vec<bool> = bincode::deserialize(&decrypted_results)?;

    // Process the measurement results
    // ...

    Ok(())
}

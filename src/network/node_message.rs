use crate::chain::transaction::Transaction;
use crate::consensus::block::Block;
use crate::crypto::hash::Hash;
use crate::qup::crypto::{QUPCrypto, QUPSignature};
use crate::utils::node_id::NodeId;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc::Sender;
use zstd::stream::{decode_all, encode_all};

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeMessage {
    Ping,
    Pong,
    TransactionBroadcast(Transaction),
    BlockBroadcast(Block),
    BlockRequest(Hash),
    BlockResponse(Block),
    PeerRequest(NodeId),
    PeerResponse(Vec<NodeId>),
    StateRequest(NodeId),
    StateResponse(Vec<u8>),
    ErrorResponse(NodeMessageError),
    QuantumStateMessage(TimeBinState),
    MeasurementResultsMessage(Vec<bool>),
    QUPBlockProposal(QUPBlock),
    QUPBlockCommit(QUPBlock),
    QUPVote(QUPVote),
    QUPUsefulWork(QUPUsefulWork),
    QUPKeyExchange(QUPKeyExchange),
    QKDKeyRequest,
    QKDKeyResponse(QKDKey),
    QKDKeyConfirmation,
    QuantumStateDistribution(QuantumState),
    QuantumStateMeasurementResults(Vec<bool>),
    pq_tls_connection: Option<PostQuantumTLSConnection>,
}

#[derive(Debug, Error)]
pub enum NodeMessageError {
    #[error("Transaction processing failed: {0}")]
    TransactionProcessingFailed(String),
    #[error("Block processing failed: {0}")]
    BlockProcessingFailed(String),
    #[error("Block not found: {0}")]
    BlockNotFound(Hash),
    #[error("Peer retrieval failed: {0}")]
    PeerRetrievalFailed(String),
    #[error("Peer processing failed: {0}")]
    PeerProcessingFailed(String),
    #[error("State retrieval failed: {0}")]
    StateRetrievalFailed(String),
    #[error("State processing failed: {0}")]
    StateProcessingFailed(String),
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Quantum key distribution failed: {0}")]
    QKDFailed(String),
    #[error("Quantum state distribution failed: {0}")]
    QuantumStateDistributionFailed(String),
}

impl NodeMessage {
    pub fn serialize(&self, crypto: &QUPCrypto) -> Result<Vec<u8>, NodeMessageError> {
        let serialized_data = bincode::serialize(self)
            .map_err(|e| NodeMessageError::SerializationFailed(e.to_string()))?;
        let compressed_data = encode_all(&serialized_data[..], 3)
            .map_err(|e| NodeMessageError::CompressionFailed(e.to_string()))?;
        let encrypted_data = crypto.encrypt(&compressed_data)?;
        Ok(encrypted_data)
    }

    pub fn deserialize(data: &[u8], crypto: &QUPCrypto) -> Result<Self, NodeMessageError> {
        let decrypted_data = crypto.decrypt(data)?;
        let decompressed_data = decode_all(&decrypted_data)
            .map_err(|e| NodeMessageError::DecompressionFailed(e.to_string()))?;
        bincode::deserialize(&decompressed_data)
            .map_err(|e| NodeMessageError::DeserializationFailed(e.to_string()))
    }

    pub fn message_type(&self) -> &str {
        match self {
            NodeMessage::Ping => "ping",
            NodeMessage::Pong => "pong",
            NodeMessage::TransactionBroadcast(_) => "transaction_broadcast",
            NodeMessage::BlockBroadcast(_) => "block_broadcast",
            NodeMessage::BlockRequest(_) => "block_request",
            NodeMessage::BlockResponse(_) => "block_response",
            NodeMessage::PeerRequest(_) => "peer_request",
            NodeMessage::PeerResponse(_) => "peer_response",
            NodeMessage::StateRequest(_) => "state_request",
            NodeMessage::StateResponse(_) => "state_response",
            NodeMessage::ErrorResponse(_) => "error_response",
            NodeMessage::QuantumStateMessage(_) => "quantum_state_message",
            NodeMessage::MeasurementResultsMessage(_) => "measurement_results_message",
            NodeMessage::QUPBlockProposal(_) => "qup_block_proposal",
            NodeMessage::QUPBlockCommit(_) => "qup_block_commit",
            NodeMessage::QUPVote(_) => "qup_vote",
            NodeMessage::QUPUsefulWork(_) => "qup_useful_work",
            NodeMessage::QUPKeyExchange(_) => "qup_key_exchange",
            NodeMessage::QKDKeyRequest => "qkd_key_request",
            NodeMessage::QKDKeyResponse(_) => "qkd_key_response",
            NodeMessage::QKDKeyConfirmation => "qkd_key_confirmation",
            NodeMessage::QuantumStateDistribution(_) => "quantum_state_distribution",
            NodeMessage::QuantumStateMeasurementResults(_) => "quantum_state_measurement_results",
        }

        // Handle TLS-related messages
        match message {
            NodeMessage::TLSMessage(data) => {
                if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
                    pq_tls_connection.send(&data).await.expect("Failed to send data over TLS");
                } else {
                    error!("TLS connection not established");
                }
            }
            NodeMessage::TLSRequest => {
                if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
                    let data = pq_tls_connection.receive().await.expect("Failed to receive data over TLS");
                    self.send_message(NodeMessage::TLSResponse(data)).await;
                } else {
                    error!("TLS connection not established");
                }
            }
            _ => {
                // Handle other messages
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QUPBlock {
    pub header: QUPBlockHeader,
    pub transactions: Vec<QUPTransaction>,
    pub useful_work: QUPUsefulWork,
    pub signature: QUPSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QUPVote {
    pub block_hash: Hash,
    pub validator: Vec<u8>,
    pub signature: QUPSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QUPUsefulWork {
    pub problem: UsefulWorkProblem,
    pub solution: UsefulWorkSolution,
    pub proof: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QKDKey {
    pub key_type: QKDKeyType,
    pub key_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QKDKeyType {
    PublicKey,
    PrivateKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantumState {
    pub state_type: QuantumStateType,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuantumStateType {
    EntangledState,
    SingleQubitState,
}

#[derive(Debug)]
pub struct NodeMessageHandler {
    response_sender: Option<Sender<NodeMessage>>,
    crypto: QUPCrypto,
}

impl NodeMessageHandler {
    pub fn new(crypto: QUPCrypto) -> Self {
        let config = PostQuantumTLSConfig::new();
        let stream = TcpStream::connect("localhost:12345").await.expect("Failed to connect to server");
        let pq_tls_connection = PostQuantumTLSConnection::new(stream, config).await.expect("Failed to establish TLS connection");

        NodeMessageHandler {
            response_sender: None,
            crypto,
            pq_tls_connection: Some(pq_tls_connection),
        }
            response_sender: None,
            crypto,
        }
    }

    pub async fn handle_message(
        &mut self,
        sender: &NodeId,
        message: NodeMessage,
        response_sender: Option<Sender<NodeMessage>>,
    ) {
        self.response_sender = response_sender;

        match message {
            NodeMessage::Ping => self.handle_ping(sender).await,
            NodeMessage::Pong => self.handle_pong(sender).await,
            NodeMessage::TransactionBroadcast(tx) => {
                self.handle_transaction_broadcast(tx, sender).await
            }
            NodeMessage::BlockBroadcast(block) => self.handle_block_broadcast(block, sender).await,
            NodeMessage::BlockRequest(hash) => self.handle_block_request(hash, sender).await,
            NodeMessage::BlockResponse(block) => self.handle_block_response(block, sender).await,
            NodeMessage::PeerRequest(node_id) => self.handle_peer_request(node_id, sender).await,
            NodeMessage::PeerResponse(peers) => self.handle_peer_response(peers, sender).await,
            NodeMessage::StateRequest(node_id) => self.handle_state_request(node_id, sender).await,
            NodeMessage::StateResponse(state) => self.handle_state_response(state, sender).await,
            NodeMessage::ErrorResponse(error) => self.handle_error_response(error, sender).await,
            NodeMessage::QuantumStateMessage(state) => {
                self.handle_quantum_state_message(state, sender).await
            }
            NodeMessage::MeasurementResultsMessage(results) => {
                self.handle_measurement_results_message(results, sender)
                    .await
            }
            NodeMessage::QUPBlockProposal(block) => {
                self.handle_qup_block_proposal(block, sender).await
            }
            NodeMessage::QUPBlockCommit(block) => self.handle_qup_block_commit(block, sender).await,
            NodeMessage::QUPVote(vote) => self.handle_qup_vote(vote, sender).await,
            NodeMessage::QUPUsefulWork(useful_work) => {
                self.handle_qup_useful_work(useful_work, sender).await
            }
            NodeMessage::QUPKeyExchange(key_exchange) => {
                self.handle_qup_key_exchange(key_exchange, sender).await
            }
            NodeMessage::QKDKeyRequest => self.handle_qkd_key_request(sender).await,
            NodeMessage::QKDKeyResponse(key) => self.handle_qkd_key_response(key, sender).await,
            NodeMessage::QKDKeyConfirmation => self.handle_qkd_key_confirmation(sender).await,
            NodeMessage::QuantumStateDistribution(state) => {
                self.handle_quantum_state_distribution(state, sender).await
            }
            NodeMessage::QuantumStateMeasurementResults(results) => {
                self.handle_quantum_state_measurement_results(results, sender)
                    .await
            }
        }
    }

    async fn establish_tls_connection(&mut self, address: &str) -> Result<(), NodeMessageError> {
        let config = PostQuantumTLSConfig::new();
        let stream = TcpStream::connect(address).await.map_err(|e| {
            NodeMessageError::ConnectionError(format!("Failed to connect to {}: {}", address, e))
        })?;
        let pq_tls_connection = PostQuantumTLSConnection::new(stream, config).await.map_err(|e| {
            NodeMessageError::ConnectionError(format!("TLS connection establishment failed: {}", e))
        })?;
        self.pq_tls_connection = Some(pq_tls_connection);
        Ok(())
    }

    async fn send_tls(&mut self, data: &[u8]) -> Result<(), NodeMessageError> {
        if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
            pq_tls_connection.send(data).await.map_err(|e| {
                NodeMessageError::ConnectionError(format!("Failed to send data over TLS: {}", e))
            })
        } else {
            Err(NodeMessageError::ConnectionError("TLS connection not established".to_string()))
        }
    }

    async fn receive_tls(&mut self) -> Result<Vec<u8>, NodeMessageError> {
        if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
            pq_tls_connection.receive().await.map_err(|e| {
                NodeMessageError::ConnectionError(format!("Failed to receive data over TLS: {}", e))
            })
        } else {
            Err(NodeMessageError::ConnectionError("TLS connection not established".to_string()))
        }
    }

    async fn handle_ping(&mut self, sender: &NodeId) {
        info!("Received ping from {}", sender);
        self.send_message(NodeMessage::Pong).await;
    }

    async fn handle_pong(&mut self, sender: &NodeId) {
        info!("Received pong from {}", sender);
        // Update the last seen timestamp for the sender
        // You can add your custom logic here
    }

    async fn handle_transaction_broadcast(&mut self, tx: Transaction, sender: &NodeId) {
        info!("Received transaction broadcast from {}", sender);
        // Validate and process the received transaction
        match self.process_transaction(tx).await {
            Ok(_) => info!("Transaction processed successfully"),
            Err(e) => {
                error!("Failed to process transaction: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_block_broadcast(&mut self, block: Block, sender: &NodeId) {
        info!("Received block broadcast from {}", sender);
        // Validate and process the received block
        match self.process_block(block).await {
            Ok(_) => info!("Block processed successfully"),
            Err(e) => {
                error!("Failed to process block: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_block_request(&mut self, hash: Hash, sender: &NodeId) {
        info!("Received block request from {}", sender);
        // Retrieve the requested block by hash
        match self.get_block(hash).await {
            Ok(block) => self.send_message(NodeMessage::BlockResponse(block)).await,
            Err(e) => self.send_error_response(e).await,
        }
    }

    async fn handle_block_response(&mut self, block: Block, sender: &NodeId) {
        info!("Received block response from {}", sender);
        // Process the received block
        match self.process_block(block).await {
            Ok(_) => info!("Block processed successfully"),
            Err(e) => {
                error!("Failed to process block: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_peer_request(&mut self, node_id: NodeId, sender: &NodeId) {
        info!("Received peer request from {}", sender);
        // Retrieve the requested peer information
        match self.get_peers(node_id).await {
            Ok(peers) => self.send_message(NodeMessage::PeerResponse(peers)).await,
            Err(e) => self.send_error_response(e).await,
        }
    }

    async fn handle_peer_response(&mut self, peers: Vec<NodeId>, sender: &NodeId) {
        info!("Received peer response from {}", sender);
        // Process the received peer information
        match self.process_peers(peers).await {
            Ok(_) => info!("Peers processed successfully"),
            Err(e) => {
                error!("Failed to process peers: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_state_request(&mut self, node_id: NodeId, sender: &NodeId) {
        info!("Received state request from {}", sender);
        // Retrieve the requested state information
        match self.get_state(node_id).await {
            Ok(state) => self.send_message(NodeMessage::StateResponse(state)).await,
            Err(e) => self.send_error_response(e).await,
        }
    }

    async fn handle_state_response(&mut self, state: Vec<u8>, sender: &NodeId) {
        info!("Received state response from {}", sender);
        // Process the received state information
        match self.process_state(state).await {
            Ok(_) => info!("State processed successfully"),
            Err(e) => {
                error!("Failed to process state: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_error_response(&mut self, error: NodeMessageError, sender: &NodeId) {
        error!("Received error response from {}: {}", sender, error);
        // Handle the received error
        match error {
            NodeMessageError::BlockNotFound(_) => {
                info!("Attempting to request the block from another peer.");
                // Here you would implement logic to request the block from another peer
            }
            NodeMessageError::PeerRetrievalFailed(_) => {
                info!("Attempting to refresh peer list.");
                // Here you would implement logic to refresh the list of peers, perhaps from a known good source
            }
            NodeMessageError::StateRetrievalFailed(_) => {
                info!("Attempting to retrieve state from a different node.");
                // Here you would implement logic to try retrieving the state from a different node
            }
            _ => {
                warn!("No recovery action defined for this error.");
                // For other errors, you might not have a specific recovery action
            }
        }
    }

    async fn handle_quantum_state_message(&mut self, state: TimeBinState, sender: &NodeId) {
        info!("Received quantum state message from {}", sender);
        // Process the received quantum state
        match self.process_quantum_state(state).await {
            Ok(_) => info!("Quantum state processed successfully"),
            Err(e) => {
                error!("Failed to process quantum state: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn handle_measurement_results_message(&mut self, results: Vec<bool>, sender: &NodeId) {
        info!("Received measurement results message from {}", sender);
        // Process the received measurement results
        match self.process_measurement_results(results).await {
            Ok(_) => info!("Measurement results processed successfully"),
            Err(e) => {
                error!("Failed to process measurement results: {}", e);
                self.send_error_response(e).await;
            }
        }
    }

    async fn send_message(&mut self, message: NodeMessage) {
        if let Some(sender) = &mut self.response_sender {
            if let Err(e) = sender.send(message).await {
                error!("Failed to send response message: {}", e);
            }
        }
    }

    async fn send_error_response(&mut self, error: NodeMessageError) {
        self.send_message(NodeMessage::ErrorResponse(error)).await;
    }

    async fn process_transaction(&mut self, tx: Transaction) -> Result<(), NodeMessageError> {
        // Validate the transaction
        if !self.validate_transaction(&tx) {
            return Err(NodeMessageError::TransactionProcessingFailed(
                "Invalid transaction".to_string(),
            ));
        }

        // Add the transaction to the transaction pool or process it according to the blockchain's rules
        match self.add_transaction_to_pool(tx).await {
            Ok(_) => Ok(()),
            Err(e) => Err(NodeMessageError::TransactionProcessingFailed(e.to_string())),
        }
    }

    async fn process_block(&mut self, block: Block) -> Result<(), NodeMessageError> {
        // Validate the block
        if !self.validate_block(&block) {
            return Err(NodeMessageError::BlockProcessingFailed(
                "Invalid block".to_string(),
            ));
        }

        // Add the block to the blockchain and update the state
        match self.add_block_to_chain(block).await {
            Ok(_) => Ok(()),
            Err(e) => Err(NodeMessageError::BlockProcessingFailed(e.to_string())),
        }
    }

    async fn get_block(&mut self, hash: Hash) -> Result<Block, NodeMessageError> {
        // Retrieve the block by hash from the local blockchain storage
        match self.retrieve_block_by_hash(hash).await {
            Ok(block) => Ok(block),
            Err(_) => Err(NodeMessageError::BlockNotFound(hash)),
        }
    }

    async fn get_peers(&mut self, node_id: NodeId) -> Result<Vec<NodeId>, NodeMessageError> {
        // Retrieve the peer information for the given node ID
        match self.retrieve_peer_info(node_id).await {
            Ok(peers) => Ok(peers),
            Err(e) => Err(NodeMessageError::PeerRetrievalFailed(e.to_string())),
        }
    }

    async fn process_peers(&mut self, peers: Vec<NodeId>) -> Result<(), NodeMessageError> {
        // Process the received peer information and update the local peer database
        match self.update_peer_database(peers).await {
            Ok(_) => Ok(()),
            Err(e) => Err(NodeMessageError::PeerProcessingFailed(e.to_string())),
        }
    }

    async fn get_state(&mut self, node_id: NodeId) -> Result<Vec<u8>, NodeMessageError> {
        // Retrieve the state information for the given node ID
        match self.retrieve_state_info(node_id).await {
            Ok(state) => Ok(state),
            Err(e) => Err(NodeMessageError::StateRetrievalFailed(e.to_string())),
        }
    }

    async fn process_state(&mut self, state: Vec<u8>) -> Result<(), NodeMessageError> {
        // Process the received state information and update the local state storage
        match self.update_local_state(state).await {
            Ok(_) => Ok(()),
            Err(e) => Err(NodeMessageError::StateProcessingFailed(e.to_string())),
        }
    }

    async fn handle_qup_block_proposal(&mut self, block: QUPBlock, sender: &NodeId) {
        // Verify the block proposal signature
        if let Err(e) = self.crypto.verify_block_proposal_signature(
            &block.header,
            &block.transactions,
            &block.useful_work,
            &block.signature,
        ) {
            error!("QUP block proposal signature verification failed: {}", e);
            return;
        }
    
        // Process the block proposal
        match self.consensus.process_block_proposal(block.clone()).await {
            Ok(_) => {
                info!("QUP block proposal processed successfully");
                // Relay the block proposal to other peers
                if let Err(e) = self.relay_qup_message(NodeMessage::QUPBlockProposal(block)).await {
                    error!("Failed to relay QUP block proposal: {}", e);
                }
            }
            Err(e) => {
                error!("Processing QUP block proposal failed: {}", e);
            }
        }
    }

    async fn handle_qup_block_commit(&mut self, block: QUPBlock, sender: &NodeId) {
        // Verify the block commit signature
        if let Err(e) = self.crypto.verify_block_commit_signature(&block.header, &block.signature) {
            error!("QUP block commit signature verification failed: {}", e);
            return;
        }
    
        // Commit the block
        match self.consensus.commit_block(block.clone()).await {
            Ok(_) => {
                info!("QUP block committed successfully");
                // Relay the block commit message to other peers
                if let Err(e) = self.relay_qup_message(NodeMessage::QUPBlockCommit(block)).await {
                    error!("Failed to relay QUP block commit: {}", e);
                }
            }
            Err(e) => {
                error!("Committing QUP block failed: {}", e);
            }
        }
    }

    async fn handle_qup_vote(&mut self, vote: QUPVote, sender: &NodeId) {
        // Verify the vote signature
        if let Err(e) = self.crypto.verify_vote_signature(&vote) {
            error!("QUP vote signature verification failed: {}", e);
            return;
        }
    
        // Process the vote
        match self.consensus.process_vote(vote.clone()).await {
            Ok(_) => {
                info!("QUP vote processed successfully");
                // Relay the vote message to other peers
                if let Err(e) = self.relay_qup_message(NodeMessage::QUPVote(vote)).await {
                    error!("Failed to relay QUP vote: {}", e);
                }
            }
            Err(e) => {
                error!("Processing QUP vote failed: {}", e);
            }
        }
    }
    
    async fn handle_qup_useful_work(&mut self, useful_work: QUPUsefulWork, sender: &NodeId) {
        // Verify the useful work
        if let Err(e) = self.crypto.verify_useful_work(&useful_work) {
            error!("QUP useful work verification failed: {}", e);
            return;
        }
    
        // Process the useful work
        match self.consensus.process_useful_work(useful_work.clone()).await {
            Ok(_) => {
                info!("QUP useful work processed successfully");
                // Relay the useful work message to other peers
                if let Err(e) = self.relay_qup_message(NodeMessage::QUPUsefulWork(useful_work)).await {
                    error!("Failed to relay QUP useful work: {}", e);
                }
            }
            Err(e) => {
                error!("Processing QUP useful work failed: {}", e);
            }
        }
    }

    async fn handle_qup_key_exchange(&mut self, key_exchange: QUPKeyExchange, sender: &NodeId) {
        // Process the key exchange message
        match self.crypto.process_key_exchange(key_exchange) {
            Ok(_) => {
                info!("QUP key exchange processed successfully");
                // Update the local key material based on the key exchange
                // ...
            }
            Err(e) => {
                error!("Processing QUP key exchange failed: {}", e);
            }
        }
    }
    
    async fn handle_qkd_key_request(&mut self, sender: &NodeId) {
        // Generate a new QKD key pair
        let (private_key, public_key) = self.crypto.generate_qkd_key_pair();
    
        // Send the QKD public key to the requesting peer
        let qkd_key_response = NodeMessage::QKDKeyResponse(public_key);
        if let Err(e) = self.send_message(sender, qkd_key_response).await {
            error!("Failed to send QKD key response to {}: {}", sender, e);
        }
    
        // Store the QKD private key locally for future communication with the peer
        self.qkd_private_keys.insert(sender.clone(), private_key);
    }
    
    async fn handle_qkd_key_response(&mut self, key: QKDKey, sender: &NodeId) {
        // Decrypt the received QKD public key using the local private key
        let decrypted_public_key = self.crypto.decrypt_with_private_key(&key);
    
        // Store the peer's QKD public key for future communication
        self.qkd_public_keys.insert(sender.clone(), decrypted_public_key);
    
        // Send a QKDKeyConfirmation message to the peer
        let qkd_key_confirmation = NodeMessage::QKDKeyConfirmation;
        if let Err(e) = self.send_message(sender, qkd_key_confirmation).await {
            error!("Failed to send QKD key confirmation to {}: {}", sender, e);
        }
    }
    
    async fn handle_qkd_key_confirmation(&mut self, sender: &NodeId) {
        // Mark the QKD key exchange as completed for the peer
        self.qkd_completed_peers.insert(sender.clone());
    
        // Perform any additional setup required for secure communication using the established QKD keys
        match self.establish_secure_communication(sender).await {
            Ok(_) => {
                info!("Secure communication established with {}", sender);
            }
            Err(e) => {
                error!("Failed to establish secure communication with {}: {}", sender, e);
            }
        }
    }
    
    async fn handle_quantum_state_distribution(&mut self, state: QuantumState, sender: &NodeId) {
        // Retrieve the QKD keys for the sender
        let private_key = self.qkd_private_keys.get(sender).unwrap();
        let public_key = self.qkd_public_keys.get(sender).unwrap();
    
        // Decrypt and deserialize the quantum state using the QKD keys
        let serialized_state = self.crypto.decrypt(&state.data, private_key).unwrap();
        let deserialized_state: QuantumState = bincode::deserialize(&serialized_state).unwrap();
    
        // Process the quantum state
        match self.process_quantum_state(deserialized_state).await {
            Ok(_) => {
                info!("Quantum state processed successfully");
            }
            Err(e) => {
                error!("Processing quantum state failed: {}", e);
            }
        }
    }
    
    async fn handle_quantum_state_measurement_results(
        &mut self,
        results: Vec<bool>,
        sender: &NodeId,
    ) {
        // Retrieve the QKD keys for the sender
        let private_key = self.qkd_private_keys.get(sender).unwrap();
        let public_key = self.qkd_public_keys.get(sender).unwrap();
    
        // Decrypt and deserialize the measurement results using the QKD keys
        let serialized_results = self.crypto.decrypt(&results, private_key).unwrap();
        let deserialized_results: Vec<bool> = bincode::deserialize(&serialized_results).unwrap();
    
        // Process the measurement results
        match self.process_measurement_results(deserialized_results).await {
            Ok(_) => {
                info!("Measurement results processed successfully");
            }
            Err(e) => {
                error!("Processing measurement results failed: {}", e);
            }
        }
    }
}

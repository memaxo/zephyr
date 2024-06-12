use crate::quantum_voting::quantum_state::QuantumState;
use crate::qup::crypto::{QUPCrypto, QUPSignature};
use libp2p::core::upgrade;
use libp2p::request_response::{
    ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
    RequestResponseEvent, RequestResponseMessage,
};
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use zstd::stream::{decode_all, encode_all};
use zstd::stream::{decode_all, encode_all};

pub const PROTOCOL_VERSION: u32 = 2;
// Quantum-resistant protocol version
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB
pub const MAX_INFLIGHT_MESSAGES: usize = 100; 
pub const FLOW_CONTROL_WINDOW: usize = 10;
pub const PING_INTERVAL: u64 = 60;
// 60 seconds
pub const PONG_TIMEOUT: u64 = 30;
// 30 seconds
pub const HANDSHAKE_TIMEOUT: u64 = 10; // 10 seconds

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMessage {
    // Existing message types...

    // New flow control message types
    FlowControlCredit { 
        credit: usize 
    },
    FlowControlAck,
    // Existing message types...

    // New flow control message types
    FlowControlCredit { 
        credit: usize 
    },
    FlowControlAck,
    TLSHandshake {
        version: u32,
        peer_id: String,
    },
    TLSKeyExchange {
        public_key: Vec<u8>,
    },
    TLSEncryptedMessage {
        data: Vec<u8>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrioritizedMessage {
    pub priority: u8,
    pub message: ProtocolMessage,
}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

pub struct MessageQueue {
    outbound: Arc<Mutex<PriorityQueue<PrioritizedMessage>>>,
    inbound: Arc<Mutex<PriorityQueue<PrioritizedMessage>>>,
    outbound_credit: usize,
    inbound_window: usize,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            outbound: Arc::new(Mutex::new(PriorityQueue::new())),
            inbound: Arc::new(Mutex::new(PriorityQueue::new())),
            outbound_credit: MAX_INFLIGHT_MESSAGES,
            inbound_window: FLOW_CONTROL_WINDOW,
        }
    }

    pub async fn enqueue_outbound(&mut self, message: ProtocolMessage, priority: u8) {
        if self.outbound_credit > 0 {
            self.outbound.lock().await.push(PrioritizedMessage { priority, message });
            self.outbound_credit -= 1;
        }
    }

    pub async fn dequeue_outbound(&mut self) -> Option<ProtocolMessage> {
        self.outbound.lock().await.pop().map(|p| p.message)
    }

    pub async fn enqueue_inbound(&mut self, message: ProtocolMessage, priority: u8) {
        self.inbound.lock().await.push(PrioritizedMessage { priority, message });
        if self.inbound.lock().await.len() >= self.inbound_window {
            self.send_flow_control().await;
        }
    }

    pub async fn dequeue_inbound(&mut self) -> Option<ProtocolMessage> {
        self.inbound.lock().await.pop().map(|p| p.message)  
    }

    async fn send_flow_control(&mut self) {
        let credit = self.inbound_window - self.inbound.lock().await.len();
        if credit > 0 {
            self.enqueue_outbound(ProtocolMessage::FlowControlCredit { credit }, 0).await;
        }
    }

    pub fn receive_flow_control(&mut self, credit: usize) {
        self.outbound_credit += credit;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMessage {
    Ping,
    Pong,
    Handshake {
        version: u32,
        peer_id: String,
        peer_address: String,
    },
    BlockRequest {
        block_hash: String,
    },
    BlockResponse {
        block: Vec<u8>,
        signature: QUPSignature,
    },
    TransactionBroadcast {
        transaction: Vec<u8>,
        signature: QUPSignature,
    },
    PeerDiscovery {
        peer_addresses: Vec<String>,
    },
    StateSyncRequest {
        shard_id: u64,
    },
    StateSyncResponse {
        shard_id: u64,
        state: Vec<u8>,
    },
    QKDKeyRequest,
    QKDKeyResponse {
        key: QKDKey,
    },
    QKDKeyConfirmation,
    QuantumStateDistribution {
        state: QuantumState,
    },
    QuantumStateMeasurementResults {
        results: Vec<bool>,
    },
    ClassicalKeyExchange {
        public_key: Vec<u8>,
    },
    ClassicalKeyExchangeResponse {
        public_key: Vec<u8>,
    },
    QuantumKeyExchange {
        public_key: Vec<u8>,
    },
    QuantumKeyExchangeResponse {
        public_key: Vec<u8>,
    },
    QUPMessage {
        message: Vec<u8>,
        signature: QUPSignature,
    },
    UsefulWorkProblem {
        problem: Vec<u8>,
    },
    UsefulWorkSolution {
        solution: Vec<u8>,
        signature: QUPSignature,
    },
    BlockProposal {
        block: Vec<u8>,
        signature: QUPSignature,
    },
    Vote {
        vote: Vec<u8>,
        signature: QUPSignature,
    },
    BlockCommit {
        block: Vec<u8>,
        signature: QUPSignature,
    },
}
    Ping,
    Pong,
    Handshake {
        version: u32,
        peer_id: String,
        peer_address: String,
    },
    BlockRequest {
        block_hash: String,
    },
    BlockResponse {
        block: Vec<u8>,
        signature: QUPSignature,
    },
    TransactionBroadcast {
        transaction: Vec<u8>,
        signature: QUPSignature,
    },
    PeerDiscovery {
        peer_addresses: Vec<String>,
    },
    StateSyncRequest {
        shard_id: u64,
    },
    StateSyncResponse {
        shard_id: u64,
        state: Vec<u8>,
    },
    QKDKeyRequest,
    QKDKeyResponse {
        key: QKDKey,
    },
        key: QKDKey,
    },
    QKDKeyConfirmation,
    QuantumStateDistribution {
        state: QuantumState,
    },
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QKDKey {
    // Fields for QKD key information
}

impl ProtocolMessage {
    pub fn serialize(&self, crypto: &QUPCrypto) -> Result<Vec<u8>, ProtocolError> {
        let mut message = Builder::new_default();
        {
            let root = message.init_root::<protocol_message::Builder>();
            match self {
                ProtocolMessage::Ping => {
                    root.set_ping(());
                }
                ProtocolMessage::Pong => {
                    root.set_pong(());
                }
                ProtocolMessage::BlockProposal { block, signature } => {
                    let mut block_proposal = root.init_block_proposal();
                    block_proposal.set_block(block);
                    block_proposal.set_signature(signature);
                }
                ProtocolMessage::Vote { vote, signature } => {
                    let mut vote_msg = root.init_vote();
                    vote_msg.set_vote(vote);
                    vote_msg.set_signature(signature);
                }
                ProtocolMessage::BlockCommit { block, signature } => {
                    let mut block_commit = root.init_block_commit();
                    block_commit.set_block(block);
                    block_commit.set_signature(signature);
                }
                ProtocolMessage::TLSHandshake { version, peer_id } => {
                    let mut tls_handshake = root.init_tls_handshake();
                    tls_handshake.set_version(*version);
                    tls_handshake.set_peer_id(peer_id);
                }
                ProtocolMessage::TLSKeyExchange { public_key } => {
                    let mut tls_key_exchange = root.init_tls_key_exchange();
                    tls_key_exchange.set_public_key(public_key);
                }
                ProtocolMessage::TLSEncryptedMessage { data } => {
                    let mut tls_encrypted_message = root.init_tls_encrypted_message();
                    tls_encrypted_message.set_data(data);
                }
            }
        }
        let serialized_data = rmps::to_vec_named(self)
            .map_err(|e| ProtocolError::SerializationFailed(e.to_string()))?;
        let compressed_data = encode_all(&serialized_data[..], 1)  
            .map_err(|e| ProtocolError::CompressionFailed(e.to_string()))?;
        crypto.encrypt(&compressed_data)
            .map_err(|e| ProtocolError::EncryptionFailed(e.to_string()))
    }

    pub fn deserialize(data: &[u8], crypto: &QUPCrypto) -> Result<Self, ProtocolError> {
        let decrypted_data = crypto.decrypt(data)
            .map_err(|e| ProtocolError::DecryptionFailed(e.to_string()))?;
        let decompressed_data = decode_all(&decrypted_data).map_err(|e| ProtocolError::DecompressionFailed(e.to_string()))?;
        rmps::from_slice(&decompressed_data).map_err(|e| ProtocolError::DeserializationFailed(e.to_string()))
    }
}

impl ProtocolMessage {
    pub fn serialize(&self, crypto: &QUPCrypto) -> Result<Vec<u8>, ProtocolError> {
        let mut message = Builder::new_default();
        {
            let root = message.init_root::<protocol_message::Builder>();
            match self {
                ProtocolMessage::Ping => {
                    root.set_ping(());
                }
                ProtocolMessage::Pong => {
                    root.set_pong(());
                }
                ProtocolMessage::BlockProposal { block, signature } => {
                    let mut block_proposal = root.init_block_proposal();
                    block_proposal.set_block(block);
                    block_proposal.set_signature(signature);
                }
                ProtocolMessage::Vote { vote, signature } => {
                    let mut vote_msg = root.init_vote();
                    vote_msg.set_vote(vote);
                    vote_msg.set_signature(signature);
                }
                ProtocolMessage::BlockCommit { block, signature } => {
                    let mut block_commit = root.init_block_commit();
                    block_commit.set_block(block);
                    block_commit.set_signature(signature);
                }
                ProtocolMessage::TLSHandshake { version, peer_id } => {
                    let mut tls_handshake = root.init_tls_handshake();
                    tls_handshake.set_version(*version);
                    tls_handshake.set_peer_id(peer_id);
                }
                ProtocolMessage::TLSKeyExchange { public_key } => {
                    let mut tls_key_exchange = root.init_tls_key_exchange();
                    tls_key_exchange.set_public_key(public_key);
                }
                ProtocolMessage::TLSEncryptedMessage { data } => {
                    let mut tls_encrypted_message = root.init_tls_encrypted_message();
                    tls_encrypted_message.set_data(data);
                }
                ProtocolMessage::TLSHandshake { version, peer_id } => {
                    let mut tls_handshake = root.init_tls_handshake();
                    tls_handshake.set_version(*version);
                    tls_handshake.set_peer_id(peer_id);
                }
                ProtocolMessage::TLSKeyExchange { public_key } => {
                    let mut tls_key_exchange = root.init_tls_key_exchange();
                    tls_key_exchange.set_public_key(public_key);
                }
                ProtocolMessage::TLSEncryptedMessage { data } => {
                    let mut tls_encrypted_message = root.init_tls_encrypted_message();
                    tls_encrypted_message.set_data(data);
                }
            }
        }
        let serialized_data = rmps::to_vec_named(self)
            .map_err(|e| ProtocolError::SerializationFailed(e.to_string()))?;
        let compressed_data = encode_all(&serialized_data[..], 1)  
            .map_err(|e| ProtocolError::CompressionFailed(e.to_string()))?;
        let compressed_data = encode_all(&serialized_data[..], 1)
            .map_err(|e| ProtocolError::CompressionFailed(e.to_string()))?;
        crypto.encrypt(&compressed_data)
            .map_err(|e| ProtocolError::EncryptionFailed(e.to_string()))
    }

    pub fn deserialize(data: &[u8], crypto: &QUPCrypto) -> Result<Self, ProtocolError> {
        let decrypted_data = crypto.decrypt(data)
            .map_err(|e| ProtocolError::DecryptionFailed(e.to_string()))?;
        let decompressed_data = decode_all(&decrypted_data).map_err(|e| ProtocolError::DecompressionFailed(e.to_string()))?;
        rmps::from_slice(&decompressed_data).map_err(|e| ProtocolError::DeserializationFailed(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]  
    DecompressionFailed(String),
    #[error("Quantum key distribution failed: {0}")]
    QKDFailed(String),
    #[error("Quantum state distribution failed: {0}")]
    QuantumStateDistributionFailed(String),
}

// Cap'n Proto schema definition
mod protocol_message {
    #[derive(capnp::Serialize, capnp::Deserialize)]
    pub struct ProtocolMessage {
        pub ping: (),
        pub pong: (),
        // Fields for other message types
        ProtocolMessage::TLSHandshake { version, peer_id } => {
            let mut tls_handshake = root.init_tls_handshake();
            tls_handshake.set_version(*version);
            tls_handshake.set_peer_id(peer_id);
        }
        ProtocolMessage::TLSKeyExchange { public_key } => {
            let mut tls_key_exchange = root.init_tls_key_exchange();
            tls_key_exchange.set_public_key(public_key);
        }
        ProtocolMessage::TLSEncryptedMessage { data } => {
            let mut tls_encrypted_message = root.init_tls_encrypted_message();
            tls_encrypted_message.set_data(data);
        }
    }
}
    BlockProposal {
        block: Vec<u8>,
        signature: QUPSignature,
    },
    Vote {
        vote: Vec<u8>,
        signature: QUPSignature,
    },
    BlockCommit {
        block: Vec<u8>,
        signature: QUPSignature,
    },
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct PriorityQueue<T> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T> PriorityQueue<T> {
    fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}
pub struct MessageQueue {
    outbound: Arc<Mutex<PriorityQueue<PrioritizedMessage>>>,
    inbound: Arc<Mutex<PriorityQueue<PrioritizedMessage>>>,
    outbound_credit: usize,
    inbound_window: usize,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            outbound: Arc::new(Mutex::new(PriorityQueue::new())),
            inbound: Arc::new(Mutex::new(PriorityQueue::new())),
            outbound_credit: MAX_INFLIGHT_MESSAGES,
            inbound_window: FLOW_CONTROL_WINDOW,
        }
    }

    pub async fn enqueue_outbound(&mut self, message: ProtocolMessage, priority: u8) {
        if self.outbound_credit > 0 {
            self.outbound.lock().await.push(PrioritizedMessage { priority, message });
            self.outbound_credit -= 1;
        }
    }

    pub async fn dequeue_outbound(&mut self) -> Option<ProtocolMessage> {
        self.outbound.lock().await.pop().map(|p| p.message)
    }

    pub async fn enqueue_inbound(&mut self, message: ProtocolMessage, priority: u8) {
        self.inbound.lock().await.push(PrioritizedMessage { priority, message });
        if self.inbound.lock().await.len() >= self.inbound_window {
            self.send_flow_control().await;
        }
    }

    pub async fn dequeue_inbound(&mut self) -> Option<ProtocolMessage> {
        self.inbound.lock().await.pop().map(|p| p.message)  
    }

    async fn send_flow_control(&mut self) {
        let credit = self.inbound_window - self.inbound.lock().await.len();
        if credit > 0 {
            self.enqueue_outbound(ProtocolMessage::FlowControlCredit { credit }, 0).await;
        }
    }

    pub fn receive_flow_control(&mut self, credit: usize) {
        self.outbound_credit += credit;
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct PrioritizedMessage {
    pub priority: u8,
    pub message: ProtocolMessage,
}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

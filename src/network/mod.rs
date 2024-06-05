pub mod sync;
pub mod tls;
mod handler;
mod protocol;
mod server;
mod client;
mod discovery;
mod config;
mod error;
mod lib;
mod network_manager;
mod quantum_resistant;
mod node_message;
mod shard_message;

use crate::chain::block::Block;
use crate::chain::consensus::{Consensus, ConsensusType};
use crate::chain::transaction::Transaction;
use crate::config::NetworkConfig;
use crate::crypto::hash::Hasher;
use crate::network::p2p::{Message, Peer, PeerError};
use crate::network::sync::{BlockSyncState, StateSync};
use crate::network::tls::{TLSConnectionError, TLSListener};
use crate::utils::node_id::NodeId;
use crate::error_handling::mod::NetworkError;
use log::{debug, error, info, warn};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Network {
    peers: RwLock<HashMap<NodeId, Peer>>,
    consensus: Arc<dyn Consensus>,
    state_sync: StateSync,
    tx_pool: RwLock<HashMap<Hasher, Transaction>>,
    block_sync_state: RwLock<BlockSyncState>,
    allowed_peers: HashSet<NodeId>,
    max_peers: usize,
    max_inbound_peers: usize,
    peer_channel: (Sender<Message>, Receiver<Message>),
    network_error_handler: Arc<dyn NetworkErrorHandler>,
}

impl Network {
    pub fn new(config: &NetworkConfig, consensus: Arc<dyn Consensus>, network_error_handler: Arc<dyn NetworkErrorHandler>) -> Self {
        let (peer_tx, peer_rx) = channel(config.peer_channel_capacity);
        Network {
            peers: RwLock::new(HashMap::new()),
            consensus,
            state_sync: StateSync::new(),
            tx_pool: RwLock::new(HashMap::new()),
            block_sync_state: RwLock::new(BlockSyncState::default()),
            allowed_peers: config.allowed_peers.iter().map(|id| NodeId::from_str(id).unwrap()).collect(),
            max_peers: config.max_peers,
            max_inbound_peers: config.max_inbound_peers,
            peer_channel: (peer_tx, peer_rx),
            network_error_handler,
        }
    }
    pub fn new(config: &NetworkConfig, consensus: Arc<dyn Consensus>) -> Self {
        let (peer_tx, peer_rx) = channel(config.peer_channel_capacity);
        Network {
            peers: RwLock::new(HashMap::new()),
            consensus,
            state_sync: StateSync::new(),
            tx_pool: RwLock::new(HashMap::new()),
            block_sync_state: RwLock::new(BlockSyncState::default()),
            allowed_peers: config.allowed_peers.clone(),
            max_peers: config.max_peers,
            max_inbound_peers: config.max_inbound_peers,
            peer_channel: (peer_tx, peer_rx),
        }
    }

    pub async fn start(&self, address: &str) {
        let listener = TLSListener::bind(address).await.expect("Failed to bind to address");
        info!("Network listening on {}", address);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let peer_addr = stream.peer_addr().unwrap().to_string();
                    if self.is_allowed(&peer_addr) && self.can_accept_inbound_peer() {
                        let peer = Peer::new(stream);
                        self.add_peer(peer).await;
                    } else {
                        debug!("Rejected inbound connection from {}", peer_addr);
                    }
                }
                Err(e) => {
                    error!("Error accepting incoming connection: {}", e);
                }
            }
        }
    }

    pub async fn connect(&self, address: &str) -> Result<(), PeerError> {
        let stream = TLSListener::connect(address).await?;
        let peer = Peer::new(stream);
        self.add_peer(peer).await;
        Ok(())
    }

    async fn add_peer(&self, peer: Peer) {
        let peer_addr = peer.address.clone();
        self.peers.write().unwrap().insert(peer_addr.clone(), peer);
        info!("Connected to peer: {}", peer_addr);
    }

    fn is_allowed(&self, peer_addr: &str) -> bool {
        self.allowed_peers.contains(peer_addr)
    }

    fn can_accept_inbound_peer(&self) -> bool {
        self.peers.read().unwrap().len() < self.max_inbound_peers
    }

    pub async fn broadcast(&self, message: Message) {
        for peer in self.peers.read().unwrap().values() {
            if let Err(e) = peer.send(message.clone()).await {
                error!("Error broadcasting message to {}: {}", peer.address, e);
            }
        }
    }

    pub async fn handle_message(&self, peer_addr: &str, message: Message) {
        match message {
            Message::Transaction(tx) => self.handle_transaction(peer_addr, tx).await,
            Message::Block(block) => self.handle_block(peer_addr, block).await,
            Message::GetBlocks(hash) => self.handle_get_blocks(peer_addr, hash).await,
            Message::Blocks(blocks) => self.handle_blocks(peer_addr, blocks).await,
            Message::GetState(height) => self.handle_get_state(peer_addr, height).await,
            Message::State(state) => self.handle_state(peer_addr, state).await,
            Message::Ping => self.handle_ping(peer_addr).await,
            Message::Pong => self.handle_pong(peer_addr).await,
        }
    }

    async fn handle_transaction(&self, peer_addr: &str, tx: Transaction) {
        let tx_hash = tx.hash();
        if !self.tx_pool.read().unwrap().contains_key(&tx_hash) {
            self.tx_pool.write().unwrap().insert(tx_hash, tx.clone());
            self.broadcast(Message::Transaction(tx)).await;
        }
    }

    async fn handle_block(&self, peer_addr: &str, block: Block) {
        if self.consensus.validate_block(&block) {
            self.state_sync.process_block(block.clone()).await;
            self.broadcast(Message::Block(block)).await;
        } else {
            warn!("Received invalid block from {}", peer_addr);
        }
    }

    async fn handle_get_blocks(&self, peer_addr: &str, hash: Hasher) {
        // Retrieve blocks starting from the given hash and send them to the peer
    }

    async fn handle_blocks(&self, peer_addr: &str, blocks: Vec<Block>) {
        // Process received blocks and update the block sync state
    }

    async fn handle_get_state(&self, peer_addr: &str, height: u64) {
        // Retrieve the state at the given height and send it to the peer
    }

    async fn handle_state(&self, peer_addr: &str, state: Vec<u8>) {
        // Process the received state and update the local state
    }

    async fn handle_ping(&self, peer_addr: &str) {
        // Respond with a pong message
    }

    async fn handle_pong(&self, peer_addr: &str) {
        // Update the last seen timestamp for the peer
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("TLS connection error: {0}")]
    TLSConnectionError(#[from] TLSConnectionError),
    #[error("Peer error: {0}")]
    PeerError(#[from] PeerError),
}

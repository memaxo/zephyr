use crate::network::p2p::message::Message;
use crate::network::p2p::peer::Peer;
use crate::network::Network;
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Gossip {
    network: Arc<Network>,
}

impl Gossip {
    pub fn new(network: Arc<Network>) -> Self {
        Gossip { network }
    }

    pub async fn propagate_transaction(&self, transaction: Message) {
        let peers = self.network.peers.read().await;
        for peer in peers.values() {
            if let Err(e) = peer.send(transaction.clone()).await {
                error!("Failed to propagate transaction to {}: {}", peer.address, e);
            }
        }
    }

    pub async fn receive_gossip(&self, peer: &Peer, message: Message) {
        match message {
            Message::Transaction(transaction) => {
                self.handle_gossip_message(peer, transaction).await;
            }
            _ => {
                debug!("Received non-transaction gossip message from {}", peer.address);
            }
        }
    }

    async fn handle_gossip_message(&self, peer: &Peer, transaction: Message) {
        info!("Handling gossip message from {}", peer.address);
        // Process the transaction
        // Add your transaction processing logic here

        // Propagate the transaction to other peers
        self.propagate_transaction(transaction).await;
    }
}

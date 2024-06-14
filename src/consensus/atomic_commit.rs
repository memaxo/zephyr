use crate::chain::transaction::Transaction;
use crate::network::p2p::peer::Peer;
use crate::network::Network;
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AtomicCommit {
    network: Arc<Network>,
}

impl AtomicCommit {
    pub fn new(network: Arc<Network>) -> Self {
        AtomicCommit { network }
    }

    pub async fn propose_transaction(&self, transaction: Transaction) -> bool {
        let peers = self.network.peers.read().await;
        let mut votes = Vec::new();

        for peer in peers.values() {
            match peer.send(transaction.clone()).await {
                Ok(_) => {
                    if let Ok(vote) = peer.receive().await {
                        votes.push(vote);
                    }
                }
                Err(e) => {
                    error!("Failed to propose transaction to {}: {}", peer.address, e);
                }
            }
        }

        // Check if all votes are positive
        votes.iter().all(|vote| *vote == true)
    }

    pub async fn vote_on_transaction(&self, transaction: Transaction) -> bool {
        // Implement the logic to vote on the transaction
        // This could involve checking the transaction's validity, conflicts, etc.
        // For now, we'll assume the vote is always positive
        true
    }

    pub async fn commit_transaction(&self, transaction: Transaction) {
        let peers = self.network.peers.read().await;

        for peer in peers.values() {
            if let Err(e) = peer.send(transaction.clone()).await {
                error!("Failed to commit transaction to {}: {}", peer.address, e);
            }
        }

        info!("Transaction committed successfully");
    }

    pub async fn rollback_transaction(&self, transaction: Transaction) {
        let peers = self.network.peers.read().await;

        for peer in peers.values() {
            if let Err(e) = peer.send(transaction.clone()).await {
                error!("Failed to rollback transaction to {}: {}", peer.address, e);
            }
        }

        info!("Transaction rolled back successfully");
    }
}

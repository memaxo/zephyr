use crate::chain::transaction::Transaction;
use crate::network::p2p::peer::Peer;
use crate::network::Network;
use crate::utils::node_id::NodeId;
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TransactionRelay {
    network: Arc<Network>,
}

impl TransactionRelay {
    pub fn new(network: Arc<Network>) -> Self {
        TransactionRelay { network }
    }

    pub async fn relay_transaction(&self, transaction: Transaction, target_shard_id: u64) {
        let peers = self.network.peers.read().await;
        for peer in peers.values() {
            if let Err(e) = peer.send_transaction(transaction.clone(), target_shard_id).await {
                error!("Failed to relay transaction to {}: {}", peer.address, e);
            }
        }
    }

    pub async fn verify_transaction_origin(&self, transaction: &Transaction, origin_shard_id: u64) -> bool {
        // Implement the logic to verify the origin of the transaction
        // This could involve checking the transaction's signature, origin shard ID, etc.
        // For now, we'll assume the verification is always successful
        true
    }

    pub async fn handle_relay_request(&self, peer: &Peer, transaction: Transaction, origin_shard_id: u64, target_shard_id: u64) {
        info!("Handling relay request from {} for transaction from shard {} to shard {}", peer.address, origin_shard_id, target_shard_id);

        if self.verify_transaction_origin(&transaction, origin_shard_id).await {
            self.relay_transaction(transaction, target_shard_id).await;
        } else {
            error!("Failed to verify transaction origin from shard {}", origin_shard_id);
        }
    }
}

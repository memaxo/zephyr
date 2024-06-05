use crate::hdcmodels::HDCModel;
use crate::network::message::Message;
use crate::network::peer::Peer;
use crate::network::NetworkManager;
use crate::qup::crypto::QUPSignature;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUpdateMessage {
    pub model_update: Vec<f64>,
    pub signature: QUPSignature,
}

pub struct Synchronizer {
    network_manager: Arc<NetworkManager>,
    model: Arc<RwLock<HDCModel>>,
}

impl Synchronizer {
    pub fn new(network_manager: Arc<NetworkManager>, model: Arc<RwLock<HDCModel>>) -> Self {
        Synchronizer {
            network_manager,
            model,
        }
    }

    pub async fn synchronize_model_update(&self, model_update: Vec<f64>) {
        // Sign the model update using the node's private key
        let signature = self.network_manager.sign_model_update(&model_update).await;

        // Create a model update message
        let message = ModelUpdateMessage {
            model_update,
            signature,
        };

        // Broadcast the model update message to all peers
        self.network_manager
            .broadcast(Message::ModelUpdate(message))
            .await;
    }

    pub async fn handle_model_update(&self, peer: &Peer, message: ModelUpdateMessage) {
        // Verify the signature of the model update
        if !self
            .network_manager
            .verify_model_update_signature(&message)
            .await
        {
            log::warn!(
                "Received model update with invalid signature from {}",
                peer.address
            );
            return;
        }

        // Acquire a write lock on the model
        let mut model = self.model.write().await;

        // Update the model with the received model update
        model.apply_update(&message.model_update);

        log::info!("Synchronized model update from {}", peer.address);
    }
}

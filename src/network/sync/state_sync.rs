use crate::chain::consensus::Consensus;
use crate::chain::shard::ShardState;
use crate::chain::state::State;
use crate::crypto::signature::{Signature, SignatureScheme};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Serialize, Deserialize, Debug)]
pub enum StateSyncMessage {
    RequestState { shard_id: u64 },
    StateResponse { shard_id: u64, state: ShardState, signature: Signature },
}

#[derive(Error, Debug)]
pub enum StateSyncError {
    #[error("Channel send error: {0}")]
    ChannelSendError(String),
    #[error("Invalid state signature for shard: {0}")]
    InvalidStateSignature(u64),
}

pub struct StateSync {
    consensus: Arc<dyn Consensus>,
    state_sync_channel: (Sender<StateSyncMessage>, Receiver<StateSyncMessage>),
    signature_scheme: SignatureScheme,
    use_quantum: bool,
}

impl StateSync {
    pub fn new(consensus: Arc<dyn Consensus>, signature_scheme: SignatureScheme, use_quantum: bool) -> Self {
        let (tx, rx) = mpsc::channel(100);
        StateSync {
            consensus,
            state_sync_channel: (tx, rx),
            signature_scheme,
            use_quantum,
        }
    }

    pub async fn start(&mut self) {
        while let Some(message) = self.state_sync_channel.1.recv().await {
            match message {
                StateSyncMessage::RequestState { shard_id } => {
                    if let Err(e) = self.handle_state_request(shard_id).await {
                        error!("Error handling state request: {}", e);
                    }
                }
                StateSyncMessage::StateResponse {
                    shard_id,
                    state,
                    signature,
                } => {
                    if let Err(e) = self.handle_state_response(shard_id, state, signature).await {
                        error!("Error handling state response: {}", e);
                    }
                }
            }
        }
    }

    async fn handle_state_request(&self, shard_id: u64) -> Result<(), StateSyncError> {
        if let Some(shard_state) = self.consensus.get_shard_state(shard_id).await {
            let signature = self.sign_state(&shard_state);
            let response = StateSyncMessage::StateResponse {
                shard_id,
                state: shard_state,
                signature,
            };
            if self.use_quantum {
                // Quantum state synchronization logic
                self.send_quantum_state_response(response).await?;
            } else {
                // Classical state synchronization logic
                self.send_state_response(response).await?;
            }
            Ok(())
        } else {
            Err(StateSyncError::InvalidShardId(shard_id))
        }
    }

    async fn handle_state_response(
        &self,
        shard_id: u64,
        state: ShardState,
        signature: Signature,
    ) -> Result<(), StateSyncError> {
        if self.verify_state_signature(&state, &signature) {
            self.consensus.update_shard_state(shard_id, state).await;
            info!("Updated state for shard {}", shard_id);
            Ok(())
        } else {
            if self.use_quantum {
                // Fallback to classical state synchronization
                warn!("Quantum state verification failed, falling back to classical mode");
                self.handle_classical_state_response(shard_id, state, signature).await?;
            } else {
                Err(StateSyncError::InvalidStateSignature(shard_id))
            }
        }
    }

    async fn handle_classical_state_response(
        &self,
        shard_id: u64,
        state: ShardState,
        signature: Signature,
    ) -> Result<(), StateSyncError> {
        if self.verify_state_signature(&state, &signature) {
            self.consensus.update_shard_state(shard_id, state).await;
            info!("Updated state for shard {} in classical mode", shard_id);
            Ok(())
        } else {
            Err(StateSyncError::InvalidStateSignature(shard_id))
        }
    }

    fn sign_state(&self, state: &ShardState) -> Signature {
        self.signature_scheme.sign(state)
    }

    fn verify_state_signature(&self, state: &ShardState, signature: &Signature) -> bool {
        self.signature_scheme.verify(state, signature)
    }

    async fn send_state_response(&self, response: StateSyncMessage) -> Result<(), StateSyncError> {
        self.state_sync_channel
            .0
            .send(response)
            .await
            .map_err(|e| StateSyncError::ChannelSendError(e.to_string()))
    }

    async fn send_quantum_state_response(&self, response: StateSyncMessage) -> Result<(), StateSyncError> {
        // Implement the logic to send the state response using quantum communication
        // This is a placeholder implementation
        self.send_state_response(response).await
    }
}

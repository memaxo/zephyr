use crate::chain::shard::ShardState;
use crate::network::sync::state_sync::{StateSyncMessage, StateSyncError};
use log::info;
use tokio::sync::mpsc::Sender;

pub struct FastSync {
    state_sync_channel: Sender<StateSyncMessage>,
}

impl FastSync {
    pub fn new(state_sync_channel: Sender<StateSyncMessage>) -> Self {
        FastSync {
            state_sync_channel,
        }
    }

    pub async fn start(&mut self, shard_id: u64) {
        // Request the latest shard state from the network
        self.request_latest_state(shard_id).await.unwrap();

        // Process state sync responses
        while let Some(message) = self.state_sync_channel.recv().await {
            match message {
                StateSyncMessage::StateResponse { shard_id, state } => {
                    self.handle_state_response(shard_id, state).await;
                }
                _ => {
                    // Ignore other messages
                }
            }
        }
    }

    async fn request_latest_state(&mut self, shard_id: u64) -> Result<(), StateSyncError> {
        let state_request = StateSyncMessage::RequestState { shard_id };
        self.state_sync_channel.send(state_request).await.map_err(|e| StateSyncError::ChannelSendError(e.to_string()))
    }

    async fn handle_state_response(&mut self, shard_id: u64, state: ShardState) {
        // Update the local shard state with the received state
        // ...

        info!("Fast sync completed for shard {}", shard_id);
    }
}

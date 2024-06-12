pub mod config;
pub mod discovery;
pub mod error;
pub mod handler;
pub mod p2p;
pub mod protocol;
pub mod server;
pub mod sync;
pub mod client;

pub use config::NetworkConfig;
pub use discovery::Discovery;
pub use error::NetworkError;
pub use handler::{Handler, HandlerImpl};
pub use p2p::message::Message;
pub use p2p::peer::Peer;
pub use protocol::ProtocolMessage;
pub use server::Server;
pub use sync::state_sync::{StateSync, StateSyncMessage};
pub use client::Client;

use libp2p::Swarm;
use std::sync::Arc;

pub struct Network {
    pub config: NetworkConfig,
    pub server: Server,
    pub discovery: Discovery,
    pub state_sync: StateSync,
    pub handler: Arc<dyn Handler>,
}

impl Network {
    pub fn new(config: NetworkConfig, handler: Arc<dyn Handler>) -> Result<Self, NetworkError> {
        let discovery = Discovery::new(config.clone())?;
        let server = Server::new(config.clone(), handler.clone())?;
        let state_sync = StateSync::new(config.clone(), handler.clone())?;

        Ok(Network {
            config,
            server,
            discovery,
            state_sync,
            handler,
        })
    }

    pub async fn start(&mut self) -> Result<(), NetworkError> {
        tokio::spawn(async move {
            if let Err(e) = self.server.start().await {
                log::error!("Server error: {}", e);
            }
        });

        tokio::spawn(async move {
            if let Err(e) = self.discovery.start().await {
                log::error!("Discovery error: {}", e);
            }
        });

        tokio::spawn(async move {
            if let Err(e) = self.state_sync.start().await {
                log::error!("State sync error: {}", e);
            }
        });

        Ok(())
    }

    pub fn connect(&mut self, address: &str) -> Result<(), NetworkError> {
        let client = Client::new(self.config.clone(), self.handler.clone())?;
        client.connect(address)?;
        Ok(())
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), NetworkError> {
        self.server.broadcast(message)
    }

    pub fn send_state_sync_message(&mut self, message: StateSyncMessage) -> Result<(), NetworkError> {
        self.state_sync.send_message(message)
    }
}

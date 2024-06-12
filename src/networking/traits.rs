use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait PeerDiscovery {
    async fn discover_peers(&self) -> Result<Vec<String>, Box<dyn Error>>;
}

#[async_trait]
pub trait ConnectionManager {
    async fn connect(&self, address: &str) -> Result<(), Box<dyn Error>>;
    async fn disconnect(&self, address: &str) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait MessagePassing {
    async fn send_message(&self, address: &str, message: &[u8]) -> Result<(), Box<dyn Error>>;
    async fn receive_message(&self, address: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}

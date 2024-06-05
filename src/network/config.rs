use std::fs;
use std::sync::RwLock;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use log::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The address to listen on for incoming connections.
    pub listen_address: String,
    /// A list of bootstrap nodes to connect to.
    pub bootstrap_nodes: Vec<String>,
    /// The maximum number of connections allowed.
    pub max_connections: usize,
    /// The timeout duration for handshakes.
    pub handshake_timeout: Duration,
    /// The timeout duration for messages.
    pub message_timeout: Duration,
    /// The interval for sending ping messages.
    pub ping_interval: Duration,
    /// The maximum size of a message.
    pub max_message_size: usize,
    /// The maximum size of a frame.
    pub max_frame_size: usize,
    /// Whether to use TLS for connections.
    pub use_tls: bool,
    /// The path to the TLS certificate file.
    pub tls_certificate_path: Option<String>,
    /// The path to the TLS private key file.
    pub tls_private_key_path: Option<String>,
    /// Whether to use quantum-resistant features.
    pub use_quantum: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            listen_address: "/ip4/0.0.0.0/tcp/0".to_string(),
            bootstrap_nodes: Vec::new(),
            max_connections: 10,
            handshake_timeout: Duration::from_secs(10),
            message_timeout: Duration::from_secs(10),
            ping_interval: Duration::from_secs(60),
            max_message_size: 1024 * 1024, // 1 MB
            max_frame_size: 1024 * 1024, // 1 MB
            use_tls: false,
            tls_certificate_path: None,
            tls_private_key_path: None,
            use_quantum: false,
        }
    }
}

impl NetworkConfig {
    /// Creates a new NetworkConfig with the specified listen address and bootstrap nodes.
    pub fn new(listen_address: String, bootstrap_nodes: Vec<String>) -> Self {
        NetworkConfig {
            listen_address,
            bootstrap_nodes,
            ..Default::default()
        }
    }

    /// Enables TLS with the specified certificate and private key paths.
    pub fn with_tls(
        mut self,
        tls_certificate_path: String,
        tls_private_key_path: String,
    ) -> Self {
        self.use_tls = true;
        self.tls_certificate_path = Some(tls_certificate_path);
        self.tls_private_key_path = Some(tls_private_key_path);
        self
    }

    /// Sets the maximum number of connections.
    pub fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Sets the handshake timeout duration.
    pub fn with_handshake_timeout(mut self, timeout: Duration) -> Self {
        self.handshake_timeout = timeout;
        self
    }

    /// Sets the message timeout duration.
    pub fn with_message_timeout(mut self, timeout: Duration) -> Self {
        self.message_timeout = timeout;
        self
    }

    /// Sets the ping interval duration.
    pub fn with_ping_interval(mut self, interval: Duration) -> Self {
        self.ping_interval = interval;
        self
    }

    /// Sets the maximum message size.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    /// Sets the maximum frame size.
    pub fn with_max_frame_size(mut self, size: usize) -> Self {
        self.max_frame_size = size;
        self
    }

    /// Loads the configuration from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(path)?;
        let config: NetworkConfig = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    /// Saves the configuration to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_data = serde_json::to_string_pretty(self)?;
        fs::write(path, config_data)?;
        Ok(())
    }
}

lazy_static::lazy_static! {
    static ref CONFIG: RwLock<NetworkConfig> = RwLock::new(NetworkConfig::default());
}

/// Reloads the configuration from a JSON file.
pub fn reload_config(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let new_config = NetworkConfig::load_from_file(path)?;
    let mut config = CONFIG.write().unwrap();
    *config = new_config;
    info!("Configuration reloaded from {}", path);
    Ok(())
}

/// Gets a clone of the current configuration.
pub fn get_config() -> NetworkConfig {
    CONFIG.read().unwrap().clone()
}

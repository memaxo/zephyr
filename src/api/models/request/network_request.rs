use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ConnectPeerRequest {
    pub peer_address: String,
}

#[derive(Deserialize, Debug)]
pub struct DisconnectPeerRequest {
    pub peer_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateNetworkConfigRequest {
    pub listen_addr: Option<String>,
    pub max_peers: Option<u32>,
    pub seed_nodes: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct GetNetworkStatsRequest {}

#[derive(Deserialize, Debug)]
pub struct BanPeerRequest {
    pub peer_id: String,
    pub ban_duration: Option<u64>, // Duration in seconds
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub is_outbound: bool,
    // Add other peer information fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub max_peers: u32,
    pub seed_nodes: Vec<String>,
    // Add other network configuration fields as needed
}
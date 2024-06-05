use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct GetPeersResponse {
    pub peers: Vec<PeerInfo>,
}

#[derive(Serialize, Debug)]
pub struct GetNetworkStatsResponse {
    pub stats: NetworkStats,
}

#[derive(Serialize, Debug)]
pub struct GetNetworkConfigResponse {
    pub config: NetworkConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub is_outbound: bool,
    pub last_seen: u64, // Unix timestamp
    pub banned_until: Option<u64>, // Unix timestamp
    // Add other peer information fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkStats {
    pub total_peers: usize,
    pub incoming_peers: usize,
    pub outgoing_peers: usize,
    pub total_bytes_received: u64,
    pub total_bytes_sent: u64,
    // Add other network stats fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub max_peers: u32,
    pub seed_nodes: Vec<String>,
    // Add other network configuration fields as needed
}
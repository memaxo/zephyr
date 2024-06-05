use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::network::{Network, Peer};

pub async fn get_peers(network: web::Data<Network>) -> impl Responder {
    let peers: Vec<PeerInfo> = network
        .peers()
        .iter()
        .map(|peer| PeerInfo::from_peer(peer))
        .collect();

    HttpResponse::Ok().json(peers)
}

pub async fn get_network_stats(network: web::Data<Network>) -> impl Responder {
    let stats = NetworkStats {
        total_peers: network.peers().len(),
        incoming_peers: network.peers().iter().filter(|p| !p.is_outbound()).count(),
        outgoing_peers: network.peers().iter().filter(|p| p.is_outbound()).count(),
        total_bytes_received: network.total_bytes_received(),
        total_bytes_sent: network.total_bytes_sent(),
    };

    HttpResponse::Ok().json(stats)
}

pub async fn get_network_config(network: web::Data<Network>) -> impl Responder {
    let config = NetworkConfig {
        listen_addr: network.listen_addr().to_string(),
        max_peers: network.max_peers(),
        seed_nodes: network.seed_nodes().iter().map(|n| n.to_string()).collect(),
    };

    HttpResponse::Ok().json(config)
}

#[derive(Serialize)]
struct PeerInfo {
    id: String,
    address: String,
    is_outbound: bool,
}

impl PeerInfo {
    fn from_peer(peer: &Peer) -> Self {
        Self {
            id: peer.id().to_string(),
            address: peer.address().to_string(),
            is_outbound: peer.is_outbound(),
        }
    }
}

#[derive(Serialize)]
struct NetworkStats {
    total_peers: usize,
    incoming_peers: usize,
    outgoing_peers: usize,
    total_bytes_received: u64,
    total_bytes_sent: u64,
}

#[derive(Serialize)]
struct NetworkConfig {
    listen_addr: String,
    max_peers: usize,
    seed_nodes: Vec<String>,
}
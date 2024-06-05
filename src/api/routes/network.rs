use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::network::{Network, Peer};

pub fn network_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/network")
            .route("/peers", web::get().to(get_peers))
            .route("/stats", web::get().to(get_network_stats))
            .route("/config", web::get().to(get_network_config)),
    );
}

#[derive(Serialize, Deserialize)]
struct PeerInfo {
    id: String,
    address: String,
    is_outbound: bool,
    // Add other relevant peer fields
}

#[derive(Serialize, Deserialize)]
struct NetworkStats {
    total_peers: usize,
    incoming_peers: usize,
    outgoing_peers: usize,
    total_bytes_received: u64,
    total_bytes_sent: u64,
    // Add other relevant network stats fields
}

#[derive(Serialize, Deserialize)]
struct NetworkConfig {
    listen_addr: String,
    max_peers: usize,
    seed_nodes: Vec<String>,
    // Add other relevant network configuration fields
}

async fn get_peers(network: web::Data<Network>) -> impl Responder {
    let peers: Vec<PeerInfo> = network
        .peers()
        .iter()
        .map(|peer| PeerInfo {
            id: peer.id().to_string(),
            address: peer.address().to_string(),
            is_outbound: peer.is_outbound(),
            // Add other relevant peer fields
        })
        .collect();

    HttpResponse::Ok().json(peers)
}

async fn get_network_stats(network: web::Data<Network>) -> impl Responder {
    let stats = NetworkStats {
        total_peers: network.peers().len(),
        incoming_peers: network.peers().iter().filter(|p| !p.is_outbound()).count(),
        outgoing_peers: network.peers().iter().filter(|p| p.is_outbound()).count(),
        total_bytes_received: network.total_bytes_received(),
        total_bytes_sent: network.total_bytes_sent(),
        // Add other relevant network stats fields
    };

    HttpResponse::Ok().json(stats)
}

async fn get_network_config(network: web::Data<Network>) -> impl Responder {
    let config = NetworkConfig {
        listen_addr: network.listen_addr().to_string(),
        max_peers: network.max_peers(),
        seed_nodes: network.seed_nodes().iter().map(|n| n.to_string()).collect(),
        // Add other relevant network configuration fields
    };

    HttpResponse::Ok().json(config)
}
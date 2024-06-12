use libp2p::{
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    PeerId, Multiaddr, Transport,
    identity::Keypair,
    tcp::TcpConfig,
    yamux::YamuxConfig,
    mplex::MplexConfig,
    noise::{Keypair as NoiseKeypair, NoiseConfig, X25519Spec, AuthenticKeypair, NoiseKeypairIdentity},
    core::upgrade,
    dns::DnsConfig,
    futures::prelude::*,
};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use crate::network::tls::{PostQuantumTLSConnection, PostQuantumTLSConfig};
use tokio::net::TcpStream;

pub struct ConnectionManager {
    swarm: Swarm<NoiseConfig<YamuxConfig>>,
    known_peers: Arc<RwLock<HashSet<PeerId>>>,
    pq_tls_connections: Arc<RwLock<HashMap<PeerId, PostQuantumTLSConnection>>>,
}

impl ConnectionManager {
    pub fn new(local_key: Keypair, bootstrap_nodes: Vec<Multiaddr>) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        info!("Local peer ID: {}", local_peer_id);

        let noise_keys = NoiseKeypair::<X25519Spec>::new().into_authentic(&local_key).expect("Signing libp2p-noise static DH keypair failed.");
        let transport = TcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .boxed();

        let swarm = SwarmBuilder::new(transport, local_peer_id.clone())
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        ConnectionManager {
            swarm,
            known_peers: Arc::new(RwLock::new(HashSet::new())),
        }

        let pq_tls_connections = Arc::new(RwLock::new(HashMap::new()));

        ConnectionManager {
            swarm,
            known_peers: Arc::new(RwLock::new(HashSet::new())),
            pq_tls_connections,
    }

    pub async fn start(&mut self) {
        info!("Starting connection manager");

        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(event) => {
                    info!("Swarm event: {:?}", event);
                }
                _ => {}
            }
        }
    }

    pub async fn connect(&mut self, addr: Multiaddr) -> Result<(), NetworkError> {
        let stream = TcpStream::connect(&addr).await.map_err(|e| {
            error!("Failed to connect to {:?}: {}", addr, e);
            NetworkError::ConnectionError(format!("Failed to connect to {:?}: {}", addr, e))
        })?;

        let config = PostQuantumTLSConfig::new();
        let pq_tls_connection = PostQuantumTLSConnection::new(stream, config).await.map_err(|e| {
            error!("TLS connection establishment failed: {}", e);
            NetworkError::ConnectionError(format!("TLS connection establishment failed: {}", e))
        })?;

        let peer_id = self.generate_peer_id(&addr)?;
        self.pq_tls_connections.write().await.insert(peer_id, pq_tls_connection);
        info!("TLS connection established with {:?}", addr);

        Ok(())
    }

    pub async fn disconnect(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
        if let Some(mut pq_tls_connection) = self.pq_tls_connections.write().await.remove(&peer_id) {
            pq_tls_connection.close().await.map_err(|e| {
                error!("Failed to close TLS connection: {}", e);
                NetworkError::ConnectionError(format!("Failed to close TLS connection: {}", e))
            })?;
            info!("TLS connection closed with peer {:?}", peer_id);
        } else {
            error!("No TLS connection found for peer {:?}", peer_id);
            return Err(NetworkError::ConnectionError(format!("No TLS connection found for peer {:?}", peer_id)));
        }
        Ok(())
    }

    pub async fn add_known_peer(&self, peer_id: PeerId) {
        self.known_peers.write().await.insert(peer_id);
    }

    pub async fn remove_known_peer(&self, peer_id: &PeerId) {
        self.known_peers.write().await.remove(peer_id);
    }

    pub async fn get_known_peers(&self) -> HashSet<PeerId> {
        self.known_peers.read().await.clone()
    }
}

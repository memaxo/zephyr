use libp2p::kad::{
    record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryResult,
};
use libp2p::swarm::{SwarmBuilder, SwarmEvent};
use libp2p::{identity::Keypair, Multiaddr, PeerId, Swarm};
use log::{debug, error, info};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const DISCOVERY_INTERVAL: Duration = Duration::from_secs(60);

pub struct PeerDiscovery {
    swarm: Swarm<Kademlia<MemoryStore>>,
    bootstrap_nodes: Vec<Multiaddr>,
    known_peers: Arc<RwLock<HashSet<PeerId>>>,
    active_peers: Arc<RwLock<HashSet<PeerId>>>,
}

impl PeerDiscovery {
    pub fn new(local_key: Keypair, bootstrap_nodes: Vec<Multiaddr>) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        info!("Local peer ID: {}", local_peer_id);

        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(5));
        let store = MemoryStore::new(local_peer_id);
        let mut kademlia = Kademlia::with_config(local_peer_id, store, kademlia_config);
        for addr in &bootstrap_nodes {
            kademlia.add_address(&local_peer_id, addr.clone());
        }

        let swarm = SwarmBuilder::new(kademlia, local_key)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        PeerDiscovery {
            swarm,
            bootstrap_nodes,
            known_peers: Arc::new(RwLock::new(HashSet::new())),
            active_peers: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub async fn start(&mut self) {
        info!("Starting peer discovery");

        for addr in &self.bootstrap_nodes {
            match self.swarm.dial(addr.clone()) {
                Ok(_) => info!("Dialed bootstrap node: {}", addr),
                Err(e) => error!("Failed to dial bootstrap node: {} (error: {})", addr, e),
            }
        }

        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(KademliaEvent::QueryResult {
                    result: QueryResult::GetClosestPeers(Ok(peer_ids)),
                    ..
                }) => {
                    for peer_id in peer_ids {
                        self.discover_peer(peer_id).await;
                    }
                }
                SwarmEvent::Behaviour(KademliaEvent::QueryResult {
                    result: QueryResult::GetProviders(Ok(peer_ids)),
                    ..
                }) => {
                    for peer_id in peer_ids {
                        self.discover_peer(peer_id).await;
                    }
                }
                SwarmEvent::Behaviour(KademliaEvent::RoutingUpdated { peer, .. }) => {
                    self.discover_peer(peer).await;
                }
                SwarmEvent::Behaviour(KademliaEvent::UnroutablePeer { peer_id }) => {
                    self.remove_peer(peer_id).await;
                }
                _ => {}
            }

            // Perform periodic peer discovery
            tokio::time::sleep(DISCOVERY_INTERVAL).await;
            self.discover_peers().await;
        }
    }

    async fn discover_peer(&mut self, peer_id: PeerId) {
        let mut known_peers = self.known_peers.write().await;
        if !known_peers.contains(&peer_id) {
            info!("Discovered new peer: {}", peer_id);
            known_peers.insert(peer_id.clone());

            let mut active_peers = self.active_peers.write().await;
            active_peers.insert(peer_id);
            info!("Added peer {} to active peers", peer_id);
        }
    }

    async fn remove_peer(&mut self, peer_id: PeerId) {
        let mut known_peers = self.known_peers.write().await;
        if known_peers.remove(&peer_id) {
            info!("Removed unroutable peer: {}", peer_id);
        }
    }

    async fn discover_peers(&mut self) {
        info!("Performing periodic peer discovery");
        let local_peer_id = *self.swarm.local_peer_id();
        self.swarm.behaviour_mut().get_closest_peers(local_peer_id);
    }

    pub async fn get_active_peers(&self) -> HashSet<PeerId> {
        self.active_peers.read().await.clone()
    }
}

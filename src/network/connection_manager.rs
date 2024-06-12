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

pub struct ConnectionManager {
    swarm: Swarm<NoiseConfig<YamuxConfig>>,
    known_peers: Arc<RwLock<HashSet<PeerId>>>,
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

    pub async fn connect(&mut self, addr: Multiaddr) {
        match self.swarm.dial(addr.clone()) {
            Ok(_) => info!("Dialed {:?}", addr),
            Err(e) => error!("Failed to dial {:?}: {:?}", addr, e),
        }
    }

    pub async fn disconnect(&mut self, peer_id: PeerId) {
        self.swarm.disconnect_peer_id(peer_id).expect("Failed to disconnect peer");
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

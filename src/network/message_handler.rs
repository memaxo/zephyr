use crate::network::p2p::message::Message;
use crate::network::tls::{PostQuantumTLSConnection, PostQuantumTLSConfig};
use libp2p::floodsub::{Floodsub, FloodsubEvent, Topic};
use libp2p::gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic, MessageId, ValidationMode};
use libp2p::swarm::Swarm;
use libp2p::{identity, PeerId};
use std::collections::HashSet;
use tokio::net::TcpStream;
use log::{error, info};

pub struct MessageHandler {
    floodsub: Floodsub,
    gossipsub: Gossipsub,
    peers: HashSet<PeerId>,
    pq_tls_connection: Option<PostQuantumTLSConnection>,
}

impl MessageHandler {
    pub fn new(local_key: identity::Keypair) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        let floodsub = Floodsub::new(local_peer_id.clone());
        let gossipsub = Gossipsub::new(
            ValidationMode::Strict,
            local_peer_id.clone(),
        ).expect("Correct configuration");

        let config = PostQuantumTLSConfig::new();
        let stream = TcpStream::connect("localhost:12345").await.expect("Failed to connect to server");
        let pq_tls_connection = PostQuantumTLSConnection::new(stream, config).await.expect("Failed to establish TLS connection");

        MessageHandler {
            pq_tls_connection: Some(pq_tls_connection),
            floodsub,
            gossipsub,
            peers: HashSet::new(),
        }
    }

    pub fn add_peer(&mut self, peer_id: PeerId) {
        self.peers.insert(peer_id.clone());
        self.floodsub.add_node_to_partial_view(peer_id.clone());
        self.gossipsub.add_explicit_peer(&peer_id);
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peers.remove(peer_id);
        self.floodsub.remove_node_from_partial_view(peer_id);
        self.gossipsub.remove_explicit_peer(peer_id);
    }

    pub fn subscribe(&mut self, topic: &str) {
        let topic = Topic::new(topic);
        self.floodsub.subscribe(topic.clone());
        self.gossipsub.subscribe(&topic).unwrap();
    }

    pub fn publish(&mut self, topic: &str, message: Message) {
        let topic = Topic::new(topic);
        if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
            let data = message.serialize().unwrap();
            pq_tls_connection.send(&data).await.expect("Failed to send message over TLS");
        } else {
            error!("TLS connection not established");
        }
    }

    pub async fn handle_events(&mut self, swarm: &mut Swarm<Self>) {
        loop {
            match swarm.next().await {
                Some(FloodsubEvent::Message(message)) => {
                    if let Some(pq_tls_connection) = &mut self.pq_tls_connection {
                        let data = pq_tls_connection.receive().await.expect("Failed to receive message over TLS");
                        if let Ok(msg) = Message::deserialize(&data) {
                            // Handle the message
                        }
                    } else {
                        error!("TLS connection not established");
                    }
                }
                Some(GossipsubEvent::Message {
                    propagation_source: _,
                    message_id: _,
                    message,
                }) => {
                    if let Ok(msg) = Message::deserialize(&message.data) {
                        // Handle the message
                    }
                }
                _ => {}
            }
        }
    }
}

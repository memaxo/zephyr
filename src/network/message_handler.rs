use crate::network::message::Message;
use prost::Message as _;
use crate::network::tls::{PostQuantumTLSConnection, PostQuantumTLSConfig};
use libp2p::floodsub::{Floodsub, FloodsubEvent, Topic};
use libp2p::gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic, MessageId, ValidationMode};
use libp2p::swarm::Swarm;
use libp2p::{identity, PeerId};
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use log::{error, info};

pub struct MessageHandler {
    floodsub: Floodsub,
    gossipsub: Gossipsub,
    peers: BTreeSet<PeerId>,
    pq_tls_connection: Option<PostQuantumTLSConnection>,
    message_cache: Arc<Mutex<LRUCache<MessageId, Message>>>,
}

struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    list: Vec<K>,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            list: Vec::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.map.get(key) {
            let index = self.list.iter().position(|k| k == key).unwrap();
            self.list.remove(index);
            self.list.push(key.clone());
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            let index = self.list.iter().position(|k| k == &key).unwrap();
            self.list.remove(index);
        } else if self.list.len() == self.capacity {
            let oldest_key = self.list.remove(0);
            self.map.remove(&oldest_key);
        }
        self.list.push(key.clone());
        self.map.insert(key, value);
    }
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
            peers: BTreeSet::new(),
            message_cache: Arc::new(Mutex::new(LRUCache::new(100))), // Adjust cache capacity as needed
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
            let mut buf = Vec::with_capacity(message.encoded_len());
            message.encode(&mut buf).expect("Failed to encode message");
            pq_tls_connection.send(&buf).await.expect("Failed to send message over TLS");
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
                        match Message::decode(&data[..]) {
                            Ok(msg) => {
                                // Handle the message
                            },
                            Err(e) => {
                                error!("Failed to decode message: {}", e);
                            }
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
                    let message_id = message.id.clone();
                    if let Some(cached_message) = self.message_cache.lock().unwrap().get(&message_id) {
                        // Handle the cached message
                    } else {
                        match Message::decode(&message.data[..]) {
                            Ok(msg) => {
                                self.message_cache.lock().unwrap().put(message_id, msg.clone());
                                // Handle the message
                            },
                            Err(e) => {
                                error!("Failed to decode gossipsub message: {}", e);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

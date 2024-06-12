use crate::chain::block::Block;
use crate::chain::transaction::Transaction;
use crate::crypto::hash::Hasher;
use libp2p::gossipsub::{
    Gossipsub, GossipsubConfigBuilder, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
    MessageId, ValidationMode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Ping,
    Pong,
    GetPeers,
    Peers(Vec<String>),
    Transaction(Transaction),
    Block(Block),
    GetBlocks(Hasher),
    Blocks(Vec<Block>),
    GetState(u64),
    State(Vec<u8>),
}

impl Message {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}

pub struct MessageHandler {
    floodsub: Floodsub,
    gossipsub: Gossipsub,
    peers: HashSet<PeerId>,
}

impl MessageHandler {
    pub fn new(local_key: identity::Keypair) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        let floodsub = Floodsub::new(local_peer_id.clone());
        let gossipsub = Gossipsub::new(
            ValidationMode::Strict,
            local_peer_id.clone(),
        ).expect("Correct configuration");

        MessageHandler {
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
        let data = message.serialize().unwrap();
        self.floodsub.publish(topic.clone(), data.clone());
        self.gossipsub.publish(topic, data).unwrap();
    }

    pub async fn handle_events(&mut self, swarm: &mut Swarm<Self>) {
        loop {
            match swarm.next().await {
                Some(FloodsubEvent::Message(message)) => {
                    if let Ok(msg) = Message::deserialize(&message.data) {
                        // Handle the message
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
}

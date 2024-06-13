use crate::qup::block::QUPBlock;
use crate::qup::crypto::{QUPKeyPair, encrypt_data, decrypt_data, sign_data, 
verify_signature, hash_data};
use crate::qup::state::QUPState;
use crate::network::{NetworkMessage, QUPMessage, UsefulWorkProblem,
UsefulWorkSolution, NetworkSender, NetworkReceiver, discover_peers,
connect_to_peer, disconnect_from_peer};
use crate::error::ConsensusError;
use crate::qup::quantum_communication::{QKDChannel, PostQuantumChannel};
use crate::hdcmodels::hdcmodels::HDCModel;
use std::sync::Arc;
use bincode;
use async_compression::tokio::bufread::{GzipEncoder, GzipDecoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use mpc::secure_aggregation;

use libp2p::{Multiaddr, PeerId, Swarm};
use libp2p::gossipsub::{Gossipsub, GossipsubConfigBuilder, GossipsubEvent, MessageAuthenticity, ValidationMode};
use libp2p::gossipsub::subscription::GossipsubSubscription;
use libp2p::swarm::SwarmEvent;
use futures::prelude::*;

pub enum NodeType {
    Classical,
    Quantum,
}

pub struct CommunicationProtocol {
    pub node_type: NodeType,
    pub key_pair: QUPKeyPair,
    pub peers: Vec<PeerId>,
    pub sender: NetworkSender,
    pub receiver: NetworkReceiver,
    pub swarm: Swarm<Gossipsub>,
}

impl CommunicationProtocol {
    pub async fn send_model_update(&self, model: &HDCModel, recipient: &str
-> Result<(), ConsensusError> {
        let serialized_model = bincode::serialize(model)?;
        let compressed_model = self.compress_data(&serialized_model).await?
        let encrypted_model = self.encrypt_with_qkd(&compressed_model,
recipient)?;
        let message = NetworkMessage::ModelUpdate {
            model: encrypted_model,
        };
        self.send_message(message, recipient).await
    }

    pub async fn receive_model_update(&self, message: NetworkMessage) ->
Result<HDCModel, ConsensusError> {
        if let NetworkMessage::ModelUpdate { model } = message {
            let decrypted_model = self.decrypt_with_qkd(&model)?;
            let decompressed_model =
self.decompress_data(&decrypted_model).await?;
            let deserialized_model: HDCModel =
bincode::deserialize(&decompressed_model)?;
            Ok(deserialized_model)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    async fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>,
ConsensusError> {
        let mut encoder = GzipEncoder::new(data);
        let mut compressed_data = Vec::new();
        encoder.read_to_end(&mut compressed_data).await?;
        Ok(compressed_data)
    }

    async fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>,
ConsensusError> {
        let mut decoder = GzipDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data).await?;
        Ok(decompressed_data)
    }

    fn encrypt_with_qkd(&self, data: &[u8], recipient: &str) ->
Result<Vec<u8>, ConsensusError> {
        let qkd_channel = QKDChannel::new();
        qkd_channel.encrypt(data, recipient)
    }

    fn decrypt_with_qkd(&self, data: &[u8]) -> Result<Vec<u8>,
ConsensusError> {
        let qkd_channel = QKDChannel::new();
        qkd_channel.decrypt(data)
    }

    pub async fn send_message(&self, message: NetworkMessage, recipient:
&str) -> Result<(), ConsensusError> {
        let serialized_message = bincode::serialize(&message)?;
        self.sender.send(serialized_message).await?;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<NetworkMessage,
ConsensusError> {
        let serialized_message = self.receiver.receive().await?;
        let network_message: NetworkMessage =
bincode::deserialize(&serialized_message)?;
        Ok(network_message)
    }

    pub async fn secure_aggregate_models(&self, models: Vec<HDCModel>) ->
Result<HDCModel, ConsensusError> {
        let aggregated_model = secure_aggregation::aggregate(models)?;
        Ok(aggregated_model)
    }

    pub fn new(node_type: NodeType, key_pair: QUPKeyPair, sender:
NetworkSender, receiver: NetworkReceiver) -> Self {
        CommunicationProtocol {
            node_type,
            key_pair,
            peers: Vec::new(),
            sender,
            receiver,
        }
    }

    pub fn discover_peers(&mut self) -> Result<(), ConsensusError> {
        let discovered_peers = discover_peers()?;
        self.peers.extend(discovered_peers);
        Ok(())
    }

    pub fn connect_to_peer(&mut self, peer: &str) -> Result<(),
ConsensusError> {
        connect_to_peer(peer)?;
        if !self.peers.contains(&peer.to_string()) {
            self.peers.push(peer.to_string());

            // Check if the connected peer is a QUP node
            if self.is_qup_node(peer)? {
                // Perform QKD with the QUP node
                self.perform_qkd(peer)?;

                // Establish a quantum communication channel
                self.establish_quantum_channel(peer)?;
            }
        }
        Ok(())
    }

    pub fn disconnect_from_peer(&mut self, peer: &str) -> Result<(),
ConsensusError> {
        disconnect_from_peer(peer)?;
        self.peers.retain(|p| p != peer);

        // Remove the shared key and quantum channel for the disconnected
peer
        self.shared_keys.remove(peer);
        self.quantum_channels.remove(peer);

        Ok(())
    }

    fn is_qup_node(&self, peer: &str) -> Result<bool, ConsensusError> {
        // Send a message to the peer to check if it's a QUP node
        let message = QUPMessage::IsQUPNode;
        let response = self.send_message(message, peer).await?;

        match response {
            QUPMessage::QUPNodeStatus(status) => Ok(status),
            _ => Err(ConsensusError::InvalidResponse),
        }
    }

    fn perform_qkd(&mut self, peer: &str) -> Result<(), ConsensusError> {
        // Perform quantum key distribution with the peer
        let shared_key = self.quantum_key_distribution.perform_qkd(peer)?;
        self.shared_keys.insert(peer.to_string(), shared_key);
        Ok(())
    }

    fn establish_quantum_channel(&mut self, peer: &str) -> Result<(),
ConsensusError> {
        // Establish a quantum communication channel with the peer
        let quantum_channel = self.quantum_channel.establish_channel(peer)?
        self.quantum_channels.insert(peer.to_string(), quantum_channel);
        Ok(())
    }

    pub async fn send_message(&self, message: QUPMessage) -> Result<(),
ConsensusError> {
        let network_message = NetworkMessage::QUPMessage(message);
        let serialized_message = bincode::serialize(&network_message)?;
        self.sender.send(serialized_message).await?;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<QUPMessage, ConsensusErro
{
        let serialized_message = self.receiver.receive().await?;
        let network_message: NetworkMessage =
bincode::deserialize(&serialized_message)?;
        if let NetworkMessage::QUPMessage(qup_message) = network_message {
            self.authenticate_message(&qup_message)?;
            self.verify_message_integrity(&qup_message)?;
            Ok(qup_message)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    pub fn send_proof(&self, proof: &[u8], recipient: &str) -> Result<(),
ConsensusError> {
        let encrypted_proof = encrypt_data(proof, &self.key_pair)?;
        let signature = sign_data(&encrypted_proof, &self.key_pair)?;
        let message = NetworkMessage::Proof {
            proof: encrypted_proof,
            signature,
        };
        send_message(recipient, message)
    }

    pub fn receive_proof(&self, message: NetworkMessage) -> Result<Vec<u8>,
ConsensusError> {
        if let NetworkMessage::Proof { proof, signature } = message {
            verify_signature(&proof, &signature, &self.key_pair)?;
            decrypt_data(&proof, &self.key_pair)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    pub fn send_result(&self, result: &[u8], recipient: &str) -> Result<(),
ConsensusError> {
        let encrypted_result = encrypt_data(result, &self.key_pair)?;
        let signature = sign_data(&encrypted_result, &self.key_pair)?;
        let message = NetworkMessage::Result {
            result: encrypted_result,
            signature,
        };
        send_message(recipient, message)
    }

    pub fn receive_result(&self, message: NetworkMessage) -> Result<Vec<u8>
ConsensusError> {
        if let NetworkMessage::Result { result, signature } = message {
            verify_signature(&result, &signature, &self.key_pair)?;
            decrypt_data(&result, &self.key_pair)
        } else {
            Err(ConsensusError::InvalidMessage)
        }
    }

    async fn authenticate_message(&self, message: &QUPMessage) -> Result<(), ConsensusError> {
        match message {
            QUPMessage::UsefulWorkProblem(problem) => {
                let serialized_problem = bincode::serialize(problem)?;
                let hashed_problem = hash_data(&serialized_problem)?;
                verify_signature(&hashed_problem, &problem.signature, &self.key_pair)
            }
            QUPMessage::UsefulWorkSolution(solution) => {
                let serialized_solution = bincode::serialize(solution)?;
                let hashed_solution = hash_data(&serialized_solution)?;
                verify_signature(&hashed_solution, &solution.signature, &self.key_pair)
            }
            _ => Ok(()),
        }
    }

    async fn verify_message_integrity(&self, message: &QUPMessage) -> Result<(), ConsensusError> {
        match message {
            QUPMessage::UsefulWorkProblem(problem) => {
                let serialized_problem = bincode::serialize(problem)?;
                let hashed_problem = hash_data(&serialized_problem)?;
                if hashed_problem != problem.hash {
                    return Err(ConsensusError::MessageIntegrityError);
                }
                Ok(())
            }
            QUPMessage::UsefulWorkSolution(solution) => {
                let serialized_solution = bincode::serialize(solution)?;
                let hashed_solution = hash_data(&serialized_solution)?;
                if hashed_solution != solution.hash {
                    return Err(ConsensusError::MessageIntegrityError);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, ConsensusError> {
        let mut encoder = GzipEncoder::new(data);
        let mut compressed_data = Vec::new();
        encoder.read_to_end(&mut compressed_data).await?;
        Ok(compressed_data)
    }

    async fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, ConsensusError> {
        let mut decoder = GzipDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data).await?;
        Ok(decompressed_data)
    }

    async fn secure_aggregate_models(&self, models: Vec<HDCModel>) -> Result<HDCModel, ConsensusError> {
        let aggregated_model = secure_aggregation::aggregate(models)?;
        Ok(aggregated_model)
    }
}
impl CommunicationProtocol {
    // ...

    pub async fn start_gossipsub(&mut self) {
        let message_authenticity = MessageAuthenticity::Signed(self.key_pair.clone());

        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(|message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                MessageId::from(s.finish().to_le_bytes())
            })
            .build()
            .expect("Valid config");

        let mut gossipsub = Gossipsub::new(message_authenticity, gossipsub_config).expect("Correct configuration");

        let topic = Topic::new("qup-network");
        let subscription = gossipsub.subscribe(&topic).unwrap();

        self.swarm = libp2p::swarm::SwarmBuilder::new(gossipsub, self.key_pair.public().to_peer_id(), {
            let tcp = libp2p::tcp::TokioTcpConfig::new().nodelay(true);
            let dns_tcp = DnsConfig::system(tcp).unwrap();
            let ws_dns_tcp = libp2p::websocket::WsConfig::new(dns_tcp.clone());
            dns_tcp.or_transport(ws_dns_tcp)
        })
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

        self.handle_gossipsub_events(subscription).await;
    }

    async fn handle_gossipsub_events(&mut self, mut subscription: GossipsubSubscription) {
        loop {
            while let Some(event) = subscription.next().await {
                match event {
                    GossipsubEvent::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    } => {
                        println!(
                            "Got message: {} with id: {} from peer: {:?}",
                            String::from_utf8_lossy(&message.data),
                            id,
                            peer_id
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    pub async fn broadcast(&mut self, topic: &str, message: &[u8]) {
        self.swarm
            .behaviour_mut()
            .publish(Topic::new(topic), message.to_vec())
            .unwrap();
    }

    pub async fn allreduce(&mut self, topic: &str, local_model: &HDCModel) -> Result<HDCModel, ConsensusError> {
        let serialized_model = bincode::serialize(local_model)?;
        self.broadcast(topic, &serialized_model).await;

        // Collect models from peers
        let mut models = vec![local_model.clone()];
        for _ in 0..self.peers.len() {
            let message = self.receive_message().await?;
            if let QUPMessage::ModelUpdate(model) = message {
                models.push(model);
            }
        }

        // Aggregate models
        let aggregated_model = self.secure_aggregate_models(models).await?;
        Ok(aggregated_model)
    }

    pub async fn gather(&mut self, topic: &str, local_data: &[u8]) -> Result<Vec<Vec<u8>>, ConsensusError> {
        self.broadcast(topic, local_data).await;

        // Collect data from peers
        let mut gathered_data = vec![local_data.to_vec()];
        for _ in 0..self.peers.len() {
            let message = self.receive_message().await?;
            if let QUPMessage::Data(data) = message {
                gathered_data.push(data);
            }
        }

        Ok(gathered_data)
    }

    // ...
}

use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use crate::network::shard_message::ShardMessage;
use crate::utils::error::NetworkError;

pub struct ShardNetwork {
    routing_table: HashMap<u64, Vec<String>>,
    relay_nodes: Vec<String>,
    message_sender: Option<Sender<(u64, ShardMessage)>>,
}

impl ShardNetwork {
    pub fn new() -> Self {
        ShardNetwork {
            routing_table: HashMap::new(),
            relay_nodes: Vec::new(),
            message_sender: None,
        }
    }

    pub async fn send_message(
        &self,
        target_shard_id: u64,
        message: ShardMessage,
    ) -> Result<(), NetworkError> {
        if let Some(committee_members) = self.routing_table.get(&target_shard_id) {
            // Direct communication
            let target_member = self.select_committee_member(target_shard_id, committee_members);
            self.send_message_to_member(target_member, message).await
        } else {
            // Relay network
            self.route_via_relay_nodes(target_shard_id, message).await
        }
    }

    async fn handle_fraud_proof(&self, fraud_proof: FraudProof) -> Result<(), NetworkError> {
        // Implement logic to handle fraud proof
        // For example, verify the fraud proof and take necessary actions
        // ...

        Ok(())
    }

    pub fn update_routing_table(&mut self, shard_id: u64, committee_members: Vec<String>) {
        self.routing_table.insert(shard_id, committee_members);
    }

    pub fn add_relay_node(&mut self, node: String) {
        self.relay_nodes.push(node);
    }

    pub async fn send_message(
        &self,
        target_shard_id: u64,
        message: ShardMessage,
    ) -> Result<(), NetworkError> {
        if let Some(committee_members) = self.routing_table.get(&target_shard_id) {
            // Direct communication
            let target_member = self.select_committee_member(target_shard_id, committee_members);
            self.send_message_to_member(target_member, message).await
        } else {
            // Relay network
            self.route_via_relay_nodes(target_shard_id, message).await
        }
    }

    fn select_committee_member(&self, shard_id: u64, committee_members: &[String]) -> String {
        // Implement a round-robin or load-balancing algorithm to select the committee member
        let index = (shard_id as usize) % committee_members.len();
        committee_members[index].clone()
    }

    async fn send_message_to_member(&self, member: String, message: ShardMessage) -> Result<(), NetworkError> {
        if let Some(sender) = &self.message_sender {
            sender.send((member.parse().unwrap(), message)).await.map_err(|e| {
                NetworkError::MessageSendingFailed(format!(
                    "Failed to send shard message to member {}: {}",
                    member, e
                ))
            })
        } else {
            Err(NetworkError::MessageSendingFailed(
                "Shard message sender not initialized".to_string(),
            ))
        }
    }

    async fn route_via_relay_nodes(&self, target_shard_id: u64, message: ShardMessage) -> Result<(), NetworkError> {
        for relay_node in &self.relay_nodes {
            if let Err(e) = self.send_message_to_member(relay_node.clone(), message.clone()).await {
                error!("Failed to route message via relay node {}: {}", relay_node, e);
            } else {
                return Ok(());
            }
        }
        Err(NetworkError::MessageSendingFailed(
            "Failed to route message via relay nodes".to_string(),
        ))
    }

    pub async fn set_message_sender(&mut self, sender: Sender<(u64, ShardMessage)>) {
        self.message_sender = Some(sender);
    }
}

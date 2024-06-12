use crate::smart_contract::types::{Value, TransactionContext};
use log::info;
use std::collections::{HashMap, VecDeque, HashSet};
use std::collections::hash_map::Entry;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Role {
    Admin,
    User,
    pub fn send_cross_chain_message(&self, message: CrossChainMessage) -> Result<(), String> {
        // Create IBC packet data
        let packet_data = IBCPacketData {
            sequence: self.get_next_sequence(&message.destination_chain)?,
            timeout_height: 0, // Set appropriate timeout height
            timeout_timestamp: 0, // Set appropriate timeout timestamp
            source_port: "transfer".to_string(), // Set appropriate source port
            source_channel: format!("{}/{}", message.source_chain, message.destination_chain),
            dest_port: "transfer".to_string(), // Set appropriate destination port
            dest_channel: format!("{}/{}", message.destination_chain, message.source_chain),
            data: serde_json::to_vec(&message)?,
        };

        // Update the message with packet data
        let routed_message = CrossChainMessage {
            packet_data,
            ..message
        };

        // Send the message over IBC
        info!("Sending cross-chain message over IBC: {:?}", routed_message);
        
        // TODO: Implement the actual sending logic using IBC modules

        // Handle packet acknowledgement
        self.handle_packet_ack(routed_message.packet_data)?;

        Ok(())
    }

    fn get_next_sequence(&self, destination_chain: &str) -> Result<u64, String> {
        // TODO: Implement logic to get the next sequence number for the channel
        Ok(1)
    }

    fn handle_packet_ack(&self, packet_data: IBCPacketData) -> Result<(), String> {
        // TODO: Implement packet acknowledgement handling
        Ok(())
    }

    pub fn query_cross_chain_state(&self, chain_id: &str, key: &str) -> Result<Value, String> {
        // Logic to query state from another chain
        info!("Querying state from chain {}: key {}", chain_id, key);
        // Simulate a state query result
        Ok(Value::String("mocked_state_value".to_string()))
    }

    pub fn transfer_cross_chain_assets(&self, chain_id: &str, amount: u64) -> Result<(), String> {
        // Logic to transfer assets to another chain
        info!("Transferring {} assets to chain {}", amount, chain_id);
        Ok(())
    }

pub struct ExecutionContext {
    pub state: HashMap<String, Value>,
    pub transaction_stack: VecDeque<TransactionContext>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub roles: HashMap<String, HashSet<Role>>,
}

impl ExecutionContext {
    pub fn new(gas_limit: u64) -> Self {
        ExecutionContext {
            state: HashMap::new(),
            transaction_stack: VecDeque::new(),
            gas_used: 0,
            gas_limit,
            roles: HashMap::new(),
        }
        Ok(())
    }

    pub fn add_role(&mut self, user: String, role: Role) {
        self.roles.entry(user.clone()).or_insert_with(HashSet::new).insert(role);
        info!("Added role {:?} to user {}", role, user);
    }

    pub fn has_role(&self, user: &str, role: &Role) -> bool {
        self.roles.get(user).map_or(false, |roles| roles.contains(role))
    }

    fn check_permission(&self, user: &str, required_role: Role) -> Result<(), String> {
        if self.has_role(user, &required_role) {
            Ok(())
        } else {
            Err(format!("Access denied for user: {}. Required role: {:?}", user, required_role))
        }
    }

pub struct ExecutionContext {
    pub state: HashMap<String, Value>,
    pub transaction_stack: VecDeque<TransactionContext>,
    pub gas_used: u64,
    pub gas_limit: u64,
}

impl ExecutionContext {
    pub fn new(gas_limit: u64) -> Self {
        ExecutionContext {
            state: HashMap::new(),
            transaction_stack: VecDeque::new(),
            gas_used: 0,
            gas_limit,
        }
    }

    pub fn start_transaction(&mut self) {
        self.transaction_stack.push_front(TransactionContext { changes: HashMap::new() });
    }

    pub fn commit_transaction(&mut self) {
        if let Some(tx) = self.transaction_stack.pop_front() {
            for (key, value) in tx.changes {
                self.state.insert(key, value);
            }
        }
    }

    pub fn rollback_transaction(&mut self) {
        self.transaction_stack.pop_front();
    }

    pub fn set_value(&mut self, user: &str, key: String, value: Value) -> Result<(), String> {
        self.check_permission(user, Role::Admin)?;
        if let Some(tx) = self.transaction_stack.front_mut() {
            tx.changes.insert(key, value);
        } else {
            self.state.insert(key, value);
        }
    }

    pub fn get_value(&self, user: &str, key: &str) -> Result<Option<&Value>, String> {
        self.check_permission(user, Role::User)?;
        for tx in &self.transaction_stack {
            if let Some(value) = tx.changes.get(key) {
                return Ok(Some(value));
            }
        }
        Ok(self.state.get(key))
    }

    pub fn has_value(&self, key: &str) -> bool {
        for tx in &self.transaction_stack {
            if tx.changes.contains_key(key) {
                return true;
            }
        }
        self.state.contains_key(key)
    }

    pub fn remove_value(&mut self, user: &str, key: &str) -> Result<(), String> {
        self.check_permission(user, Role::Admin)?;
        if let Some(tx) = self.transaction_stack.front_mut() {
            tx.changes.remove(key);
        } else {
            self.state.remove(key);
        }
    }

    pub fn clear_state(&mut self) {
        self.state.clear();
        self.transaction_stack.clear();
    }

    pub fn use_gas(&mut self, amount: u64) -> Result<(), String> {
        if self.gas_used + amount > self.gas_limit {
            return Err(format!("Insufficient gas. Used: {}, Limit: {}", self.gas_used, self.gas_limit));
        }
        self.gas_used += amount;
        Ok(())
    }

    pub fn rebate_gas(&mut self, amount: u64) {
        if amount <= self.gas_used {
            self.gas_used -= amount;
        } else {
            self.gas_used = 0;
        }
    }
        self.gas_used = 0;
    }

    pub fn set_gas_limit(&mut self, limit: u64) {
        self.gas_limit = limit;
    }
}

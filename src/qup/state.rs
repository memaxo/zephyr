use crate::chain::state::{Account, StateDB};
use crate::crypto::hash::Hash;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::delegator::QUPDelegator;
use crate::qup::qup_hdcmodels::QUPHDCModels;
use crate::qup::validator::QUPValidator;
use crate::storage::state_storage::StateStorage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use rayon::prelude::*;

pub struct QUPState {
    pub accounts: HashMap<String, AccountState>,
    pub blocks: Vec<QUPBlock>,
    pub config: Arc<QUPConfig>,
    pub state_db: Arc<StateDB>,
    pub delegator: Arc<QUPDelegator>,
    pub validator: Arc<QUPValidator>,
    pub hdc_models: Arc<QUPHDCModels>,
    pub state_storage: Arc<StateStorage>,
    pub network_state: Mutex<NetworkState>,
}

impl QUPState {
    pub fn prune_state(&mut self, prune_threshold: u64) {
        self.prune_old_blocks(prune_threshold);
        self.prune_old_accounts(prune_threshold);
    }

    fn prune_old_blocks(&mut self, prune_threshold: u64) {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.blocks.retain(|block| block.timestamp >= current_time - prune_threshold);
    }

    fn prune_old_accounts(&mut self, prune_threshold: u64) {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.accounts.retain(|_, account| account.last_updated >= current_time - prune_threshold);
    }

    pub fn create_snapshot(&self) -> QUPStateSnapshot {
        QUPStateSnapshot {
            accounts: self.accounts.clone(),
            blocks: self.blocks.clone(),
            network_state: self.get_network_state(),
        }
    }

    pub fn load_snapshot(&mut self, snapshot: QUPStateSnapshot) {
        self.accounts = snapshot.accounts;
        self.blocks = snapshot.blocks;
        self.update_network_state(snapshot.network_state);
    }

    pub fn create_network_state_snapshot(&self) -> NetworkStateSnapshot {
        let network_state = self.network_state.lock().unwrap();
        NetworkStateSnapshot {
            node_count: network_state.node_count,
            active_nodes: network_state.active_nodes.clone(),
            task_distribution: network_state.task_distribution.clone(),
        }
    }

    pub fn load_network_state_snapshot(&self, snapshot: NetworkStateSnapshot) {
        let mut network_state = self.network_state.lock().unwrap();
        network_state.node_count = snapshot.node_count;
        network_state.active_nodes = snapshot.active_nodes;
        network_state.task_distribution = snapshot.task_distribution;
    }

    pub fn synchronize_state(&self, other_state: &QUPState) {
        // Prune old state before synchronizing
        let mut pruned_state = self.clone();
        pruned_state.prune_state(self.config.state_pruning_threshold);

        // Collect state updates
        let mut state_updates = Vec::new();
        let mut network_state = self.network_state.lock().unwrap();
        let other_network_state = other_state.network_state.lock().unwrap();

        // Synchronize accounts
        for (id, account) in &other_state.accounts {
            state_updates.push((id.clone(), account.clone()));
        }

        // Update state in parallel
        pruned_state.update_state_parallel(&state_updates);

        // Synchronize blocks
        for block in &other_state.blocks {
            if !self.blocks.contains(block) {
                self.blocks.push(block.clone());
            }
        }

        // Synchronize network state
        network_state.node_count = network_state.node_count.max(other_network_state.node_count);
        network_state.active_nodes = network_state.active_nodes.iter().chain(other_network_state.active_nodes.iter()).cloned().collect();
        network_state.task_distribution.extend(other_network_state.task_distribution.clone());
    }

    pub fn ensure_consistency(&self) {
        let mut network_state = self.network_state.lock().unwrap();

        // Ensure all active nodes are unique
        network_state.active_nodes.sort();
        network_state.active_nodes.dedup();

        // Ensure task distribution is balanced
        let total_tasks: usize = network_state.task_distribution.values().sum();
        let average_tasks = total_tasks / network_state.node_count.max(1);

        for tasks in network_state.task_distribution.values_mut() {
            *tasks = average_tasks;
        }
    }
    pub fn new(config: Arc<QUPConfig>, state_db: Arc<StateDB>, delegator: Arc<QUPDelegator>, validator: Arc<QUPValidator>, hdc_models: Arc<QUPHDCModels>, state_storage: Arc<StateStorage>) -> Self {
        let mut state = QUPState {
            accounts: HashMap::new(),
            blocks: Vec::new(),
            config: config.clone(),
            state_db: state_db.clone(),
            delegator: delegator.clone(),
            validator: validator.clone(),
            hdc_models: hdc_models.clone(),
            state_storage: state_storage.clone(),
            network_state: Mutex::new(NetworkState::default()),
        };

        // Initialize quantum nodes
        let quantum_nodes = (0..config.quantum_node_settings.max_qubits)
            .map(|_| QuantumNode::new(config.quantum_node_settings.clone()))
            .collect();

        // Initialize classical nodes
        let classical_nodes = (0..config.network_config.node_count)
            .map(|_| ClassicalNode::new())
            .collect();

        // Initialize ParallelProcessor
        let parallel_processor = ParallelProcessor::new(quantum_nodes, classical_nodes);

        // Set up initial network state
        state.update_network_state(NetworkState {
            node_count: config.network_config.node_count,
            active_nodes: vec![],
            task_distribution: HashMap::new(),
        });

        state
    }

    pub fn execute_transactions_parallel(&mut self, transactions: &[Transaction]) {
        let chunked_transactions = transactions.par_chunks(self.config.parallel_transaction_chunk_size);

        chunked_transactions.for_each(|chunk| {
            for transaction in chunk {
                self.execute_transaction(transaction);
            }
        });
    }

    pub fn add_account(&mut self, id: String, account: Account) {
        self.accounts.insert(id, account);
    }

    pub fn add_block(&mut self, block: QUPBlock) {
        self.blocks.push(block);
    }

    pub fn update_state_parallel(&mut self, updates: &[(String, Account)]) {
        let chunked_updates = updates.par_chunks(self.config.parallel_state_update_chunk_size);

        chunked_updates.for_each(|chunk| { 
            for (id, account) in chunk {
                self.accounts.insert(id.clone(), account.clone());
            }
        });
    }

    pub fn get_account(&self, id: &str) -> Option<&Account> {
        self.accounts.get(id)
    }

    pub fn get_latest_block(&self) -> Option<&QUPBlock> {
        self.blocks.last()
    }

    pub fn update_network_state(&self, new_state: NetworkState) {
        let mut network_state = self.network_state.lock().unwrap();
        *network_state = new_state;
    }

    pub fn get_network_state(&self) -> NetworkState {
        let network_state = self.network_state.lock().unwrap();
        network_state.clone()
    }
}

#[derive(Clone)]
pub struct NetworkStateSnapshot {
    pub node_count: usize,
    pub active_nodes: Vec<String>,
    pub task_distribution: HashMap<String, usize>,
}
#[derive(Clone)]
pub struct QUPStateSnapshot {
    pub accounts: HashMap<String, Account>,
    pub blocks: Vec<QUPBlock>,
    pub network_state: NetworkState,
}

#[derive(Clone)]
pub struct NetworkStateSnapshot {
    pub node_count: usize,
    pub active_nodes: Vec<String>,
    pub task_distribution: HashMap<String, usize>,
}
pub struct NetworkState {
    pub node_count: usize,
    pub active_nodes: Vec<String>,
    pub task_distribution: HashMap<String, usize>,
}

use std::collections::HashMap;

#[derive(Clone)]
pub struct AccountState {
    pub balance: u64,
    pub nonce: u64,
    pub storage: HashMap<String, String>,
}

impl AccountState {
    pub fn new(balance: u64) -> Self {
        AccountState {
            balance,
            nonce: 0,
            storage: HashMap::new(),
        }
    }
}

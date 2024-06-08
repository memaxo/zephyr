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

pub struct QUPState {
    pub accounts: HashMap<String, Account>,
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
    pub fn new(config: Arc<QUPConfig>, state_db: Arc<StateDB>, delegator: Arc<QUPDelegator>, validator: Arc<QUPValidator>, hdc_models: Arc<QUPHDCModels>, state_storage: Arc<StateStorage>) -> Self {
        QUPState {
            accounts: HashMap::new(),
            blocks: Vec::new(),
            config,
            state_db,
            delegator,
            validator,
            hdc_models,
            state_storage,
            network_state: Mutex::new(NetworkState::default()),
        }
    }

    pub fn add_account(&mut self, id: String, account: Account) {
        self.accounts.insert(id, account);
    }

    pub fn add_block(&mut self, block: QUPBlock) {
        self.blocks.push(block);
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

#[derive(Clone, Default)]
pub struct NetworkState {
    pub node_count: usize,
    pub active_nodes: Vec<String>,
    pub task_distribution: HashMap<String, usize>,
}


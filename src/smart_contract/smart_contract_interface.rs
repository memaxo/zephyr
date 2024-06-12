use crate::smart_contract::types::{SmartContract, ContractState};
use crate::utils::error::Result;
use crate::smart_contract::logging::init_logging;
use log::info;

pub trait SmartContractInterface {
    fn deploy_contract(&self, contract: SmartContract) -> Result<String>;
    fn execute_contract(&self, contract_id: &str, function_name: &str, args: &[u8]) -> Result<Vec<u8>>;
    fn get_contract_state(&self, contract_id: &str) -> Result<ContractState>;
    fn upgrade_contract(&self, contract_id: &str, new_code: &str) -> Result<()>;
    fn get_proxy_contract(&self, contract_id: &str) -> Result<String>;
    fn send_cross_chain_message(&self, message: CrossChainMessage) -> Result<()>;
    fn query_cross_chain_state(&self, chain_id: &str, key: &str) -> Result<Value>;
    fn transfer_cross_chain_assets(&self, chain_id: &str, amount: u64) -> Result<()>;
}

pub struct ProxyContract {
    pub target_contract_id: String,
}

impl ProxyContract {
    pub fn new(target_contract_id: String) -> Self {
        let proxy_contract = ProxyContract { target_contract_id };
        info!("ProxyContract created with target_contract_id: {}", target_contract_id);
        proxy_contract
    }

    pub fn upgrade(&mut self, new_target_contract_id: String) {
        self.target_contract_id = new_target_contract_id;
    }
}

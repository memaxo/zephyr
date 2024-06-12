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

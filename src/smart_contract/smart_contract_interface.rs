use crate::smart_contract::types::{SmartContract, ContractState};
use crate::utils::error::Result;

pub trait SmartContractInterface {
    fn deploy_contract(&self, contract: SmartContract) -> Result<String>;
    fn execute_contract(&self, contract_id: &str, function_name: &str, args: &[u8]) -> Result<Vec<u8>>;
    fn get_contract_state(&self, contract_id: &str) -> Result<ContractState>;
}

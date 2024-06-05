use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DeployContractRequest {
    pub bytecode: String,
    pub constructor_args: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CallContractRequest {
    pub contract_address: String,
    pub function_name: String,
    pub function_args: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetContractStateRequest {
    pub contract_address: String,
}

#[derive(Deserialize, Debug)]
pub struct GetContractRequest {
    pub contract_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractFunctionInput {
    pub name: String,
    pub data_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployedContract {
    pub address: String,
    pub bytecode: String,
    pub constructor_args: Vec<ContractFunctionInput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractState {
    pub variables: Vec<(String, String)>,
}
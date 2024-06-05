use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetValidatorsResponse {
    pub validators: Vec<ValidatorInfo>,
}

#[derive(Serialize, Debug)]
pub struct GetConsensusStateResponse {
    pub qup_state: QUPState,
}

#[derive(Serialize, Debug)]
pub struct GetConsensusParametersResponse {
    pub qup_parameters: QUPParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorInfo {
    pub public_key: String,
    pub stake: u64,
    pub useful_work_score: f64,
    // Add other validator fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QUPState {
    pub current_epoch: u64,
    pub validators: Vec<String>, // List of validator public keys
    pub total_stake: u64,
    pub useful_work_threshold: u64,
    // Add other QUP state fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QUPParameters {
    pub epoch_length: u64,
    pub minimum_stake: u64,
    pub voting_threshold: u64,
    pub useful_work_difficulty: u64,
    pub validator_reward_ratio: f64,
    pub delegator_reward_ratio: f64,
    // Add other QUP parameters as needed
}

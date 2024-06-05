use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetValidatorsRequest {}

#[derive(Deserialize, Debug)]
pub struct GetConsensusStateRequest {}

#[derive(Deserialize, Debug)]
pub struct GetConsensusParametersRequest {}

#[derive(Deserialize, Debug)]
pub struct UpdateConsensusParametersRequest {
    pub qup_parameters: QUPParameters,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct QUPParameters {
    pub epoch_length: u64,
    pub minimum_stake: u64,
    pub voting_threshold: u64,
    pub useful_work_difficulty: u64,
    // Add other QUP parameters as needed
}

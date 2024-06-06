use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::chain::consensus::ChainConsensus;
use crate::chain::validator::ChainValidator;

pub async fn get_consensus_state(qup_consensus: web::Data<QUPConsensus>) -> impl Responder {
    let state = ChainState {
        current_epoch: qup_consensus.current_epoch(),
        validators: qup_consensus
            .validators()
            .iter()
            .map(|v| v.public_key().to_string())
            .collect(),
        // Add other relevant QUP state fields
    };

    HttpResponse::Ok().json(state)
}

pub async fn get_validators(qup_consensus: web::Data<QUPConsensus>) -> impl Responder {
    let validators = qup_consensus
        .validators()
        .iter()
        .map(|v| ValidatorInfo::from_validator(v))
        .collect();

    HttpResponse::Ok().json(validators)
}

pub async fn get_consensus_parameters(qup_consensus: web::Data<QUPConsensus>) -> impl Responder {
    let parameters = QUPParameters {
        epoch_length: qup_consensus.epoch_length(),
        minimum_stake: qup_consensus.minimum_stake(),
        voting_threshold: qup_consensus.voting_threshold(),
        useful_work_difficulty: qup_consensus.useful_work_difficulty(),
        // Add other relevant QUP parameters
    };

    HttpResponse::Ok().json(parameters)
}

#[derive(Serialize)]
struct ChainState {
    current_epoch: u64,
    validators: Vec<String>, // list of validator public keys
                             // Add other relevant QUP state fields
}

#[derive(Serialize)]
struct ValidatorInfo {
    public_key: String,
    stake: u64,
    useful_work_score: f64,
    // Add other relevant validator fields
}

impl ValidatorInfo {
    fn from_validator(validator: &ChainValidator) -> Self {
        Self {
            public_key: validator.public_key().to_string(),
            stake: validator.stake(),
            useful_work_score: validator.useful_work_score(),
            // Add other relevant validator fields
        }
    }
}

#[derive(Serialize)]
struct ChainParameters {
    epoch_length: u64,
    minimum_stake: u64,
    voting_threshold: u64,
    useful_work_difficulty: u64,
    // Add other relevant QUP parameters
}

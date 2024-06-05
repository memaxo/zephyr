use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::consensus::{ConsensusEngine, PoUW, QDPoS};

pub fn consensus_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/consensus")
            .route("/state", web::get().to(get_consensus_state))
            .route("/validators", web::get().to(get_validators))
            .route("/parameters", web::get().to(get_consensus_parameters)),
    );
}

#[derive(Serialize, Deserialize)]
struct ConsensusState {
    consensus_type: String,
    pouw_state: Option<PoUWState>,
    qdpos_state: Option<QDPoSState>,
}

#[derive(Serialize, Deserialize)]
struct PoUWState {
    current_round: u64,
    winning_model_hash: String,
    // Add other relevant PoUW state fields
}

#[derive(Serialize, Deserialize)]
struct QDPoSState {
    current_epoch: u64,
    validators: Vec<String>, // list of validator public keys
    // Add other relevant QDPoS state fields
}

#[derive(Serialize, Deserialize)]
struct Validator {
    public_key: String,
    stake: f64,
    // Add other relevant validator fields
}

#[derive(Serialize, Deserialize)]
struct ConsensusParameters {
    pouw: Option<PoUWParameters>,
    qdpos: Option<QDPoSParameters>,
}

#[derive(Serialize, Deserialize)]
struct PoUWParameters {
    difficulty_adjustment_interval: u64,
    // Add other relevant PoUW parameters
}

#[derive(Serialize, Deserialize)]
struct QDPoSParameters {
    epoch_length: u64,
    minimum_stake: f64,
    // Add other relevant QDPoS parameters
}

async fn get_consensus_state(consensus_engine: web::Data<ConsensusEngine>) -> impl Responder {
    let state = match consensus_engine.consensus_type() {
        ConsensusType::PoUW => {
            let pouw = consensus_engine.pouw().unwrap();
            ConsensusState {
                consensus_type: "PoUW".to_string(),
                pouw_state: Some(PoUWState {
                    current_round: pouw.current_round(),
                    winning_model_hash: pouw.winning_model_hash().to_string(),
                    // Add other relevant PoUW state fields
                }),
                qdpos_state: None,
            }
        }
        ConsensusType::QDPoS => {
            let qdpos = consensus_engine.qdpos().unwrap();
            ConsensusState {
                consensus_type: "QDPoS".to_string(),
                pouw_state: None,
                qdpos_state: Some(QDPoSState {
                    current_epoch: qdpos.current_epoch(),
                    validators: qdpos
                        .validators()
                        .iter()
                        .map(|v| v.public_key().to_string())
                        .collect(),
                    // Add other relevant QDPoS state fields
                }),
            }
        }
    };

    HttpResponse::Ok().json(state)
}

async fn get_validators(consensus_engine: web::Data<ConsensusEngine>) -> impl Responder {
    let validators = match consensus_engine.consensus_type() {
        ConsensusType::PoUW => Vec::new(), // PoUW doesn't have validators
        ConsensusType::QDPoS => {
            let qdpos = consensus_engine.qdpos().unwrap();
            qdpos
                .validators()
                .iter()
                .map(|v| Validator {
                    public_key: v.public_key().to_string(),
                    stake: v.stake(),
                    // Add other relevant validator fields
                })
                .collect()
        }
    };

    HttpResponse::Ok().json(validators)
}

async fn get_consensus_parameters(
    consensus_engine: web::Data<ConsensusEngine>,
) -> impl Responder {
    let parameters = match consensus_engine.consensus_type() {
        ConsensusType::PoUW => ConsensusParameters {
            pouw: Some(PoUWParameters {
                difficulty_adjustment_interval: consensus_engine.pouw().unwrap().difficulty_adjustment_interval(),
                // Add other relevant PoUW parameters
            }),
            qdpos: None,
        },
        ConsensusType::QDPoS => ConsensusParameters {
            pouw: None,
            qdpos: Some(QDPoSParameters {
                epoch_length: consensus_engine.qdpos().unwrap().epoch_length(),
                minimum_stake: consensus_engine.qdpos().unwrap().minimum_stake(),
                // Add other relevant QDPoS parameters
            }),
        },
    };

    HttpResponse::Ok().json(parameters)
}
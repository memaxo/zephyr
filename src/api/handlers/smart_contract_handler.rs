use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::smart_contract::{SmartContractEngine, SmartContract, ContractState};

pub async fn deploy_contract(
    smart_contract_engine: web::Data<SmartContractEngine>,
    deploy_request: web::Json<DeployRequest>,
) -> impl Responder {
    let bytecode = hex::decode(&deploy_request.bytecode).map_err(|_| {
        HttpResponse::BadRequest().body("Invalid bytecode")
    })?;

    let contract = smart_contract_engine
        .deploy(bytecode, &deploy_request.constructor_args)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to deploy contract"))?;

    let contract_info = ContractInfo {
        address: contract.address().to_string(),
        bytecode: deploy_request.bytecode.clone(),
        constructor_args: deploy_request.constructor_args.clone(),
    };

    HttpResponse::Ok().json(contract_info)
}

pub async fn get_contract(
    smart_contract_engine: web::Data<SmartContractEngine>,
    address: web::Path<String>,
) -> impl Responder {
    let contract = smart_contract_engine
        .get_contract(&address.into_inner())
        .map_err(|_| HttpResponse::NotFound().body("Contract not found"))?;

    let contract_info = ContractInfo {
        address: contract.address().to_string(),
        bytecode: hex::encode(&contract.bytecode()),
        constructor_args: contract.constructor_args().to_vec(),
    };

    HttpResponse::Ok().json(contract_info)
}

pub async fn call_contract(
    smart_contract_engine: web::Data<SmartContractEngine>,
    address: web::Path<String>,
    call_request: web::Json<CallRequest>,
) -> impl Responder {
    let result = smart_contract_engine
        .call_contract(&address.into_inner(), &call_request.method, &call_request.args)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to call contract"))?;

    HttpResponse::Ok().json(result)
}

pub async fn get_contract_state(
    smart_contract_engine: web::Data<SmartContractEngine>,
    address: web::Path<String>,
) -> impl Responder {
    let state = smart_contract_engine
        .get_contract_state(&address.into_inner())
        .map_err(|_| HttpResponse::NotFound().body("Contract not found"))?;

    let contract_state = ContractState {
        variables: state
            .variables()
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
    };

    HttpResponse::Ok().json(contract_state)
}

#[derive(Deserialize)]
struct DeployRequest {
    bytecode: String,
    constructor_args: Vec<String>,
}

#[derive(Serialize)]
struct ContractInfo {
    address: String,
    bytecode: String,
    constructor_args: Vec<String>,
}

#[derive(Deserialize)]
struct CallRequest {
    method: String,
    args: Vec<String>,
}

#[derive(Serialize)]
struct ContractState {
    variables: Vec<(String, String)>,
}
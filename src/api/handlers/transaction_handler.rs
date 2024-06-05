use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::chain::{TransactionPool, Blockchain, Transaction, TransactionStatus};

pub async fn submit_transaction(
    transaction_pool: web::Data<TransactionPool>,
    blockchain: web::Data<Blockchain>,
    request: web::Json<SubmitTransactionRequest>,
) -> impl Responder {
    let signature = hex::decode(&request.signature).map_err(|_| {
        HttpResponse::BadRequest().body("Invalid signature format")
    })?;

    let transaction = Transaction {
        sender: request.sender.clone(),
        recipient: request.recipient.clone(),
        amount: request.amount,
        signature,
        // Add other transaction fields as needed
    };

    if transaction.validate(&blockchain).is_err() {
        return HttpResponse::BadRequest().body("Invalid transaction");
    }

    transaction_pool.add_transaction(transaction).map_err(|_| {
        HttpResponse::InternalServerError().body("Failed to submit transaction")
    })?;

    HttpResponse::Ok().body("Transaction submitted successfully")
}

pub async fn get_transaction_status(
    blockchain: web::Data<Blockchain>,
    tx_hash: web::Path<String>,
) -> impl Responder {
    let transaction_status = blockchain
        .get_transaction_status(&tx_hash.into_inner())
        .map_err(|_| HttpResponse::NotFound().body("Transaction not found"))?;

    let response = TransactionStatusResponse {
        hash: tx_hash.into_inner(),
        status: format!("{:?}", transaction_status.status),
        block_number: transaction_status.block_number,
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_transaction_history(
    blockchain: web::Data<Blockchain>,
) -> impl Responder {
    let transactions: Vec<TransactionInfo> = blockchain
        .iter_transactions()
        .map(|tx| TransactionInfo::from_transaction(tx))
        .collect();

    let response = TransactionHistoryResponse { transactions };

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct SubmitTransactionRequest {
    sender: String,
    recipient: String,
    amount: f64,
    signature: String,
    // Add other transaction fields as needed
}

#[derive(Serialize)]
struct TransactionStatusResponse {
    hash: String,
    status: String,
    block_number: Option<u64>,
}

#[derive(Serialize)]
struct TransactionHistoryResponse {
    transactions: Vec<TransactionInfo>,
}

#[derive(Serialize)]
struct TransactionInfo {
    hash: String,
    sender: String,
    recipient: String,
    amount: f64,
    block_number: u64,
    timestamp: u64,
}

impl TransactionInfo {
    fn from_transaction(tx: &Transaction) -> Self {
        Self {
            hash: tx.hash().to_string(),
            sender: tx.sender().to_string(),
            recipient: tx.recipient().to_string(),
            amount: tx.amount(),
            block_number: tx.block_number(),
            timestamp: tx.timestamp(),
        }
    }
}
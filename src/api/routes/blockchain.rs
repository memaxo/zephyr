use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::chain::blockchain::Blockchain;

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    hash: String,
    height: u64,
    timestamp: u64,
    transactions_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionInfo {
    hash: String,
    block_height: u64,
    timestamp: u64,
    sender: String,
    receiver: String,
    amount: f64,
}

pub fn blockchain_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blockchain")
            .route("/blocks", web::get().to(get_blocks))
            .route("/blocks/{height}", web::get().to(get_block))
            .route("/transactions", web::get().to(get_transactions))
            .route("/transactions/{hash}", web::get().to(get_transaction))
            .route("/status", web::get().to(get_chain_status)),
    );
}

async fn get_blocks(blockchain: web::Data<Blockchain>) -> impl Responder {
    let blocks: Vec<BlockInfo> = blockchain
        .iter()
        .map(|block| BlockInfo {
            hash: block.hash(),
            height: block.height,
            timestamp: block.timestamp,
            transactions_count: block.transactions.len(),
        })
        .collect();

    HttpResponse::Ok().json(blocks)
}

async fn get_block(blockchain: web::Data<Blockchain>, height: web::Path<u64>) -> impl Responder {
    let block = blockchain.get_block(height.into_inner());
    if let Some(block) = block {
        let block_info = BlockInfo {
            hash: block.hash(),
            height: block.height,
            timestamp: block.timestamp,
            transactions_count: block.transactions.len(),
        };
        HttpResponse::Ok().json(block_info)
    } else {
        HttpResponse::NotFound().body("Block not found")
    }
}

async fn get_transactions(blockchain: web::Data<Blockchain>) -> impl Responder {
    let transactions: Vec<TransactionInfo> = blockchain
        .iter()
        .flat_map(|block| {
            block
                .transactions
                .iter()
                .map(|tx| TransactionInfo {
                    hash: tx.hash(),
                    block_height: block.height,
                    timestamp: block.timestamp,
                    sender: tx.sender.to_string(),
                    receiver: tx.receiver.to_string(),
                    amount: tx.amount,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    HttpResponse::Ok().json(transactions)
}

async fn get_transaction(
    blockchain: web::Data<Blockchain>,
    hash: web::Path<String>,
) -> impl Responder {
    let transaction = blockchain.find_transaction(&hash.into_inner());
    if let Some(tx) = transaction {
        let block = blockchain.get_block(tx.block_height);
        if let Some(block) = block {
            let tx_info = TransactionInfo {
                hash: tx.hash(),
                block_height: tx.block_height,
                timestamp: block.timestamp,
                sender: tx.sender.to_string(),
                receiver: tx.receiver.to_string(),
                amount: tx.amount,
            };
            HttpResponse::Ok().json(tx_info)
        } else {
            HttpResponse::NotFound().body("Transaction not found")
        }
    } else {
        HttpResponse::NotFound().body("Transaction not found")
    }
}

async fn get_chain_status(blockchain: web::Data<Blockchain>) -> impl Responder {
    let chain_status = ChainStatus {
        height: blockchain.height(),
        difficulty: blockchain.difficulty(),
        // add more chain status fields as needed
    };

    HttpResponse::Ok().json(chain_status)
}

#[derive(Serialize, Deserialize)]
struct ChainStatus {
    height: u64,
    difficulty: u64,
    // add more chain status fields as needed
}
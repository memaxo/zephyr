use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::chain::blockchain::Blockchain;
use crate::chain::block::Block;
use crate::chain::transaction::Transaction;

pub async fn get_blocks(blockchain: web::Data<Blockchain>) -> impl Responder {
    let blocks: Vec<BlockInfo> = blockchain
        .iter()
        .map(|block| BlockInfo::from_block(block))
        .collect();

    HttpResponse::Ok().json(blocks)
}

pub async fn get_block(
    blockchain: web::Data<Blockchain>,
    height: web::Path<u64>,
) -> impl Responder {
    let block = blockchain.get_block(height.into_inner());
    if let Some(block) = block {
        let block_info = BlockInfo::from_block(&block);
        HttpResponse::Ok().json(block_info)
    } else {
        HttpResponse::NotFound().body("Block not found")
    }
}

pub async fn get_transactions(
    blockchain: web::Data<Blockchain>,
) -> impl Responder {
    let transactions: Vec<TransactionInfo> = blockchain
        .iter()
        .flat_map(|block| {
            block
                .transactions
                .iter()
                .map(|tx| TransactionInfo::from_transaction(tx, block.height, block.timestamp))
                .collect::<Vec<_>>()
        })
        .collect();

    HttpResponse::Ok().json(transactions)
}

pub async fn get_transaction(
    blockchain: web::Data<Blockchain>,
    hash: web::Path<String>,
) -> impl Responder {
    let transaction = blockchain.find_transaction(&hash.into_inner());
    if let Some(tx) = transaction {
        let block = blockchain.get_block(tx.block_height);
        if let Some(block) = block {
            let tx_info = TransactionInfo::from_transaction(&tx, block.height, block.timestamp);
            HttpResponse::Ok().json(tx_info)
        } else {
            HttpResponse::NotFound().body("Transaction not found")
        }
    } else {
        HttpResponse::NotFound().body("Transaction not found")
    }
}

pub async fn get_chain_status(
    blockchain: web::Data<Blockchain>,
) -> impl Responder {
    let chain_status = ChainStatus {
        height: blockchain.height(),
        difficulty: blockchain.difficulty(),
        // add more chain status fields as needed
    };

    HttpResponse::Ok().json(chain_status)
}

#[derive(Serialize)]
struct BlockInfo {
    hash: String,
    height: u64,
    timestamp: u64,
    transactions_count: usize,
}

impl BlockInfo {
    fn from_block(block: &Block) -> Self {
        Self {
            hash: block.hash(),
            height: block.height,
            timestamp: block.timestamp,
            transactions_count: block.transactions.len(),
        }
    }
}

#[derive(Serialize)]
struct TransactionInfo {
    hash: String,
    block_height: u64,
    timestamp: u64,
    sender: String,
    recipient: String,
    amount: f64,
}

impl TransactionInfo {
    fn from_transaction(
        tx: &Transaction,
        block_height: u64,
        block_timestamp: u64,
    ) -> Self {
        Self {
            hash: tx.hash(),
            block_height,
            timestamp: block_timestamp,
            sender: tx.sender.to_string(),
            recipient: tx.recipient.to_string(),
            amount: tx.amount,
        }
    }
}

#[derive(Serialize)]
struct ChainStatus {
    height: u64,
    difficulty: u64,
    // add more chain status fields as needed
}
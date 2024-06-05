use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetBlockRequest {
    pub block_hash: Option<String>,
    pub block_height: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct SubmitBlockRequest {
    pub block: BlockData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct BlockData {
    pub header: BlockHeader,
    pub transactions: Vec<TransactionData>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub difficulty: u64,
    pub nonce: u64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TransactionData {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub signature: String,
    // Add other transaction fields as needed
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionsRequest {
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct SubmitTransactionRequest {
    pub transaction: TransactionData,
}

#[derive(Deserialize, Debug)]
pub struct GetChainInfoRequest {}
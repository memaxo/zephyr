use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct GetBlockResponse {
    pub block: BlockData,
}

#[derive(Serialize, Debug)]
pub struct SubmitBlockResponse {
    pub block_hash: String,
}

#[derive(Serialize, Debug)]
pub struct GetTransactionsResponse {
    pub transactions: Vec<TransactionData>,
}

#[derive(Serialize, Debug)]
pub struct GetTransactionResponse {
    pub transaction: TransactionData,
}

#[derive(Serialize, Debug)]
pub struct GetChainInfoResponse {
    pub height: u64,
    pub difficulty: u64,
    pub total_transactions: u64,
    // Add other chain information fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData {
    pub header: BlockHeader,
    pub transactions: Vec<TransactionData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub difficulty: u64,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub hash: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: u64,
    // Add other transaction fields as needed
}
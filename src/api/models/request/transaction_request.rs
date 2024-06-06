use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SubmitTransactionRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub signature: String,
    // Add other transaction fields as needed
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionRequest {
    pub transaction_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionsRequest {
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub sender: Option<String>,
    pub recipient: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub block_number: u64,
    pub timestamp: u64,
    pub useful_work_solution: Option<UsefulWorkSolution>,
    pub history_proof: Option<HistoryProof>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionStatus {
    pub transaction_hash: String,
    pub status: TransactionStatusType,
    pub block_number: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionStatusType {
    Pending,
    Confirmed,
    Failed,
}

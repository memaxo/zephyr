use crate::chain::transaction::Transaction;
use crate::crypto::hash::Hash;
use crate::qup::crypto::QUPSignature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsefulWorkProblem {
    Knapsack(KnapsackProblem),
    VertexCover(VertexCoverProblem),
    // Add more useful work problem types as needed
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnapsackProblem {
    pub capacity: u64,
    pub weights: Vec<u64>,
    pub values: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexCoverProblem {
    pub graph: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsefulWorkSolution {
    Knapsack(KnapsackSolution),
    VertexCover(VertexCoverSolution),
    // Add more useful work solution types as needed
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnapsackSolution {
    pub selected_items: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexCoverSolution {
    pub vertex_cover: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QUPBlockHeader {
    pub version: u32,
    pub prev_block_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub difficulty: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QUPTransaction {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
    pub signature: QUPSignature,
    // Add more transaction-specific fields as needed
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QUPVote {
    pub voter: Vec<u8>,
    pub block_hash: Hash,
    pub signature: QUPSignature,
}

// Add more types and structs specific to the QUP module as needed

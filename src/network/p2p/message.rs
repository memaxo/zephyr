use crate::chain::block::Block;
use crate::chain::transaction::Transaction;
use crate::crypto::hash::Hasher;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Ping,
    Pong,
    GetPeers,
    Peers(Vec<String>),
    Transaction(Transaction),
    Block(Block),
    GetBlocks(Hasher),
    Blocks(Vec<Block>),
    GetState(u64),
    State(Vec<u8>),
}

impl Message {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}
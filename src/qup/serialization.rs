use crate::qup::types::{QuantumBlock, QuantumTransaction};
use bincode;
use serde_json;

pub fn serialize_quantum_transaction(transaction: &QuantumTransaction) -> Vec<u8> {
    bincode::serialize(transaction).expect("Failed to serialize QuantumTransaction")
}

pub fn deserialize_quantum_transaction(data: &[u8]) -> QuantumTransaction {
    bincode::deserialize(data).expect("Failed to deserialize QuantumTransaction")
}

pub fn serialize_quantum_block(block: &QuantumBlock) -> Vec<u8> {
    bincode::serialize(block).expect("Failed to serialize QuantumBlock")
}

pub fn deserialize_quantum_block(data: &[u8]) -> QuantumBlock {
    bincode::deserialize(data).expect("Failed to deserialize QuantumBlock")
}

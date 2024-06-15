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
use crate::hdcmodels::hdcmodels::HDCModel;
use crate::error::ConsensusError;
use async_compression::tokio::bufread::{GzipEncoder, GzipDecoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn compress_data(data: &[u8]) -> Result<Vec<u8>, ConsensusError> {
    let mut encoder = GzipEncoder::new(data);
    let mut compressed_data = Vec::new();
    encoder.read_to_end(&mut compressed_data).await?;
    Ok(compressed_data)
}

pub async fn decompress_data(data: &[u8]) -> Result<Vec<u8>, ConsensusError> {
    let mut decoder = GzipDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).await?;
    Ok(decompressed_data)
}

pub fn serialize_hdc_model(model: &HDCModel) -> Result<Vec<u8>, ConsensusError> {
    bincode::serialize(model).map_err(|e| ConsensusError::SerializationError(e.to_string()))
}

pub fn deserialize_hdc_model(data: &[u8]) -> Result<HDCModel, ConsensusError> {
    bincode::deserialize(data).map_err(|e| ConsensusError::DeserializationError(e.to_string()))
}

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
}

pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
    let compressed_data = encoder.finish().map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
    Ok(compressed_data)
}

pub fn decompress_data(compressed_data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;
    Ok(decompressed_data)
}

pub fn compress_data_zstd(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let compressed_data = zstd::encode_all(data, 0).map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
    Ok(compressed_data)
}

pub fn decompress_data_zstd(compressed_data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let decompressed_data = zstd::decode_all(compressed_data).map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;
    Ok(decompressed_data)
}

pub fn compress_data_lz4(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut compressed_data = Vec::new();
    lz4::block::compress_to_vec(data, &mut compressed_data, lz4::block::CompressionMode::DEFAULT)?;
    Ok(compressed_data)
}

pub fn decompress_data_lz4(compressed_data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut decompressed_data = Vec::new();
    lz4::block::decompress_to_vec(compressed_data, &mut decompressed_data)?;
    Ok(decompressed_data)
}
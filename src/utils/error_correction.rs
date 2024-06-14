use sha2::{Sha256, Digest};
use reed_solomon_erasure::galois_8::ReedSolomon;
use std::error::Error;

pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn verify_checksum(data: &[u8], checksum: &[u8]) -> bool {
    let calculated_checksum = calculate_checksum(data);
    calculated_checksum == checksum
}

pub fn correct_errors(data: &mut [u8], checksum: &[u8]) -> Result<(), Box<dyn Error>> {
    let rs = ReedSolomon::new(255, 1)?; // Example parameters, adjust as needed
    let mut shards: Vec<&mut [u8]> = data.chunks_mut(255).collect();
    rs.reconstruct(&mut shards)?;

    if verify_checksum(data, checksum) {
        Ok(())
    } else {
        Err("Error correction failed".into())
    }
}

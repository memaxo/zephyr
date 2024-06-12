use crate::chain::block::{Block, QUPBlock};
use crate::qup::crypto::QUPCrypto;
use crate::qup::state::QUPState;
use crate::chain::validation::{ValidationError, validate_transaction, validate_smart_contracts};
use anyhow::Result;
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

const BLOCK_TIME_WINDOW: Duration = Duration::from_secs(30);
const REQUIRED_TX_VALID_RATIO: f32 = 0.66;

pub fn validate_block(
    block: &Block, 
    qup_crypto: &QUPCrypto,
    qup_state: &QUPState,
) -> Result<(), ValidationError> {
    // Verify the block hash
    if !block.verify_hash() {
        return Err(ValidationError::InvalidBlockHash);
    }
    
    // Verify the block proposer's signature
    validate_block_signature(block, qup_crypto)?;

    // Verify the block timestamp 
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let block_time = Duration::from_secs(block.timestamp);
    if current_time.saturating_sub(block_time) > BLOCK_TIME_WINDOW {
        return Err(ValidationError::BlockTimeExceedsAllowedWindow);
    }
    
    // Validate transactions in parallel
    let valid_tx_count = block.transactions
        .par_iter()
        .map(|tx| validate_transaction(tx, qup_crypto, qup_state))
        .filter(|r| r.is_ok())
        .count();
        
    let total_txs = block.transactions.len();
    if valid_tx_count < (REQUIRED_TX_VALID_RATIO * total_txs as f32) as usize {
        return Err(ValidationError::InsufficientValidTransactions {
            valid: valid_tx_count,
            total: total_txs,
        });
    }
    
    validate_smart_contracts(&block.smart_contracts)?;

    Ok(())
}

pub fn validate_qup_block(
    block: &QUPBlock,
    qup_crypto: &QUPCrypto, 
    qup_state: &QUPState,
) -> Result<(), ValidationError> {
    // Implement QUP block validation logic here
    Ok(())
}

fn validate_block_signature(
    block: &Block,
    qup_crypto: &QUPCrypto,
) -> Result<(), ValidationError> {
    // Verify the block proposer's signature using qup_crypto
    // ...
    Ok(())
}
use crate::chain::block::{Block, QUPBlock};
use crate::qup::crypto::QUPCrypto;
use crate::qup::state::QUPState;
use crate::chain::validation::{ValidationError, validate_transaction, validate_smart_contracts};
use anyhow::Result;
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

const BLOCK_TIME_WINDOW: Duration = Duration::from_secs(30);
const REQUIRED_TX_VALID_RATIO: f32 = 0.66;

pub fn validate_block(
    block: &Block, 
    qup_crypto: &QUPCrypto,
    qup_state: &QUPState,
) -> Result<(), ValidationError> {
    // Verify the block hash
    if !block.verify_hash() {
        return Err(ValidationError::InvalidBlockHash);
    }
    
    // Verify the block proposer's signature
    validate_block_signature(block, qup_crypto)?;

    // Verify the block timestamp 
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let block_time = Duration::from_secs(block.timestamp);
    if current_time.saturating_sub(block_time) > BLOCK_TIME_WINDOW {
        return Err(ValidationError::BlockTimeExceedsAllowedWindow);
    }
    
    // Validate transactions in parallel
    let valid_tx_count = block.transactions
        .par_iter()
        .map(|tx| validate_transaction(tx, qup_crypto, qup_state))
        .filter(|r| r.is_ok())
        .count();
        
    let total_txs = block.transactions.len();
    if valid_tx_count < (REQUIRED_TX_VALID_RATIO * total_txs as f32) as usize {
        return Err(ValidationError::InsufficientValidTransactions {
            valid: valid_tx_count,
            total: total_txs,
        });
    }
    
    validate_smart_contracts(&block.smart_contracts)?;

    Ok(())
}

pub fn validate_qup_block(
    block: &QUPBlock,
    qup_crypto: &QUPCrypto, 
    qup_state: &QUPState,
) -> Result<(), ValidationError> {
    // Implement QUP block validation logic here
    Ok(())
}

fn validate_block_signature(
    block: &Block,
    qup_crypto: &QUPCrypto,
) -> Result<(), ValidationError> {
    // Verify the block proposer's signature using qup_crypto
    // ...
    Ok(())
}

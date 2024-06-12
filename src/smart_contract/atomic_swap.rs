use crate::smart_contract::types::{Value, ExecutionContext};
use log::info;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AtomicSwap {
    pub hash: [u8; 32],
    pub recipient: String,
    pub expiration: u64,
    pub amount: u64,
}

impl AtomicSwap {
    pub fn new(secret: &[u8], recipient: String, expiration: u64, amount: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let hash = hasher.finalize();
        
        AtomicSwap {
            hash: hash.into(),
            recipient,
            expiration,
            amount,
        }
    }
    
    pub fn lock(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let key = format!("atomic_swap_{}", hex::encode(&self.hash));
        
        if context.has_value(&key) {
            return Err("Atomic swap already exists".to_string());
        }
        
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        if now >= self.expiration {
            return Err("Atomic swap has already expired".to_string());
        }
        
        context.set_value("system", key, Value::Integer(self.amount as i64))?;
        
        info!("Locked {} assets in atomic swap with hash {}", self.amount, hex::encode(&self.hash));
        
        Ok(())
    }
    
    pub fn unlock(&self, secret: &[u8], context: &mut ExecutionContext) -> Result<(), String> {
        let key = format!("atomic_swap_{}", hex::encode(&self.hash));
        
        if !context.has_value(&key) {
            return Err("Atomic swap does not exist".to_string());
        }
        
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let hash = hasher.finalize();
        
        if hash.as_slice() != self.hash {
            return Err("Invalid secret for atomic swap".to_string());
        }
        
        let amount = match context.get_value("system", &key)? {
            Some(Value::Integer(amount)) => *amount as u64,
            _ => return Err("Invalid atomic swap value".to_string()),
        };
        
        context.remove_value("system", &key)?;
        context.transfer_cross_chain_assets(&self.recipient, amount)?;
        
        info!("Unlocked {} assets from atomic swap with hash {}", amount, hex::encode(&self.hash));
        
        Ok(())
    }
    
    pub fn refund(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let key = format!("atomic_swap_{}", hex::encode(&self.hash));
        
        if !context.has_value(&key) {
            return Err("Atomic swap does not exist".to_string());
        }
        
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        if now < self.expiration {
            return Err("Atomic swap has not expired yet".to_string());
        }
        
        let amount = match context.get_value("system", &key)? {
            Some(Value::Integer(amount)) => *amount as u64,
            _ => return Err("Invalid atomic swap value".to_string()),
        };
        
        context.remove_value("system", &key)?;
        
        info!("Refunded {} assets from expired atomic swap with hash {}", amount, hex::encode(&self.hash));
        
        Ok(())
    }
}

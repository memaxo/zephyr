use crate::chain::Blockchain;
use std::sync::{Arc, Mutex};
use log::{info, error};
use std::process;

pub fn initialize_blockchain() -> Arc<Mutex<Blockchain>> {
    match Blockchain::new() {
        Ok(blockchain) => {
            info!("Blockchain initialized successfully.");
            Arc::new(Mutex::new(blockchain))
        },
        Err(e) => {
            error!("Failed to initialize blockchain: {}", e);
            process::exit(1);
        },
    }
}

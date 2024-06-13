use crate::qup::fault_tolerance::FaultTolerantDistributedTrainingNode;
use crate::config::Config;
use log::{info, error};
use std::process;

pub async fn start_training() {
    // Parse configuration
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        },
    };

    // Create a DistributedTrainer instance
    let trainer = FaultTolerantDistributedTrainingNode::new(config);

    // Start training
    match trainer.train().await {
        Ok(_) => info!("Training completed successfully."),
        Err(e) => {
            error!("Training failed: {}", e);
            process::exit(1);
        },
    }
}

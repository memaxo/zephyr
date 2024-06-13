// Main entry point for the ZephyrChain blockchain application.
// Initializes the blockchain, configures logging, and launches the user interface.

mod api; // Handles network requests
mod chain; // Core blockchain functionality and data structures
mod consensus; // Consensus mechanisms
mod network; // Networking and peer-to-peer communication
mod optimization_problems; // Optimization problems for PoUW consensus
mod quantum_entropy; // Quantum entropy source
mod smart_contract; // Smart contract execution and management
mod types; // Common data types and structures
mod ui; // User interface for interacting with the blockchain
mod utils; // Utility functions and modules
mod zkp_crate;
mod secure_core;
mod mining;
mod error_handling;
mod crypto;
mod parallel; // Zero-knowledge proof library

use std::{
    sync::{Arc, Mutex},
    process,
};
use log::{info, error};
use env_logger::{Builder, Env};
use crate::{chain::Blockchain, ui::start_ui, qup::interface::{QUPBlockProposal, QUPVoteHandler, QUPStateProvider}, qup::consensus::QUPConsensus};
use crate::qup::fault_tolerance::FaultTolerantDistributedTrainingNode;
use crate::qup::monitoring::{collect_metrics, evaluate_model};
use crate::config::Config;
use log::{info, LevelFilter};
use env_logger::{Builder, Env};

#[tokio::main]
async fn main() {
    // Initialize logging with default settings or environment configuration
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("ZephyrChain node initializing...");
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("ZephyrChain node initializing...");

    // Create a new blockchain instance and wrap it for thread safety
    let blockchain = match Blockchain::new() {
        Ok(blockchain) => blockchain,
        Err(e) => {
            error!("Failed to initialize blockchain: {}", e);
            process::exit(1);
        },
    };
    let blockchain = Arc::new(Mutex::new(blockchain));
    info!("Blockchain initialized successfully.");
        Ok(blockchain) => blockchain,
        Err(e) => {
            eprintln!("Failed to initialize blockchain: {}", e);
            process::exit(1);
        },
    };
    // Create a QUPConsensus instance
    info!("Creating QUPConsensus instance...");
    let qup_consensus = QUPConsensus {
        // Initialize fields
    };

    // Inject QUP traits
    let block_proposal: Arc<dyn QUPBlockProposal> = Arc::new(qup_consensus);
    let vote_handler: Arc<dyn QUPVoteHandler> = Arc::new(qup_consensus);
    let state_provider: Arc<dyn QUPStateProvider> = Arc::new(qup_consensus);

    // Launch the API server with QUP dependencies
    info!("Launching API server...");
    let api_server = api::start_server(blockchain.clone());

    // Launch the network server
    info!("Launching network server...");
    let network_server = network::start_server(blockchain.clone());

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

    // Collect and report metrics
    let metrics = collect_metrics();
    info!("Training Metrics: {:?}", metrics);

    // Evaluate the model
    let validation_dataset = Dataset::load("validation");
    let evaluation_metrics = evaluate_model(&trainer.model, &validation_dataset);
    info!("Evaluation Metrics: {:?}", evaluation_metrics);
    info!("Launching user interface...");
    match start_ui(blockchain.clone()).await {
        Ok(_) => info!("ZephyrChain node started and user interface launched successfully."),
        Err(e) => {
            error!("Failed to start user interface: {}", e);
            process::exit(1);
        },
    }

    // Wait for the API server and network server to finish
    if let Err(e) = tokio::try_join!(api_server, network_server) {
        error!("Error running servers: {}", e);
    }
}

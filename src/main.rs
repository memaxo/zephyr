// Main entry point for the ZephyrChain blockchain application.
// Initializes the blockchain, configures logging, and launches the user interface.

mod api;
mod chain;
mod consensus;
mod network;
mod optimization_problems;
mod quantum_entropy;
mod smart_contract;
mod types;
mod ui;
mod utils;
mod zkp_crate;
mod secure_core;
mod mining;
mod error_handling;
mod crypto;
mod parallel;
mod data_loading;
mod model_initialization;
mod training_orchestration;
mod result_reporting;

use std::{
    sync::{Arc, Mutex},
    process,
};
use log::{info, error};
use env_logger::{Builder, Env};
use clap::{App, Arg};
use crate::{chain::Blockchain, ui::start_ui, qup::interface::{QUPBlockProposal, QUPVoteHandler, QUPStateProvider}, qup::consensus::QUPConsensus};
use crate::qup::fault_tolerance::FaultTolerantDistributedTrainingNode;
use crate::qup::monitoring::{collect_metrics, evaluate_model};
use crate::config::{Config, DistributedTrainingConfig};
use log::{info, LevelFilter};
use env_logger::{Builder, Env};

#[tokio::main]
async fn main() {
    // Initialize logging with default settings or environment configuration
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("ZephyrChain node initializing...");

    // Define CLI arguments
    let matches = App::new("ZephyrChain")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("ZephyrChain blockchain application")
        .arg(Arg::with_name("dataset")
            .long("dataset")
            .value_name("DATASET")
            .help("Specifies the dataset to use")
            .takes_value(true))
        .arg(Arg::with_name("model")
            .long("model")
            .value_name("MODEL")
            .help("Specifies the model to use")
            .takes_value(true))
        .arg(Arg::with_name("batch_size")
            .long("batch_size")
            .value_name("BATCH_SIZE")
            .help("Specifies the batch size for training")
            .takes_value(true))
        .arg(Arg::with_name("learning_rate")
            .long("learning_rate")
            .value_name("LEARNING_RATE")
            .help("Specifies the learning rate for training")
            .takes_value(true))
        .arg(Arg::with_name("parallelism")
            .long("parallelism")
            .value_name("PARALLELISM")
            .help("Specifies the parallelism strategy (data, model, hybrid, pipeline)")
            .takes_value(true))
        .arg(Arg::with_name("num_pipeline_stages")
            .long("num_pipeline_stages")
            .value_name("NUM_PIPELINE_STAGES")
            .help("Specifies the number of pipeline stages")
            .takes_value(true))
        .get_matches();

    // Extract CLI arguments
    let dataset = matches.value_of("dataset").unwrap_or("default_dataset");
    let model = matches.value_of("model").unwrap_or("default_model");
    let batch_size = matches.value_of("batch_size").unwrap_or("32").parse::<usize>().unwrap();
    let learning_rate = matches.value_of("learning_rate").unwrap_or("0.001").parse::<f64>().unwrap();
    let parallelism = matches.value_of("parallelism").unwrap_or("data");
    let num_pipeline_stages = matches.value_of("num_pipeline_stages").unwrap_or("1").parse::<usize>().unwrap();

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

    // Create a DistributedTrainer instance with CLI arguments
    let distributed_training_config = DistributedTrainingConfig {
        num_nodes: config.distributed_training_config.num_nodes,
        batch_size,
        learning_rate,
        aggregation_frequency: config.distributed_training_config.aggregation_frequency,
        data_parallelism: parallelism == "data",
        model_parallelism: parallelism == "model",
        hybrid_parallelism: parallelism == "hybrid",
        pipeline_parallelism: parallelism == "pipeline",
        num_pipeline_stages,
    };

    let trainer = FaultTolerantDistributedTrainingNode::new(distributed_training_config);

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

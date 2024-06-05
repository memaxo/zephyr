pub mod block;
pub mod config;
pub mod consensus;
pub mod crypto;
pub mod delegator;
pub mod error;
pub mod hdcomm;
pub mod qup_hdcmodels;
pub mod reward;
pub mod state;
pub mod types;
pub mod utils;
pub mod validator;

// Re-export the main types, structs, and functions
pub use block::QUPBlock;
pub use config::{QUPConfig, RewardScheme, UsefulWorkConfig};
pub use consensus::QUPConsensus;
pub use crypto::{QUPCrypto, QUPKeyPair, QUPSignature};
pub use delegator::QUPDelegator;
pub use error::{QUPError, UsefulWorkError};
pub use hdcomm::HDCommunication;
pub use qup_hdcmodels::QUPHDCModels; // Add this line to re-export the QUPHDCModels struct
pub use reward::RewardDistributor;
pub use state::QUPState;
pub use types::{
    KnapsackProblem, KnapsackSolution, QUPBlockHeader, QUPTransaction, QUPVote, VertexCoverProblem,
    VertexCoverSolution,
};
pub use utils::{
    calculate_block_hash, calculate_transaction_hash, verify_block_signature,
    verify_transaction_signature, verify_vote_signature,
};
pub use validator::QUPValidator;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Blockchain error: {0}")]
    BlockchainError(#[from] BlockchainError),

    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] CryptoError),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Consensus error: {0}")]
    ConsensusError(#[from] ConsensusError),

    #[error("Smart contract error: {0}")]
    SmartContractError(#[from] SmartContractError),

    #[error("Wallet error: {0}")]
    WalletError(#[from] WalletError),

    #[error("Configuration error: {0}")]
    ConfigurationError(#[from] ConfigurationError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid data error: {0}")]
    InvalidDataError(String),

    #[error("Permission denied error: {0}")]
    PermissionDeniedError(String),

    #[error("Resource not found error: {0}")]
    ResourceNotFoundError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Block validation failed")]
    BlockValidationFailed,

    #[error("Transaction validation failed")]
    TransactionValidationFailed,

    #[error("Blockchain state error")]
    BlockchainStateError,

    // Add more blockchain-specific error variants
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection failed")]
    ConnectionFailed,

    #[error("Message sending failed")]
    MessageSendingFailed,

    #[error("Message receiving failed")]
    MessageReceivingFailed,

    // Add more network-specific error variants
}

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("Decryption failed")]
    DecryptionFailed,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    // Add more crypto-specific error variants
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Data not found")]
    DataNotFound,

    #[error("Data corruption detected")]
    DataCorruption,

    #[error("Storage unavailable")]
    StorageUnavailable,

    // Add more storage-specific error variants
}

#[derive(Debug, Error)]
pub enum ConsensusError {
    #[error("Invalid block proposer")]
    InvalidBlockProposer,

    #[error("Consensus timeout")]
    ConsensusTimeout,

    #[error("Consensus message invalid")]
    ConsensusMessageInvalid,

    // Add more consensus-specific error variants
}

#[derive(Debug, Error)]
pub enum SmartContractError {
    #[error("Execution failed")]
    ExecutionFailed,

    #[error("Invalid smart contract")]
    InvalidSmartContract,

    #[error("Smart contract not found")]
    SmartContractNotFound,

    // Add more smart contract-specific error variants
}

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid address")]
    InvalidAddress,

    #[error("Wallet locked")]
    WalletLocked,

    // Add more wallet-specific error variants
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Missing configuration")]
    MissingConfiguration,

    #[error("Invalid configuration")]
    InvalidConfiguration,

    #[error("Configuration file not found")]
    ConfigurationFileNotFound,

    // Add more configuration-specific error variants
}
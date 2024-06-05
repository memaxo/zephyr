use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Message sending failed: {0}")]
    MessageSendingFailed(String),

    #[error("Message receiving failed: {0}")]
    MessageReceivingFailed(String),

    #[error("Peer disconnected")]
    PeerDisconnected,

    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    #[error("Invalid message format")]
    InvalidMessageFormat,

    #[error("Incompatible protocol version: {0}")]
    IncompatibleProtocolVersion(String),

    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Network unavailable")]
    NetworkUnavailable,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Address already in use: {0}")]
    AddressAlreadyInUse(String),

    #[error("Connection reset by peer")]
    ConnectionResetByPeer,

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Network error: {0}")]
    Other(String),
}
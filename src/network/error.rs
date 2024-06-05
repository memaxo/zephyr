use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NetworkError {
    ConnectionError(String),
    HandshakeError(String),
    MessageError(String),
    ProtocolError(String),
    PeerError(String),
    DiscoveryError(String),
    TlsError(String),
    Other(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            NetworkError::HandshakeError(msg) => write!(f, "Handshake error: {}", msg),
            NetworkError::MessageError(msg) => write!(f, "Message error: {}", msg),
            NetworkError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            NetworkError::PeerError(msg) => write!(f, "Peer error: {}", msg),
            NetworkError::DiscoveryError(msg) => write!(f, "Discovery error: {}", msg),
            NetworkError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            NetworkError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for NetworkError {}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::Other(err.to_string())
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(err: serde_json::Error) -> Self {
        NetworkError::MessageError(err.to_string())
    }
}

impl From<libp2p::core::transport::TransportError> for NetworkError {
    fn from(err: libp2p::core::transport::TransportError) -> Self {
        NetworkError::ConnectionError(err.to_string())
    }
}

impl From<libp2p::core::upgrade::ProtocolError> for NetworkError {
    fn from(err: libp2p::core::upgrade::ProtocolError) -> Self {
        NetworkError::ProtocolError(err.to_string())
    }
}

impl From<libp2p::kad::KademliaError> for NetworkError {
    fn from(err: libp2p::kad::KademliaError) -> Self {
        NetworkError::DiscoveryError(err.to_string())
    }
}

impl From<libp2p::identity::error::SigningError> for NetworkError {
    fn from(err: libp2p::identity::error::SigningError) -> Self {
        NetworkError::TlsError(err.to_string())
    }
}
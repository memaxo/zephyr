use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeIdError {
    #[error("Invalid node ID format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId(String);

impl NodeId {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let id: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(16)
            .collect();
        NodeId(id)
    }

    pub fn from_str(id: &str) -> Result<Self, NodeIdError> {
        if id.len() != 16 || !id.chars().all(|c| c.is_ascii_alphanumeric()) {
            Err(NodeIdError::InvalidFormat(id.to_string()))
        } else {
            Ok(NodeId(id.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl FromStr for NodeId {
    type Err = NodeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeId(s.to_string()))
    }
}

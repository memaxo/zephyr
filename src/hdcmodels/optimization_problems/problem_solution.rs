use serde::{Serialize, Deserialize};
use crate::state::account::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSolution {
    pub solution_data: Vec<u8>,
    pub objective_value: f64,
    pub metadata: Option<String>,
}

impl ProblemSolution {
    pub fn new(solution_data: Vec<u8>, objective_value: f64, metadata: Option<String>) -> Self {
        ProblemSolution {
            solution_data,
            objective_value,
            metadata,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub resources: Vec<String>,
    pub reward: u64,
    pub deadline: DateTime<Utc>,
    pub creator: String,
}

impl Task {
    pub fn new(
        id: u64,
        description: String,
        resources: Vec<String>,
        reward: u64,
        deadline: DateTime<Utc>,
        creator: String,
    ) -> Self {
        Task {
            id,
            description,
            resources,
            reward,
            deadline,
            creator,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if self.resources.is_empty() {
            return Err("Resources cannot be empty".to_string());
        }
        if self.reward == 0 {
            return Err("Reward must be greater than zero".to_string());
        }
        if self.deadline <= Utc::now() {
            return Err("Deadline must be in the future".to_string());
        }
        Ok(())
    }
}
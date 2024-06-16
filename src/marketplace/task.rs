use std::collections::HashMap;
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
    pub version: u64,
    pub weights: Option<HashMap<String, f64>>,
}

impl Task {
    pub fn new(
        id: u64,
        description: String,
        resources: Vec<String>,
        reward: u64,
        deadline: DateTime<Utc>,
        creator: String,
        weights: Option<HashMap<String, f64>>,
        weights: Option<HashMap<String, f64>>,
    ) -> Self {
        Task {
            id,
            description,
            resources,
            reward,
            deadline,
            creator,
            version: 0,
            weights,
        }
    }
}

impl Task {
    pub fn new(
        id: u64,
        description: String,
        resources: Vec<String>,
        reward: u64,
        deadline: DateTime<Utc>,
        creator: String,
        Task {
            id,
            description,
            resources,
            reward,
            deadline,
            creator,
            version: 0,
            weights,
        }
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
            version: 0,
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

    pub fn increment_version(&mut self) {
        self.version += 1;
    }
}


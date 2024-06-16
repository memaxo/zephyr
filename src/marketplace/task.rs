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
        // Check if description is empty
        if self.description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        // Check if resources are empty
        if self.resources.is_empty() {
            return Err("Resources cannot be empty".to_string());
        }

        // Check if reward is greater than zero
        if self.reward == 0 {
            return Err("Reward must be greater than zero".to_string());
        }

        // Check if deadline is in the future
        if self.deadline <= Utc::now() {
            return Err("Deadline must be in the future".to_string());
        }

        // Check if the task is feasible based on resource requirements
        if !self.is_feasible() {
            return Err("Task is not feasible with the given resources".to_string());
        }

        // Check if the reward structure is realistic
        if !self.is_reward_realistic() {
            return Err("Reward structure is unrealistic".to_string());
        }

        // Flag suspicious or unrealistic tasks
        if self.is_suspicious() {
            return Err("Task is flagged as suspicious".to_string());
        }

        Ok(())
    }

    fn is_feasible(&self) -> bool {
        // Placeholder for actual feasibility check logic
        // For now, return true
        true
    }

    fn is_reward_realistic(&self) -> bool {
        // Placeholder for actual reward realism check logic
        // For now, return true
        true
    }

    fn is_suspicious(&self) -> bool {
        // Placeholder for actual suspicious task detection logic
        // For now, return false
        false
    }

    pub fn increment_version(&mut self) {
        self.version += 1;
    }
}


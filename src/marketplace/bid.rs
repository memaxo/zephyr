use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bid {
    pub node_id: String,
    pub proposed_time: DateTime<Utc>,
    pub proposed_reward: u64,
    pub proof_of_capability: String,
    pub submission_time: DateTime<Utc>,
}

impl Bid {
    pub fn validate(&self, task_deadline: &DateTime<Utc>, minimum_reward: u64, maximum_reward: u64) -> Result<(), String> {
        // Check if proposed time is past the task deadline
        if self.proposed_time > *task_deadline {
            return Err("Proposed time is past the task deadline".to_string());
        }

        // Check if proposed reward is within acceptable range
        if self.proposed_reward < minimum_reward {
            return Err("Proposed reward is below the minimum acceptable reward".to_string());
        }
        if self.proposed_reward > maximum_reward {
            return Err("Proposed reward exceeds the maximum acceptable reward".to_string());
        }

        // Verify the authenticity of the proof_of_capability
        if !self.verify_proof_of_capability() {
            return Err("Proof of capability is not authentic".to_string());
        }

        Ok(())
    }

    fn verify_proof_of_capability(&self) -> bool {
        // Placeholder for actual proof of capability verification logic
        // For now, return true
        true
    }

impl Bid {
    pub fn new(
        node_id: String,
        proposed_time: DateTime<Utc>,
        proposed_reward: u64,
        proof_of_capability: String,
        submission_time: DateTime<Utc>,
    ) -> Self {
        Bid {
            node_id,
            proposed_time,
            proposed_reward,
            proof_of_capability,
            submission_time,
        }
    }
}


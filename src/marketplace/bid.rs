use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bid {
    pub node_id: String,
    pub proposed_time: DateTime<Utc>,
    pub proposed_reward: u64,
    pub proof_of_capability: String,
}

impl Bid {
    pub fn new(
        node_id: String,
        proposed_time: DateTime<Utc>,
        proposed_reward: u64,
        proof_of_capability: String,
    ) -> Self {
        Bid {
            node_id,
            proposed_time,
            proposed_reward,
            proof_of_capability,
        }
    }
}

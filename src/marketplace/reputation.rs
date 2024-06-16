use std::collections::HashMap;

pub struct Reputation {
    scores: HashMap<String, f64>,
    pub fn get_priority(&self, node_id: &str) -> f64 {
        let base_priority = 1.0;
        let reputation = self.get_reputation(node_id);
        base_priority + reputation * 0.1 // Example: increase priority based on reputation
    }

    pub fn get_fee_discount(&self, node_id: &str) -> f64 {
        let base_fee = 1.0;
        let reputation = self.get_reputation(node_id);
        base_fee - reputation * 0.05 // Example: reduce fee based on reputation
    }

    pub fn can_participate(&self, node_id: &str) -> bool {
        let reputation = self.get_reputation(node_id);
        reputation >= 0.5 // Example: minimum reputation required to participate
    }

impl Reputation {
    pub fn new() -> Self {
        Reputation {
            scores: HashMap::new(),
        }
    }

    pub fn update_reputation(&mut self, node_id: &str, completion_rate: f64, timeliness: f64, quality: f64, feedback: f64) {
        let decay_factor = 0.9;
        let weight_completion = 0.4;
        let weight_timeliness = 0.3;
        let weight_quality = 0.2;
        let weight_feedback = 0.1;

        let score = self.scores.entry(node_id.to_string()).or_insert(0.0);
        let new_score = weight_completion * completion_rate
            + weight_timeliness * timeliness
            + weight_quality * quality
            + weight_feedback * feedback;

        *score = (*score * decay_factor + new_score).max(0.0); // Ensure reputation score is non-negative
    }

    pub fn get_reputation(&self, node_id: &str) -> f64 {
        *self.scores.get(node_id).unwrap_or(&0.0)
    }

    pub fn reset_reputation(&mut self, node_id: &str) {
        self.scores.insert(node_id.to_string(), 0.0);
    }
}

use std::collections::HashMap;

pub struct Reputation {
    scores: HashMap<String, f64>,
}

impl Reputation {
    pub fn new() -> Self {
        Reputation {
            scores: HashMap::new(),
        }
    }

    pub fn update_reputation(&mut self, node_id: &str, delta: f64) {
        let score = self.scores.entry(node_id.to_string()).or_insert(0.0);
        *score += delta;
    }

    pub fn get_reputation(&self, node_id: &str) -> f64 {
        *self.scores.get(node_id).unwrap_or(&0.0)
    }

    pub fn reset_reputation(&mut self, node_id: &str) {
        self.scores.insert(node_id.to_string(), 0.0);
    }
}

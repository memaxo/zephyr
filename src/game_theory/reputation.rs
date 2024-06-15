// src/game_theory/reputation.rs

use crate::qup::types::{NodeId, ReputationScore, ReputationAction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Reputation {
    scores: HashMap<NodeId, ReputationScore>,
    history: HashMap<NodeId, Vec<ReputationAction>>,
}

impl Reputation {
    pub fn new() -> Self {
        Reputation {
            scores: HashMap::new(),
            history: HashMap::new(),
        }
    }

    pub fn update(&mut self, node_id: NodeId, action: ReputationAction) {
        // Update the reputation score based on the node's action
        let score = self.scores.entry(node_id.clone()).or_insert(0);
        match action {
            ReputationAction::SuccessfulBlockProposal => *score += 10,
            ReputationAction::UsefulWorkContribution => *score += 5,
            ReputationAction::MaliciousBehavior => *score -= 20,
            // Add more actions and their corresponding score updates
        }

        // Update the reputation history
        self.history
            .entry(node_id)
            .or_insert(Vec::new())
            .push(action);
    }

    pub fn get_score(&self, node_id: &NodeId) -> Option<ReputationScore> {
        self.scores.get(node_id).cloned()
    }

    pub fn get_history(&self, node_id: &NodeId) -> Option<&Vec<ReputationAction>> {
        self.history.get(node_id)
    }

    pub fn calculate_average_score(&self) -> f64 {
        if self.scores.is_empty() {
            0.0
        } else {
            self.scores.values().sum::<ReputationScore>() as f64 / self.scores.len() as f64
        }
    }

    pub fn get_top_reputed_nodes(&self, count: usize) -> Vec<NodeId> {
        let mut nodes: Vec<NodeId> = self.scores.keys().cloned().collect();
        nodes.sort_by(|a, b| self.scores[b].cmp(&self.scores[a]));
        nodes.truncate(count);
        nodes
    }
}
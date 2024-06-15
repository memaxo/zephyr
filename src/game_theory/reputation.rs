// src/game_theory/reputation.rs

use crate::qup::types::{NodeId, ReputationScore, ReputationAction, ReputationOracle, DecentralizedIdentity};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Reputation {
    scores: HashMap<NodeId, ReputationScore>,
    history: HashMap<NodeId, Vec<ReputationAction>>,
    pub fn integrate_with_oracle(&mut self, oracle: &ReputationOracle) {
        for (node_id, score) in oracle.get_reputation_scores() {
            self.scores.insert(node_id.clone(), score);
        }
    }

    pub fn integrate_with_identity_system(&mut self, identity_system: &DecentralizedIdentity) {
        for (node_id, identity) in identity_system.get_verified_identities() {
            // Update reputation based on verified identities
            if identity.is_trusted() {
                let score = self.scores.entry(node_id.clone()).or_insert(0);
                *score += 10; // Example boost for verified identities
            }
        }
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
        log::info!("Updating reputation for node: {:?} with action: {:?}", node_id, action);
        let score = self.scores.entry(node_id.clone()).or_insert(0);
        match action {
            ReputationAction::SuccessfulBlockProposal => *score += 10,
            ReputationAction::UsefulWorkContribution => *score += 5,
            ReputationAction::MaliciousBehavior => *score -= 20,
            ReputationAction::TransactionValidation => *score += 8,
            ReputationAction::IncorrectTransactionValidation => *score -= 15,
            ReputationAction::NetworkParticipation => *score += 2,
            ReputationAction::CommunityFeedbackPositive => *score += 5,
            ReputationAction::CommunityFeedbackNegative => *score -= 5,
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
            // Use parallel iterator for performance optimization
            self.scores.par_values().sum::<ReputationScore>() as f64 / self.scores.len() as f64
        }
    }

    pub fn get_top_reputed_nodes(&self, count: usize) -> Vec<NodeId> {
        let mut nodes: Vec<NodeId> = self.scores.keys().cloned().collect();
        nodes.sort_by(|a, b| self.scores[b].cmp(&self.scores[a]));
        nodes.truncate(count);
        nodes
    }
}

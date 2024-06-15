use crate::qup::types::{NodeId, ReputationAction, StakeAmount, RewardAmount, PenaltyAmount};
use crate::qup::block::QUPBlock;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Reputation {
    pub node_id: NodeId,
    pub score: u64,
    pub history: Vec<ReputationAction>,
}

impl Reputation {
    pub fn new(node_id: NodeId) -> Self {
        Reputation {
            node_id,
            score: 0,
            history: Vec::new(),
        }
    }
}

pub struct RewardManager {
    pub reward_rate: f64,
    pub penalty_rate: f64,
}

impl RewardManager {
    pub fn new(reward_rate: f64, penalty_rate: f64) -> Self {
        RewardManager {
            reward_rate,
            penalty_rate,
        }
    }

    pub fn calculate_rewards(&self, block: &QUPBlock) -> HashMap<NodeId, RewardAmount> {
        let mut rewards = HashMap::new();

        // Use parallel iterator for performance optimization
        block.stakes.par_iter().for_each(|(node_id, stake)| {
            let performance = block.performance.get(node_id).unwrap_or(&0.0);
            let reward = self.calculate_individual_reward(*stake, *performance, self.reward_rate);
            rewards.insert(node_id.clone(), reward);
        }

        rewards
    }

    fn calculate_individual_reward(&self, stake: u64, performance: f64, reward_rate: f64) -> u64 {
        let stake_reward = stake as f64 * reward_rate;
        let performance_bonus = stake_reward * performance;
        (stake_reward + performance_bonus) as u64
    }

    pub fn distribute_rewards(&self, rewards: &HashMap<NodeId, RewardAmount>, state: &mut QUPState) {
        for (node_id, reward) in rewards {
            state.add_tokens(node_id, *reward);
        }
    }

    pub fn calculate_penalties(&self, node_id: &NodeId, violation: &str) -> PenaltyAmount {
        let base_penalty = match violation {
            "Double-signing" => 100.0,
            "Invalid block proposal" => 50.0,
            "Failed validation" => 30.0,
            "Offline" => 20.0,
            "Incorrect verification result" => 40.0,
            _ => 10.0,
        };
        (base_penalty * self.penalty_rate) as PenaltyAmount
    }

    pub fn apply_penalties(&self, node_id: &NodeId, penalty: PenaltyAmount, state: &mut QUPState) {
        state.reduce_stake(node_id, penalty);
        update_reputation(node_id, ReputationAction::MaliciousBehavior);
    }
    }
}

pub struct VerificationGame {
    pub task_id: String,
    pub challenger: NodeId,
    pub challengee: NodeId,
    pub stake: StakeAmount,
}

impl VerificationGame {
    pub fn new(task_id: String, challenger: NodeId, challengee: NodeId, stake: StakeAmount) -> Self {
        VerificationGame {
            task_id,
            challenger,
            challengee,
            stake,
        }
    }

    pub fn initiate(&self, agent_id: NodeId, task_id: String, challenge: String) {
        // Create a new VerificationGame instance
        let verification_game = VerificationGame {
            task_id: task_id.clone(),
            challenger: agent_id.clone(),
            challengee: challenge.clone(),
            stake: 0, // Set the initial stake
        };

        // Broadcast the challenge to other nodes
        // (Assuming a function `broadcast_challenge` exists)
        broadcast_challenge(&verification_game);

        // Set a timeout for the verification process
        // (Assuming a function `set_verification_timeout` exists)
        set_verification_timeout(&verification_game);
    }
    }

    pub fn resolve(&self, task_id: String, result: bool, state: &mut QUPState) {
        // Retrieve the VerificationGame instance
        if let Some(verification_game) = get_verification_game(&task_id) {
            if result {
                // Reward the agent who initiated the challenge
                state.add_tokens(&verification_game.challenger, verification_game.stake);
                // Penalize the node that provided the incorrect result
                self.apply_penalties(&verification_game.challengee, verification_game.stake, state);
            } else {
                // Penalize the agent who initiated the challenge
                self.apply_penalties(&verification_game.challenger, verification_game.stake, state);
            }
        }
    }
    }
}

pub fn update_reputation(node_id: &NodeId, action: ReputationAction) {
    // Retrieve the Reputation object for the node_id
    let mut reputation = get_reputation(node_id);

    // Update the reputation score based on the action
    match action {
        ReputationAction::SuccessfulBlockProposal => reputation.score += 10,
        ReputationAction::FailedBlockProposal => reputation.score -= 10,
        ReputationAction::SuccessfulUsefulWork => reputation.score += 5,
        ReputationAction::MaliciousBehavior => reputation.score -= 20,
        // Add more actions and their corresponding score updates
    }

    // Store the updated Reputation object
    set_reputation(node_id, reputation);
}
}

use crate::chain::state::QUPState;
use crate::network::quantum_resistant::QuantumResistantConnectionManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rewards {
    pub reward_pool: u64,
    pub reward_rate: f64,
    pub rewards: HashMap<String, u64>,
}

impl Rewards {
    pub fn new(initial_reward_pool: u64, reward_rate: f64) -> Self {
        Rewards {
            reward_pool: initial_reward_pool,
            reward_rate,
            rewards: HashMap::new(),
        }
    }

    pub async fn distribute_rewards(
        &mut self,
        participants: &[String],
        total_participation: u64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), RewardError> {
        let reward_per_participant = self.calculate_reward_per_participant(total_participation);

        for participant in participants {
            let reward = reward_per_participant;
            *self.rewards.entry(participant.clone()).or_insert(0) += reward;
            self.reward_pool -= reward;

            // Update the participant's reward in the QUP state
            state.update_reward(participant, reward).await?;
        }

        // Broadcast the reward distribution to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution {
            participants: participants.to_vec(),
            total_participation,
        };
        connection_manager.broadcast(message).await?;

        Ok(())
    }

    fn calculate_reward_per_participant(&self, total_participation: u64) -> u64 {
        ((self.reward_pool as f64) * self.reward_rate / (total_participation as f64)) as u64
    }

    pub async fn claim_reward(
        &mut self,
        participant: &str,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<Option<u64>, RewardError> {
        let claimed_reward = self.rewards.remove(participant);

        if let Some(reward) = claimed_reward {
            // Update the participant's claimed reward in the QUP state
            state.claim_reward(participant, reward).await?;

            // Broadcast the reward claim to the network using quantum-resistant communication
            let message = GovernanceMessage::RewardDistribution {
                participant: participant.to_string(),
                reward,
            };
            connection_manager.broadcast(message).await?;
        }

        Ok(claimed_reward)
    }

    pub fn get_unclaimed_reward(&self, participant: &str) -> Option<&u64> {
        self.rewards.get(participant)
    }

    pub async fn add_to_reward_pool(
        &mut self,
        amount: u64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), RewardError> {
        self.reward_pool += amount;

        // Update the reward pool in the QUP state
        state.update_reward_pool(self.reward_pool).await?;

        // Broadcast the reward pool update to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution {
            new_reward_pool: self.reward_pool,
        };
        connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn update_reward_rate(
        &mut self,
        new_rate: f64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), RewardError> {
        self.reward_rate = new_rate;

        // Update the reward rate in the QUP state
        state.update_reward_rate(new_rate).await?;

        // Broadcast the reward rate update to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution {
            new_reward_rate: new_rate,
        };
        connection_manager.broadcast(message).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RewardMessage {
    RewardDistributed {
        participants: Vec<String>,
        total_participation: u64,
    },
    RewardClaimed {
        participant: String,
        reward: u64,
    },
    RewardPoolUpdated {
        new_reward_pool: u64,
    },
    RewardRateUpdated {
        new_reward_rate: f64,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum RewardError {
    #[error("Failed to update reward in the QUP state: {0}")]
    FailedToUpdateReward(String),
    #[error("Failed to claim reward in the QUP state: {0}")]
    FailedToClaimReward(String),
    #[error("Failed to update reward pool in the QUP state: {0}")]
    FailedToUpdateRewardPool(String),
    #[error("Failed to update reward rate in the QUP state: {0}")]
    FailedToUpdateRewardRate(String),
    #[error("Failed to broadcast reward message: {0}")]
    FailedToBroadcastRewardMessage(String),
}

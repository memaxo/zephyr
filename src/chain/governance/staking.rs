use crate::chain::state::QUPState;
use crate::network::quantum_resistant::QuantumResistantConnectionManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staking {
    pub total_staked: u64,
    pub staked_amounts: HashMap<String, u64>,
    pub reward_rate: f64,
    pub lock_period: u64,
}

impl Staking {
    pub fn new(reward_rate: f64, lock_period: u64) -> Self {
        Staking {
            total_staked: 0,
            staked_amounts: HashMap::new(),
            reward_rate,
            lock_period,
        }
    }

    pub async fn stake(
        &mut self,
        staker: String,
        amount: u64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), StakingError> {
        *self.staked_amounts.entry(staker.clone()).or_insert(0) += amount;
        self.total_staked += amount;

        // Update the staking information in the QUP state
        state.update_staking(staker.clone(), amount).await?;

        // Broadcast the staking update to the network using quantum-resistant communication
        let message = GovernanceMessage::StakingUpdate {
            staker,
            amount,
            total_staked: self.total_staked,
        };
        connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn unstake(
        &mut self,
        staker: &str,
        amount: u64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), StakingError> {
        if let Some(staked_amount) = self.staked_amounts.get_mut(staker) {
            if *staked_amount >= amount {
                *staked_amount -= amount;
                self.total_staked -= amount;

                // Update the staking information in the QUP state
                state.update_staking(staker.to_string(), -amount).await?;

                // Broadcast the unstaking update to the network using quantum-resistant communication
                let message = GovernanceMessage::UnstakingUpdate {
                    staker: staker.to_string(),
                    amount,
                    total_staked: self.total_staked,
                };
                connection_manager.broadcast(message).await?;

                Ok(())
            } else {
                Err(StakingError::InsufficientStakedAmount)
            }
        } else {
            Err(StakingError::StakerNotFound)
        }
    }

    pub fn get_staked_amount(&self, staker: &str) -> Option<&u64> {
        self.staked_amounts.get(staker)
    }

    pub fn calculate_reward(&self, staker: &str) -> Option<u64> {
        self.get_staked_amount(staker)
            .map(|amount| ((self.reward_rate * (*amount as f64)) as u64))
    }

    pub async fn distribute_rewards(
        &mut self,
        rewards: &mut HashMap<String, u64>,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), StakingError> {
        for (staker, staked_amount) in &self.staked_amounts {
            let reward = self.calculate_reward(staker).unwrap_or(0);
            *rewards.entry(staker.clone()).or_insert(0) += reward;

            // Update the reward distribution in the QUP state
            state.distribute_reward(staker, reward).await?;
        }

        // Broadcast the reward distribution to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution {
            rewards: rewards.clone(),
        };
        connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn update_reward_rate(
        &mut self,
        new_rate: f64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), StakingError> {
        self.reward_rate = new_rate;

        // Update the reward rate in the QUP state
        state.update_reward_rate(new_rate).await?;

        // Broadcast the reward rate update to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution { new_rate };
        connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn update_lock_period(
        &mut self,
        new_period: u64,
        state: &mut QUPState,
        connection_manager: &QuantumResistantConnectionManager,
    ) -> Result<(), StakingError> {
        self.lock_period = new_period;

        // Update the lock period in the QUP state
        state.update_lock_period(new_period).await?;

        // Broadcast the lock period update to the network using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution { new_period };
        connection_manager.broadcast(message).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StakingMessage {
    StakeUpdated {
        staker: String,
        amount: u64,
        total_staked: u64,
    },
    UnstakeUpdated {
        staker: String,
        amount: u64,
        total_staked: u64,
    },
    RewardsDistributed {
        rewards: HashMap<String, u64>,
    },
    RewardRateUpdated {
        new_rate: f64,
    },
    LockPeriodUpdated {
        new_period: u64,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum StakingError {
    #[error("Insufficient staked amount")]
    InsufficientStakedAmount,
    #[error("Staker not found")]
    StakerNotFound,
    #[error("Failed to update staking in the QUP state: {0}")]
    FailedToUpdateStaking(String),
    #[error("Failed to distribute reward in the QUP state: {0}")]
    FailedToDistributeReward(String),
    #[error("Failed to update reward rate in the QUP state: {0}")]
    FailedToUpdateRewardRate(String),
    #[error("Failed to update lock period in the QUP state: {0}")]
    FailedToUpdateLockPeriod(String),
    #[error("Failed to broadcast staking message: {0}")]
    FailedToBroadcastStakingMessage(String),
}

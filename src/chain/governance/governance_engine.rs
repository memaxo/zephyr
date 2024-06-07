use crate::chain::governance::proposal::{Proposal, ProposalStatus};
use crate::chain::governance::rewards::Rewards;
use crate::chain::governance::staking::Staking;
use crate::chain::governance::treasury::Treasury;
use crate::chain::governance::voting::{Vote, Voting};
use crate::chain::state::QUPState;
use crate::network::quantum_resistant::QuantumResistantConnectionManager;
use crate::qup::validator::QUPValidator;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GovernanceEngine {
    voting: Arc<Voting>,
    treasury: Arc<Treasury>,
    rewards: Arc<Rewards>,
    staking: Arc<Staking>,
    qup_state: Arc<QUPState>,
    connection_manager: Arc<QuantumResistantConnectionManager>,
}

impl GovernanceEngine {
    pub fn new(
        voting: Arc<Voting>,
        treasury: Arc<Treasury>,
        rewards: Arc<Rewards>,
        staking: Arc<Staking>,
        qup_state: Arc<QUPState>,
        connection_manager: Arc<QuantumResistantConnectionManager>,
    ) -> Self {
        GovernanceEngine {
            voting,
            treasury,
            rewards,
            staking,
            qup_state,
            connection_manager,
        }
    }

    pub async fn create_proposal(&self, proposal: Proposal) -> Result<(), String> {
        // Store the proposal in the QUP state
        self.qup_state.add_proposal(proposal.clone()).await?;

        // Broadcast the proposal to other nodes using quantum-resistant communication
        let message = GovernanceMessage::NewProposal(proposal);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn vote(&mut self, vote: Vote) -> Result<(), String> {
        // Cast the vote using the voting module
        self.voting.cast_vote(vote.clone())?;

        // Update the vote in the QUP state
        self.qup_state.add_vote(vote.clone()).await?;

        // Broadcast the vote to other nodes using quantum-resistant communication
        let message = GovernanceMessage::NewVote(vote);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn execute_proposal(&self, proposal_id: u64) -> Result<(), String> {
        // Retrieve the proposal from the QUP state
        let proposal = self.qup_state.get_proposal(proposal_id).await?;

        // Check if the proposal is in the correct state for execution
        if proposal.status != ProposalStatus::Succeeded {
            return Err("Proposal is not in the Succeeded state".to_string());
        }

        // Execute the proposal based on its type and parameters
        match proposal.proposal_type {
            ProposalType::AllocateFunds { allocation_id, amount } => {
                self.treasury.allocate(allocation_id, amount)?;
            }
            ProposalType::UpdateRewardRate { new_rate } => {
                self.rewards.update_reward_rate(new_rate).await?;
            }
            ProposalType::UpdateLockPeriod { new_period } => {
                self.staking.update_lock_period(new_period).await?;
            }
            ProposalType::TransferFunds { recipient, amount } => {
                self.treasury.transfer(recipient, amount)?;
            }
        }

        // Update the proposal status in the QUP state
        self.qup_state
            .update_proposal_status(proposal_id, ProposalStatus::Executed)
            .await?;

        // Broadcast the executed proposal to other nodes using quantum-resistant communication
        let message = GovernanceMessage::ExecutedProposal(proposal_id);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn stake(&mut self, staker: String, amount: u64) -> Result<(), String> {
        // Perform the staking using the staking module
        self.staking.stake(staker.clone(), amount, &mut self.qup_state, &self.connection_manager).await?;

        // Broadcast the staking update to other nodes using quantum-resistant communication
        let message = GovernanceMessage::StakingUpdate(staker, amount);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn unstake(&mut self, staker: String, amount: u64) -> Result<(), String> {
        // Perform the unstaking using the staking module
        self.staking.unstake(&staker, amount, &mut self.qup_state, &self.connection_manager).await?;

        // Broadcast the unstaking update to other nodes using quantum-resistant communication
        let message = GovernanceMessage::UnstakingUpdate(staker, amount);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn distribute_rewards(&mut self) -> Result<(), String> {
        let mut rewards = HashMap::new();

        // Distribute rewards to stakers using the rewards module
        self.rewards.distribute_rewards(&mut rewards, &mut self.qup_state, &self.connection_manager).await?;

        // Broadcast the reward distribution to other nodes using quantum-resistant communication
        let message = GovernanceMessage::RewardDistribution(rewards);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn update_validator_set(
        &mut self,
        validators: Vec<QUPValidator>,
    ) -> Result<(), String> {
        // Update the validator set in the QUP state
        self.qup_state.update_validators(validators.clone()).await?;

        // Broadcast the updated validator set to other nodes using quantum-resistant communication
        let message = GovernanceMessage::ValidatorSetUpdate(validators);
        self.connection_manager.broadcast(message).await?;

        Ok(())
    }

    pub async fn deposit_to_treasury(&mut self, amount: u64) -> Result<(), String> {
        self.treasury.deposit(amount);
        Ok(())
    }

    pub async fn withdraw_from_treasury(&mut self, amount: u64) -> Result<(), String> {
        self.treasury.withdraw(amount)?;
        Ok(())
    }

    pub async fn allocate_treasury_funds(&mut self, allocation_id: String, amount: u64) -> Result<(), String> {
        self.treasury.allocate(allocation_id, amount)?;
        Ok(())
    }

    pub async fn deallocate_treasury_funds(&mut self, allocation_id: &str) -> Result<(), String> {
        self.treasury.deallocate(allocation_id)?;
        Ok(())
    }

    pub async fn get_treasury_allocation(&self, allocation_id: &str) -> Option<u64> {
        self.treasury.get_allocation(allocation_id)
    }

    pub async fn update_reward_rate(&mut self, new_rate: f64) -> Result<(), String> {
        self.rewards.update_reward_rate(new_rate, &mut self.qup_state, &self.connection_manager).await?;
        Ok(())
    }

    pub async fn update_lock_period(&mut self, new_period: u64) -> Result<(), String> {
        self.staking.update_lock_period(new_period, &mut self.qup_state, &self.connection_manager).await?;
        Ok(())
    }
}

// Define governance-related messages for quantum-resistant communication
pub enum GovernanceMessage {
    NewProposal(Proposal),
    NewVote(Vote),
    ExecutedProposal(u64),
    StakingUpdate(String, u64),
    UnstakingUpdate(String, u64),
    RewardDistribution(HashMap<String, u64>),
    ValidatorSetUpdate(Vec<QUPValidator>),
}

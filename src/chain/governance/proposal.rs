use crate::chain::blockchain::BlockchainError as GovernanceError;
use crate::chain::state::QUPState;
use crate::chain::storage::proposal_storage::ProposalStorage;
use crate::chain::transaction::Transaction as ProposalTransaction;
use crate::crypto::hash::{Hashable, Hasher};
use crate::network::quantum_resistant::{
    QuantumResistantConnection, QuantumResistantConnectionManager,
};
use crate::qup::crypto::QUPCrypto;
use crate::utils::timestamp::Timestamp;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub enum ProposalStatus {
    Pending,
    Active,
    Succeeded,
    Defeated,
    Executed,
}

impl ProposalStatus {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ProposalStatus::Pending => vec![0],
            ProposalStatus::Active => vec![1],
            ProposalStatus::Succeeded => vec![2],
            ProposalStatus::Defeated => vec![3],
            ProposalStatus::Executed => vec![4],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub created_at: Timestamp,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub status: ProposalStatus,
    pub qup_signature: Vec<u8>,
}

impl Proposal {
    pub fn new(
        id: u64,
        title: String,
        description: String,
        proposer: String,
        start_time: Timestamp,
        end_time: Timestamp,
        qup_crypto: &QUPCrypto,
    ) -> Result<Self, GovernanceError> {
        let proposal = Proposal {
            id,
            title,
            description,
            proposer,
            created_at: Timestamp::now(),
            start_time,
            end_time,
            status: ProposalStatus::Pending,
            qup_signature: vec![],
        };

        let qup_signature = qup_crypto.sign_proposal(&proposal)?;
        let signed_proposal = Proposal {
            qup_signature,
            ..proposal
        };

        Ok(signed_proposal)
    }

    pub fn is_active(&self) -> bool {
        let current_time = Timestamp::now();
        current_time >= self.start_time && current_time < self.end_time
    }

    pub fn update_status(
        &mut self,
        new_status: ProposalStatus,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), GovernanceError> {
        self.status = new_status;
        self.qup_signature = qup_crypto.sign_proposal(self)?;
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        let current_time = Timestamp::now();
        current_time >= self.end_time
    }

    pub async fn execute(
        &mut self,
        state: &mut QUPState,
        qup_crypto: &QUPCrypto,
    ) -> Result<(), GovernanceError> {
        if self.status == ProposalStatus::Succeeded {
            self.update_status(ProposalStatus::Executed, qup_crypto)?;
            state.apply_proposal_changes(self).await?;
            info!("Proposal executed: {:?}", self);
            Ok(())
        } else {
            Err(GovernanceError::InvalidProposalStatus(
                "Cannot execute proposal. It has not succeeded yet.".to_string(),
            ))
        }
    }

    pub fn create_proposal_transaction(&self) -> ProposalTransaction {
        ProposalTransaction::new(
            self.id,
            self.title.clone(),
            self.description.clone(),
            self.proposer.clone(),
            self.start_time,
            self.end_time,
        )
    }

    pub async fn save(&self, storage: &Arc<ProposalStorage>) -> Result<(), GovernanceError> {
        storage.save_proposal(self.clone()).await?;
        debug!("Proposal saved: {:?}", self);
        Ok(())
    }

    pub async fn load(id: u64, storage: &Arc<ProposalStorage>) -> Result<Self, GovernanceError> {
        let proposal = storage.load_proposal(id).await?;
        debug!("Proposal loaded: {:?}", proposal);
        Ok(proposal)
    }

    pub async fn delete(&self, storage: &Arc<ProposalStorage>) -> Result<(), GovernanceError> {
        storage.delete_proposal(self.id).await?;
        debug!("Proposal deleted: {:?}", self);
        Ok(())
    }

    pub async fn start_voting(&self, voting: &mut Voting, connection_manager: &QuantumResistantConnectionManager) -> Result<(), GovernanceError> {
        if self.is_active() {
            voting.start_voting_for_proposal(self.id)?;
            info!("Voting started for proposal: {:?}", self);

            // Broadcast the voting start event to the network using quantum-resistant communication
            let message = GovernanceMessage::NewProposal(self.clone());
            connection_manager.broadcast(message).await?;

            Ok(())
        } else {
            Err(GovernanceError::InvalidProposalStatus("Cannot start voting. The proposal is not active.".to_string()))
    }

    pub fn verify_proposal_integrity(&self, qup_crypto: &QUPCrypto) -> bool {
        // Perform integrity checks on the proposal data
        // Example: Check for empty fields, invalid timestamps, etc.
        !self.title.is_empty()
            && !self.description.is_empty()
            && self.start_time < self.end_time
            && qup_crypto.verify_proposal_signature(self)
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Hasher::new();
        hasher.update(&self.id.to_le_bytes());
        hasher.update(self.title.as_bytes());
        hasher.update(self.description.as_bytes());
        hasher.update(self.proposer.as_bytes());
        hasher.update(&self.created_at.to_bytes());
        hasher.update(&self.start_time.to_bytes());
        hasher.update(&self.end_time.to_bytes());
        hasher.update(&self.status.to_bytes());
        hasher.finalize()
    }
}

impl Hashable for Proposal {
    fn hash(&self) -> Vec<u8> {
        self.calculate_hash()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProposalMessage {
    VotingStarted(u64),
    // Add more proposal-related messages as needed
}

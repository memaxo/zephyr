use crate::blockchain::consensus::{ConsensusMessage, ConsensusProtocol};
use crate::blockchain::governance::GovernanceProposal;
use crate::crypto::signature::{sign_data, verify_signature};
use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::HDCModel;

pub struct SecureHDCModelUpdate {
    pub model_id: String,
    pub updated_model: HDCModel,
    pub signature: Vec<u8>,
}

impl SecureHDCModelUpdate {
    pub fn new(model_id: String, updated_model: HDCModel, private_key: &[u8]) -> Self {
        let encoded_model = encode_data(&updated_model.to_bytes(), updated_model.dimension);
        let signature = sign_data(&encoded_model, private_key);

        SecureHDCModelUpdate {
            model_id,
            updated_model,
            signature,
        }
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        let encoded_model =
            encode_data(&self.updated_model.to_bytes(), self.updated_model.dimension);
        verify_signature(&encoded_model, &self.signature, public_key)
    }
}

pub struct SecureHDCModelUpdateProposal {
    pub model_update: SecureHDCModelUpdate,
    pub proposer: String,
    pub justification: String,
}

impl GovernanceProposal for SecureHDCModelUpdateProposal {
    fn proposal_type(&self) -> String {
        "SecureHDCModelUpdate".to_string()
    }

    fn propose(&self, consensus_protocol: &mut dyn ConsensusProtocol) {
        let proposal_message = ConsensusMessage::SecureHDCModelUpdateProposal(self.clone());
        consensus_protocol.broadcast_message(proposal_message);
    }
}

pub struct SecureHDCModelUpdateVote {
    pub proposal_id: String,
    pub voter: String,
    pub vote: bool,
    pub signature: Vec<u8>,
}

impl SecureHDCModelUpdateVote {
    pub fn new(proposal_id: String, voter: String, vote: bool, private_key: &[u8]) -> Self {
        let vote_data = format!("{}{}{}", proposal_id, voter, vote);
        let signature = sign_data(vote_data.as_bytes(), private_key);

        SecureHDCModelUpdateVote {
            proposal_id,
            voter,
            vote,
            signature,
        }
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        let vote_data = format!("{}{}{}", self.proposal_id, self.voter, self.vote);
        verify_signature(vote_data.as_bytes(), &self.signature, public_key)
    }
}

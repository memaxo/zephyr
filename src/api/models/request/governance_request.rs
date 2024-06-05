use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SubmitProposalRequest {
    pub title: String,
    pub description: String,
    // Add other proposal fields as needed
}

#[derive(Deserialize, Debug)]
pub struct CastVoteRequest {
    pub proposal_id: u64,
    pub vote_type: VoteType,
    // Add other vote fields as needed (e.g., voter signature)
}

#[derive(Deserialize, Debug)]
pub struct GetProposalRequest {
    pub proposal_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct GetProposalsRequest {
    pub status: Option<ProposalStatus>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct GetGovernanceParametersRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProposalStatus {
    Open,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GovernanceParameters {
    pub voting_period: u64,
    pub quorum_ratio: f64,
    // Add other governance parameters as needed
}
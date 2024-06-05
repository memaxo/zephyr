use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::chain::governance::{GovernanceEngine, Proposal, Vote, VoteType};

pub async fn get_proposals(
    governance_engine: web::Data<GovernanceEngine>,
) -> impl Responder {
    let proposals: Vec<ProposalInfo> = governance_engine
        .proposals()
        .iter()
        .map(|p| ProposalInfo::from_proposal(p))
        .collect();

    HttpResponse::Ok().json(proposals)
}

pub async fn get_proposal(
    governance_engine: web::Data<GovernanceEngine>,
    id: web::Path<u64>,
) -> impl Responder {
    let proposal = governance_engine.get_proposal(id.into_inner());
    if let Some(p) = proposal {
        let proposal_info = ProposalInfo::from_proposal(&p);
        HttpResponse::Ok().json(proposal_info)
    } else {
        HttpResponse::NotFound().body("Proposal not found")
    }
}

pub async fn get_proposal_votes(
    governance_engine: web::Data<GovernanceEngine>,
    id: web::Path<u64>,
) -> impl Responder {
    let proposal = governance_engine.get_proposal(id.into_inner());
    if let Some(p) = proposal {
        let votes: Vec<VoteInfo> = p
            .votes
            .iter()
            .map(|v| VoteInfo::from_vote(v))
            .collect();
        HttpResponse::Ok().json(votes)
    } else {
        HttpResponse::NotFound().body("Proposal not found")
    }
}

pub async fn get_voting_power(
    governance_engine: web::Data<GovernanceEngine>,
) -> impl Responder {
    let voting_power = governance_engine.get_voting_power();
    let voter_power: Vec<(String, f64)> = voting_power
        .iter()
        .map(|(voter, power)| (voter.to_string(), *power))
        .collect();
    let total_voting_power: f64 = voter_power.iter().map(|(_, power)| power).sum();

    let response = VotingPower {
        total_voting_power,
        voter_power,
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_governance_parameters(
    governance_engine: web::Data<GovernanceEngine>,
) -> impl Responder {
    let params = GovernanceParameters {
        voting_period: governance_engine.voting_period(),
        quorum_ratio: governance_engine.quorum_ratio(),
        // Add other relevant governance parameters
    };

    HttpResponse::Ok().json(params)
}

#[derive(Serialize)]
struct ProposalInfo {
    id: u64,
    title: String,
    description: String,
    status: String,
    submission_time: u64,
    voting_start_time: Option<u64>,
    voting_end_time: Option<u64>,
}

impl ProposalInfo {
    fn from_proposal(proposal: &Proposal) -> Self {
        Self {
            id: proposal.id,
            title: proposal.title.clone(),
            description: proposal.description.clone(),
            status: format!("{:?}", proposal.status),
            submission_time: proposal.submission_time,
            voting_start_time: proposal.voting_start_time,
            voting_end_time: proposal.voting_end_time,
        }
    }
}

#[derive(Serialize)]
struct VoteInfo {
    voter: String,
    vote_type: String,
    vote_time: u64,
}

impl VoteInfo {
    fn from_vote(vote: &Vote) -> Self {
        Self {
            voter: vote.voter.clone(),
            vote_type: format!("{:?}", vote.vote_type),
            vote_time: vote.vote_time,
        }
    }
}

#[derive(Serialize)]
struct VotingPower {
    total_voting_power: f64,
    voter_power: Vec<(String, f64)>, // (voter, voting_power)
}

#[derive(Serialize)]
struct GovernanceParameters {
    voting_period: u64,
    quorum_ratio: f64,
    // Add other relevant governance parameters
}
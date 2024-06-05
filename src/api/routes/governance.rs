use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::chain::governance::{GovernanceEngine, Proposal, ProposalStatus, Vote, VoteType};

pub fn governance_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/governance")
            .route("/proposals", web::get().to(get_proposals))
            .route("/proposals/{id}", web::get().to(get_proposal))
            .route("/proposals/{id}/votes", web::get().to(get_proposal_votes))
            .route("/voting-power", web::get().to(get_voting_power))
            .route("/parameters", web::get().to(get_governance_parameters)),
    );
}

#[derive(Serialize, Deserialize)]
struct ProposalInfo {
    id: u64,
    title: String,
    description: String,
    status: String,
    submission_time: u64,
    voting_start_time: Option<u64>,
    voting_end_time: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct VoteInfo {
    voter: String,
    vote_type: String,
    vote_time: u64,
}

#[derive(Serialize, Deserialize)]
struct VotingPower {
    total_voting_power: f64,
    voter_power: Vec<(String, f64)>, // (voter, voting_power)
}

#[derive(Serialize, Deserialize)]
struct GovernanceParameters {
    voting_period: u64,
    quorum_ratio: f64,
    // Add other relevant governance parameters
}

async fn get_proposals(governance_engine: web::Data<GovernanceEngine>) -> impl Responder {
    let proposals: Vec<ProposalInfo> = governance_engine
        .proposals()
        .iter()
        .map(|p| ProposalInfo {
            id: p.id,
            title: p.title.clone(),
            description: p.description.clone(),
            status: format!("{:?}", p.status),
            submission_time: p.submission_time,
            voting_start_time: p.voting_start_time,
            voting_end_time: p.voting_end_time,
        })
        .collect();

    HttpResponse::Ok().json(proposals)
}

async fn get_proposal(
    governance_engine: web::Data<GovernanceEngine>,
    id: web::Path<u64>,
) -> impl Responder {
    let proposal = governance_engine.get_proposal(id.into_inner());
    if let Some(p) = proposal {
        let proposal_info = ProposalInfo {
            id: p.id,
            title: p.title.clone(),
            description: p.description.clone(),
            status: format!("{:?}", p.status),
            submission_time: p.submission_time,
            voting_start_time: p.voting_start_time,
            voting_end_time: p.voting_end_time,
        };
        HttpResponse::Ok().json(proposal_info)
    } else {
        HttpResponse::NotFound().body("Proposal not found")
    }
}

async fn get_proposal_votes(
    governance_engine: web::Data<GovernanceEngine>,
    id: web::Path<u64>,
) -> impl Responder {
    let proposal = governance_engine.get_proposal(id.into_inner());
    if let Some(p) = proposal {
        let votes: Vec<VoteInfo> = p
            .votes
            .iter()
            .map(|v| VoteInfo {
                voter: v.voter.clone(),
                vote_type: format!("{:?}", v.vote_type),
                vote_time: v.vote_time,
            })
            .collect();
        HttpResponse::Ok().json(votes)
    } else {
        HttpResponse::NotFound().body("Proposal not found")
    }
}

async fn get_voting_power(governance_engine: web::Data<GovernanceEngine>) -> impl Responder {
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

async fn get_governance_parameters(
    governance_engine: web::Data<GovernanceEngine>,
) -> impl Responder {
    let params = GovernanceParameters {
        voting_period: governance_engine.voting_period(),
        quorum_ratio: governance_engine.quorum_ratio(),
        // Add other relevant governance parameters
    };

    HttpResponse::Ok().json(params)
}
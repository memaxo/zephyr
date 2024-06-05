use std::sync::Arc;
use warp::{Filter, Reply, Rejection, http::StatusCode, http::HeaderMap};
use crate::chain::blockchain::Blockchain;
use crate::chain::transaction::Transaction;
use serde::{Serialize, Deserialize};
use secp256k1::ecdsa::Signature;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::api::middleware::authentication::{AuthenticatedUser, with_auth, ApiError};
use crate::api::middleware::cors::setup_cors;
use crate::api::middleware::xss::reject_xss;
use crate::api::middleware::csrf::protect_csrf;

#[derive(Debug, Serialize, Deserialize)]
struct NewTransactionRequest {
    sender: String,
    receiver: String,
    amount: f64,
    signature: String,
    sp_key: String,
}

async fn handle_submit_transaction(
    new_tx: NewTransactionRequest,
    blockchain: Arc<Blockchain>,
    _user: AuthenticatedUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate transaction amount
    if new_tx.amount <= 0.0 {
        return Err(warp::reject::custom(ApiError::NegativeAmount));
    }

    // Parse and validate signature
    let signature = hex::decode(&new_tx.signature)
        .map_err(|_| warp::reject::custom(ApiError::InvalidSignatureFormat))
        .and_then(|sig| {
            Signature::from_der(&sig).map_err(|_| warp::reject::custom(ApiError::InvalidSignatureFormat))
        })?;

    // Parse and validate SP key
    let sp_key = hex::decode(&new_tx.sp_key)
        .map_err(|_| warp::reject::custom(ApiError::InvalidSPKeyFormat))?;

    // Parse and validate sender's public key
    let sender = PublicKey::from_str(&new_tx.sender)
        .map_err(|_| warp::reject::custom(ApiError::InvalidPublicKeyFormat))?;

    let transaction = Transaction {
        sender,
        receiver: new_tx.receiver,
        amount: new_tx.amount,
        signature,
        sp_key,
        proof: /* value for proof */,
        encrypted_details: /* value for encrypted_details */,
        post_quantum_signature: /* value for post_quantum_signature */,
    };

    blockchain
        .add_transaction(transaction)
        .map(|_| warp::reply::with_status("Transaction submitted successfully", StatusCode::OK))
        .map_err(|e| {
            warp::reply::with_status(
                format!("Failed to submit transaction: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
        })
}

async fn handle_query_block(
    block_number: u64,
    blockchain: Arc<Blockchain>,
    _user: AuthenticatedUser,
) -> Result<impl Reply, Rejection> {
    blockchain
        .get_block(block_number)
        .map(|block| warp::reply::json(&block))
        .ok_or_else(|| warp::reject::custom(ApiError::BlockNotFound))
}

pub fn api(blockchain: Arc<Blockchain>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = setup_cors();
    let auth = with_auth();

    let submit_transaction = warp::post()
        .and(warp::path("transactions"))
        .and(warp::body::json())
        .and(with_blockchain(blockchain.clone()))
        .and(auth.clone())
        .and_then(handle_submit_transaction);

    let query_block = warp::get()
        .and(warp::path!("blocks" / u64))
        .and(with_blockchain(blockchain.clone()))
        .and(auth.clone())
        .and_then(handle_query_block);

    submit_transaction
        .or(query_block)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_rejection)
}

fn with_blockchain(blockchain: Arc<Blockchain>) -> impl Filter<Extract = (Arc<Blockchain>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || blockchain.clone())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(api_err) = err.find::<ApiError>() {
        Ok(warp::reply::with_status(
            api_err.to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Err(err)
    }
}
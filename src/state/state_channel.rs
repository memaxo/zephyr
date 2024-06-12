use crate::chain::{blockchain::Blockchain, transaction::Transaction};
use crate::crypto::{verify_signature, PublicKey};
use crate::network::quantum_resistant::{QuantumResistantConnection, QuantumResistantConnectionManager};
use crate::network::state_channel_message::{StateChannelMessage, StateChannelMessageHandler};
use crate::qup::crypto::QUPCrypto;
use crate::qup::types::{QUPSignature, UsefulWorkSolution};
use crate::utils::error::StateChannelError;
use log::{debug, error, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use warp::http::Responseuse;
use zeroize::Zeroize;

const MAX_OFF_CHAIN_TRANSACTIONS: usize = 100;

#[derive(pub struct OffChainTransaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: QUPSignature,
    pub useful_work: Option<UsefulWorkSolution>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dispute {
    pub disputed_transaction: OffChainTransaction,
    pub reason: String,
    pub resolution: Option<DisputeResolution>,
    pub arbitration: Option<Arbitration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DisputeResolution {
    AdjustedTransaction(OffChainTransaction),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArbitrationStatus {
    Requested,
    InProgress { arbitrator: String },
    Resolved { resolution: DisputeResolution },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arbitration {
    pub status: ArbitrationStatus,
    pub arbitrator_details: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateChannelOptimized {
    pub id: String,
    pub parties: [String; 2],
    pub balance: BTreeMap<String, f64>,
    pub off_chain_transactions: VecDeque<OffChainTransaction>,
    pub closed: bool,
    pub dispute: Option<Dispute>,
}

impl StateChannelOptimized {
    pub fn new(id: String, party1: String, party2: String) -> Self {
        let mut balance = BTreeMap::new();
        balance.insert(party1.clone(), 0.0);
        balance.insert(party2.clone(), 0.0);

        StateChannelOptimized {
            id,
            parties: [party1, party2],
            balance,
            off_chain_transactions: VecDeque::new(),
            closed: false,
            dispute: None,
        }
    }

    pub fn add_transaction_optimized(
            &mut self,
            transaction: OffChainTransaction,
            qup_crypto: &QUPCrypto,
        ) -> Result<(), StateChannelError> {
            if self.closed {
                return Err(StateChannelError::ChannelClosed);
            }
            if let Some(dispute) = &self.dispute {
                return Err(StateChannelError::ChannelInDispute(dispute.reason.clone()));
            }
            if self.parties.contains(&transaction.sender) && self.parties.contains(&transaction.receiver) {
                let sender_public_key = PublicKey::from_party(&transaction.sender);
                if qup_crypto.verify_transaction_signature(&transaction)? {
                    *self.balance.entry(transaction.sender.clone()).or_insert(0.0) -= transaction.amount;
                    *self.balance.entry(transaction.receiver.clone()).or_insert(0.0) += transaction.amount;
                    self.off_chain_transactions.push_back(transaction);
                    debug!("Transaction added to state channel: {:?}", self.id);
                    Ok(())
                } else {
                    Err(StateChannelError::InvalidSignature)
                }
            } else {
                Err(StateChannelError::InvalidTransactionParties)
            }
        }

    pub async fn close_channel(&mut self, blockchain: &mut Blockchain) -> Result<(), StateChannelError> {
        let lock = self.acquire_closure_lock().await;
        if self.closed {
            return Err(StateChannelError::ChannelAlreadyClosed);
        }

        if self.dispute.is_some() {
            return Err(StateChannelError::ChannelInDispute("Dispute must be resolved before closing".to_string()));
        }

        let result = self.execute_channel_closure(blockchain).await;
        lock.unlock().await;
        result
    }

    async fn acquire_closure_lock(&self) -> DistributedLock {
        // Implement logic to acquire a distributed lock for channel closure
        // This could involve using a consensus-based approach or a lock service
        // Return the acquired lock
        unimplemented!()
    }

    async fn execute_channel_closure(&mut self, blockchain: &mut Blockchain) -> Result<(), StateChannelError> {

        let final_state_transaction = Transaction {
            sender: self.parties[0].clone(),
            receiver: self.parties[1].clone(),
            amount: self.balance[&self.parties[1]].clone(),
            signature: vec![],
        };

        match blockchain.add_transaction(final_state_transaction).await {
            Ok(_) => {
                self.closed = true;
                info!("State channel closed: {:?}", self.id);
                Ok(())
            }
            Err(e) => Err(StateChannelError::FailedToFinalizeState(format!("{}", e))),
        }
    }

    pub fn initiate_dispute(&mut self, transaction: OffChainTransaction, reason: String) -> Result<(), StateChannelError> {
        if self.closed {
            Err(StateChannelError::ChannelAlreadyClosed)
        } else if !self.off_chain_transactions.contains(&transaction) {
            Err(StateChannelError::TransactionNotFound)
        } else {
            self.dispute = Some(Dispute {
                disputed_transaction: transaction,
                reason,
                resolution: None,
                arbitration: None,
            });
            debug!("Dispute initiated in state channel: {:?}", self.id);
            Ok(())
        }
    }

    pub fn resolve_dispute(&mut self, resolution: DisputeResolution) -> Result<(), StateChannelError> {
        if self.closed {
            Err(StateChannelError::ChannelClosed)
        } else if let Some(dispute) = &mut self.dispute {
            dispute.resolution = Some(resolution.clone());
            self.dispute = None;
            info!("Dispute resolved in state channel: {:?}", self.id);
            Ok(())
        } else {
            Err(StateChannelError::NoDisputeToResolve)
        }
    }

    pub fn resolve_dispute_with_modifications(&mut self, resolution: DisputeResolution, modifications: Vec<OffChainTransaction>) -> Result<(), StateChannelError> {
        if self.closed {
            Err(StateChannelError::ChannelClosed)
        } else if let Some(dispute) = &mut self.dispute {
            dispute.resolution = Some(resolution.clone());
            self.off_chain_transactions.retain(|tx| !modifications.contains(tx));
            self.off_chain_transactions.extend(modifications);
            self.dispute = None;
            info!("Dispute resolved with modifications in state channel: {:?}", self.id);
            Ok(())
        } else {
            Err(StateChannelError::NoDisputeToResolve)
        }
    }

    pub fn start_arbitration(&mut self, arbitrator: String) -> Result<(), StateChannelError> {
        if let Some(dispute) = &mut self.dispute {
            if let Some(Arbitration { status: ArbitrationStatus::Requested, .. }) = dispute.arbitration {
                dispute.arbitration = Some(Arbitration {
                    status: ArbitrationStatus::InProgress { arbitrator },
                    arbitrator_details: dispute.arbitration.as_ref().unwrap().arbitrator_details.clone(),
                });
                debug!("Arbitration started in state channel: {:?}", self.id);
                Ok(())
            } else {
                Err(StateChannelError::ArbitrationNotRequested)
            }
        } else {
            Err(StateChannelError::NoDisputeToArbitrate)
        }
    }

    pub fn conclude_arbitration(&mut self, resolution: DisputeResolution) -> Result<(), StateChannelError> {
        if let Some(dispute) = &mut self.dispute {
            if let Some(Arbitration { status: ArbitrationStatus::InProgress { .. }, .. }) = dispute.arbitration {
                dispute.arbitration = Some(Arbitration {
                    status: ArbitrationStatus::Resolved { resolution: resolution.clone() },
                    arbitrator_details: dispute.arbitration.as_ref().unwrap().arbitrator_details.clone(),
                });
                info!("Arbitration concluded in state channel: {:?}", self.id);
                self.resolve_dispute(resolution)
            } else {
                Err(StateChannelError::ArbitrationNotInProgress)
            }
        } else {
            Err(StateChannelError::NoDisputeToArbitrate)
        }
    }

    pub fn conclude_arbitration(&mut self, resolution: DisputeResolution) -> Result<(), StateChannelError> {
        if let Some(dispute) = &mut self.dispute {
            if let Some(Arbitration { status: ArbitrationStatus::InProgress { .. }, .. }) = dispute.arbitration {
                dispute.arbitration = Some(Arbitration {
                    status: ArbitrationStatus::Resolved { resolution: resolution.clone() },
                    arbitrator_details: dispute.arbitration.as_ref().unwrap().arbitrator_details.clone(),
                });
                info!("Arbitration concluded in state channel: {:?}", self.id);
                self.resolve_dispute(resolution)
            } else {
                Err(StateChannelError::ArbitrationNotInProgress)
            }
        } else {
            Err(StateChannelError::NoDisputeToArbitrate)
        }
    }
}

pub struct ChannelManagerOptimized {
    pub channels: Arc<RwLock<HashMap<String, StateChannelOptimized>>>,
    message_handler: Arc<StateChannelMessageHandler>,
    connection_manager: Arc<QuantumResistantConnectionManager>,
    qup_crypto: Arc<QUPCrypto>,
}

impl ChannelManagerOptimized {
    pub fn new(
        message_handler: Arc<StateChannelMessageHandler>,
        connection_manager: Arc<QuantumResistantConnectionManager>,
        qup_crypto: Arc<QUPCrypto>,
    ) -> Self {
        ChannelManagerOptimized {
            channels: Arc::new(RwLock::new(HashMap::new())),
            message_handler,
            connection_manager,
            qup_crypto,
        }
    }

    pub fn create_channel(&mut self, id: String, party1: String, party2: String) -> Result<(), StateChannelError> {
        let mut channels = self.channels.write().unwrap();
        if channels.contains_key(&id) {
            Err(StateChannelError::ChannelAlreadyExists(id))
        } else {
            let new_channel = StateChannelOptimized::new(id.clone(), party1, party2);
            channels.insert(id.clone(), new_channel);
            debug!("State channel created: {:?}", id);
            Ok(())
        }
    }

    pub fn add_off_chain_transaction_optimized(
            &mut self,
            channel_id: &str,
            transaction: OffChainTransaction,
        ) -> Result<(), StateChannelError> {
            let mut channels = self.channels.write().unwrap();
            channels
                .get_mut(channel_id)
                .ok_or_else(|| StateChannelError::ChannelNotFound(channel_id.to_string()))
                .and_then(|channel| channel.add_transaction_optimized(transaction, &self.qup_crypto))
        }


    pub fn initiate_dispute(&mut self, channel_id: &str, transaction: OffChainTransaction, reason: String) -> Result<(), StateChannelError> {
        let mut channels = self.channels.write().unwrap();
        channels
            .get_mut(channel_id)
            .ok_or_else(|| StateChannelError::ChannelNotFound(channel_id.to_string()))
            .and_then(|channel| channel.initiate_dispute(transaction, reason))
    }

    pub fn resolve_dispute(&mut self, channel_id: &str, resolution: DisputeResolution) -> Result<(), StateChannelError> {
        let mut channels = self.channels.write().unwrap();
        channels
            .get_mut(channel_id)
            .ok_or_else(|| StateChannelError::ChannelNotFound(channel_id.to_string()))
            .and_then(|channel| channel.resolve_dispute(resolution))
    }

    pub async fn close_channel(&mut self, channel_id: &str, blockchain: &mut Blockchain) -> Result<(), StateChannelError> {
        let mut channels = self.channels.write().unwrap();
        channels
            .get_mut(channel_id)
            .ok_or_else(|| StateChannelError::ChannelNotFound(channel_id.to_string()))
            .and_then(|channel| channel.close_channel(blockchain))
    }

    pub async fn manage_channels_optimized(&mut self, blockchain: Arc<RwLock<Blockchain>>) {
        let channels = self.channels.read().unwrap();
        channels.par_iter().for_each(|(id, channel)| {
            if !channel.closed && channel.off_chain_transactions.len() > MAX_OFF_CHAIN_TRANSACTIONS {
                let mut blockchain = blockchain.write().unwrap();
                match channel.close_channel(&mut blockchain) {
                    Ok(_) => info!("Successfully closed state channel: {}", id),
                    Err(e) => error!("Error closing state channel {}: {:?}", id, e),
                }
            }
        });
    }

    pub async fn start_message_handler(&mut self) {
            let (tx, mut rx) = mpsc::channel(1024);
            let channels = self.channels.clone();
            let connection_manager = self.connection_manager.clone();
            let qup_crypto = self.qup_crypto.clone();

            tokio::spawn(async move {
                while let Some((channel_id, message)) = rx.recv().await {
                    let mut channels = channels.write().unwrap();
                    if let Some(channel) = channels.get_mut(&channel_id) {
                        match message {
                            StateChannelMessage::Transaction(transaction) => {
                                if let Err(e) = channel.add_transaction_optimized(transaction, &qup_crypto) {
                                    error!("Failed to add transaction to state channel {}: {:?}", channel_id, e);
                                }
                            }
                            StateChannelMessage::DisputeInitiation { transaction, reason } => {
                                if let Err(e) = channel.initiate_dispute(transaction, reason) {
                                    error!("Failed to initiate dispute in state channel {}: {:?}", channel_id, e);
                                }
                            }
                            StateChannelMessage::DisputeResolution(resolution) => {
                                if let Err(e) = channel.resolve_dispute(resolution) {
                                    error!("Failed to resolve dispute in state channel {}: {:?}", channel_id, e);
                                }
                            }
                        }
                    }
                }
            });

            self.message_handler.set_channel(tx).await;
        }
}

pub fn state_channel_api(
    manager: Arc<RwLock<ChannelManagerOptimized>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("state_channel")
        .and(warp::post())
        .and(create_channel_route(manager.clone()))
        .or(send_transaction_route(manager.clone()))
        .or(query_channel_state_route(manager))
}

fn create_channel_route(
    manager: Arc<RwLock<ChannelManagerOptimized>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("create")
        .and(warp::body::json())
        .map(move |body: HashMap<String, String>| {
            let mut manager = manager.write().unwrap();
            match manager.create_channel(body["id"].clone(), body["party1"].clone(), body["party2"].clone()) {
                Ok(_) => Response::builder().status(200).body("Channel created successfully".to_string()).unwrap(),
                Err(e) => Response::builder().status(400).body(format!("{:?}", e)).unwrap(),
            }
        })
}

fn send_transaction_route(
    manager: Arc<RwLock<ChannelManagerOptimized>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("send_transaction")
        .and(warp::body::json())
        .map(move |body: HashMap<String, serde_json::Value>| {
            let mut manager = manager.write().unwrap();
            let transaction: OffChainTransaction = serde_json::from_value(body["transaction"].clone()).unwrap();
            match manager.add_off_chain_transaction_optimized(&body["channel_id"].as_str().unwrap(), transaction) {
                Ok(_) => Response::builder().status(200).body("Transaction sent successfully".to_string()).unwrap(),
                Err(e) => Response::builder().status(400).body(format!("{:?}", e)).unwrap(),
            }
        })
}

fn query_channel_state_route(
    manager: Arc<RwLock<ChannelManagerOptimized>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("query_state")
        .and(warp::query::<HashMap<String, String>>())
        .map(move |query: HashMap<String, String>| {
            let manager = manager.read().unwrap();
            if let Some(channel) = manager.channels.read().unwrap().get(&query["id"]) {
                let response = serde_json::json!({
                    "id": channel.id,
                    "parties": channel.parties,
                    "balance": channel.balance,
                    "closed": channel.closed,
                    "dispute": channel.dispute,
                });
                Response::builder().status(200).body(response.to_string()).unwrap()
            } else {
                Response::builder().status(404).body(format!("Channel not found: {}", query["id"])).unwrap()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_keypair;
    use crate::qup::crypto::QUPCrypto;
    use crate::utils::test_utils::TestBlockchain;

    #[tokio::test]
    async fn test_create_state_channel() {
        let message_handler = Arc::new(StateChannelMessageHandler::default());
        let connection_manager = Arc::new(QuantumResistantConnectionManager::default());
        let qup_crypto = Arc::new(QUPCrypto::default());
        let mut manager = ChannelManagerOptimized::new(message_handler, connection_manager, qup_crypto);
        let channel_id = "test_channel".to_string();
        let party1 = "party1".to_string();
        let party2 = "party2".to_string();

        assert!(manager.create_channel(channel_id.clone(), party1.clone(), party2.clone()).is_ok());
        assert!(manager.channels.read().unwrap().contains_key(&channel_id));
        assert_eq!(manager.channels.read().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_add_off_chain_transaction() {
        let message_handler = Arc::new(StateChannelMessageHandler::default());
        let connection_manager = Arc::new(QuantumResistantConnectionManager::default());
        let qup_crypto = Arc::new(QUPCrypto::default());
        let mut manager = ChannelManagerOptimized::new(message_handler, connection_manager, qup_crypto.clone());
        let channel_id = "test_channel".to_string();
        let party1 = "party1".to_string();
        let party2 = "party2".to_string();

        let (party1_pubkey, party1_privkey) = generate_keypair();
        let transaction = OffChainTransaction {
            sender: party1.clone(),
            receiver: party2.clone(),
            amount: 100.0,
            signature: qup_crypto.sign_transaction(&party1_privkey, "test_transaction".as_bytes()).unwrap(),
            useful_work: None,
        };

        manager.create_channel(channel_id.clone(), party1.clone(), party2.clone()).unwrap();
        assert!(manager.add_off_chain_transaction_optimized(&channel_id, transaction).is_ok());

        let channel = manager.channels.read().unwrap().get(&channel_id).unwrap();
        assert_eq!(channel.off_chain_transactions.len(), 1);
        assert_eq!(channel.balance[&party1], -100.0);
        assert_eq!(channel.balance[&party2], 100.0);
    }

    #[tokio::test]
    async fn test_close_state_channel() {
        let message_handler = Arc::new(StateChannelMessageHandler::default());
        let connection_manager = Arc::new(QuantumResistantConnectionManager::default());
        let qup_crypto = Arc::new(QUPCrypto::default());
        let mut manager = ChannelManagerOptimized::new(message_handler, connection_manager, qup_crypto.clone());
        let channel_id = "test_channel".to_string();
        let party1 = "party1".to_string();
        let party2 = "party2".to_string();

        let (party1_pubkey, party1_privkey) = generate_keypair();
        let transaction = OffChainTransaction {
            sender: party1.clone(),
            receiver: party2.clone(),
            amount: 100.0,
            signature: qup_crypto.sign_transaction(&party1_privkey, "test_transaction".as_bytes()).unwrap(),
            useful_work: None,
        };

        manager.create_channel(channel_id.clone(), party1.clone(), party2.clone()).unwrap();
        manager.add_off_chain_transaction_optimized(&channel_id, transaction).unwrap();

        let mut test_blockchain = TestBlockchain::new();
        assert!(manager.close_channel(&channel_id, &mut test_blockchain).await.is_ok());

        let channel = manager.channels.read().unwrap().get(&channel_id).unwrap();
        assert!(channel.closed);
        assert_eq!(test_blockchain.transactions.len(), 1);
    }
}

use crate::chain::transaction::Transaction;
use crate::secure_core::secure_vault::SecureVault;
use crate::consensus::consensus_config::ConsensusConfig;
use crate::network::network_message::{NetworkMessage, ShardMessage};
use crate::utils::compression::{compress_data, decompress_data, CompressionError, DecompressionError};
use crate::utils::encryption::{EncryptionKey, EncryptionError, DecryptionError};
use aes_gcm::{Aes256Gcm, aead::{Aead, NewAead}, Nonce};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::mpsc::{self, Sender, Receiver};

const PRUNE_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours
const TRANSACTION_RETENTION_PERIOD: Duration = Duration::from_secs(604800); // 1 week


#[derive(Debug, Error)]
pub enum ShardError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error sending message to shard: {0}")]
    MessageSendError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(#[from] EncryptionError),
    #[error("Decryption error: {0}")]
    DecryptionError(#[from] DecryptionError),
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Compression error: {0}")]
    CompressionError(#[from] CompressionError),
    #[error("Decompression error: {0}")]
    DecompressionError(#[from] DecompressionError),
    #[error("Failed to generate unique nonce")]
    NonceGenerationError,
}

pub struct Shard {
    pub transactions: Arc<RwLock<VecDeque<Vec<u8>>>>,
    pub incoming_messages: Receiver<NetworkMessage>,
    encryption_key: Arc<RwLock<EncryptionKey>>,
    pub outgoing_messages: Sender<NetworkMessage>,
    pub shard_channels: HashMap<u64, Sender<NetworkMessage>>,
    pub shard_id: u64,
    pub total_shards: u64,
    last_prune_time: Instant,
    consensus_config: ConsensusConfig,
    nonce_counter: Arc<RwLock<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardState {
    pub shard_id: u64,
    pub transactions: Vec<Transaction>,
}

impl Hash for ShardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.shard_id.hash(state);
        for transaction in &self.transactions {
            transaction.hash(state);
        }
    }
}

impl Shard {
    pub fn new(shard_id: u64, total_shards: u64, encryption_key: EncryptionKey, consensus_config: ConsensusConfig) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        let encryption_key = Arc::new(RwLock::new(encryption_key));
        let nonce_counter = Arc::new(RwLock::new(0));
        Shard {
            transactions: Arc::new(RwLock::new(VecDeque::new())),
            incoming_messages: rx,
            encryption_key,
            outgoing_messages: tx,
            shard_channels: HashMap::new(),
            shard_id,
            total_shards,
            last_prune_time: Instant::now(),
            consensus_config,
            nonce_counter,
        }
    }

    pub async fn process_incoming_messages(&mut self, secure_vault: &SecureVault) -> Result<(), ShardError> {
        while let Some(message) = self.incoming_messages.recv().await {
            match message {
                NetworkMessage::Transaction(compressed_transaction) => {
                    let transaction = self.decompress_and_decrypt_data(&compressed_transaction, secure_vault)?;
                    self.verify_transaction(&transaction)?;
                    self.encrypt_and_compress_transaction(transaction, secure_vault)?;
                }
                NetworkMessage::ShardMessage(shard_message) => {
                    self.handle_shard_message(shard_message).await?;
                }
                _ => {
                    debug!("Received unsupported network message type");
                }
            }
        }
        Ok(())
    }

    pub async fn send_message_to_shard(&self, shard_id: u64, message: NetworkMessage) -> Result<(), ShardError> {
        self.shard_channels.get(&shard_id)
            .ok_or_else(|| ShardError::MessageSendError(format!("Shard channel not found for shard {}", shard_id)))
            .and_then(|sender| sender.send(message).await.map_err(|e| ShardError::MessageSendError(format!("Failed to send message to shard {}: {}", shard_id, e))))
    }

    pub async fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ShardError> {
        let shard_id = self.calculate_shard_for_transaction(&transaction);
        if shard_id == self.shard_id {
            self.compress_and_store_transaction(transaction)?;
            Ok(())
        } else {
            let compressed_transaction = self.compress_data(&transaction)?;
            let message = NetworkMessage::Transaction(compressed_transaction);
            self.send_message_to_shard(shard_id, message).await
        }
    }

    fn encrypt_and_compress_transaction(&self, transaction: Transaction, secure_vault: &SecureVault) -> Result<(), ShardError> {
        let encryption_key = self.encryption_key.read().map_err(|_| ShardError::EncryptionError("Failed to acquire read lock for encryption key".to_string()))?;
        let nonce = self.generate_unique_nonce()?;
        let encrypted_transaction = secure_vault.encrypt_transaction(&transaction, &encryption_key, &nonce).map_err(ShardError::EncryptionError)?;
        let compressed_transaction = compress_data(&encrypted_transaction).map_err(ShardError::CompressionError)?;
        let mut transactions = self.transactions.write().map_err(|_| ShardError::SerializationError("Failed to acquire write lock for transactions".to_string()))?;
        transactions.push_back(compressed_transaction);
        Ok(())
    }

    fn decompress_and_decrypt_data(&self, compressed_data: &[u8], secure_vault: &SecureVault) -> Result<Transaction, ShardError> {
        let decompressed_data = decompress_data(compressed_data).map_err(ShardError::DecompressionError)?;
        let decryption_key = self.encryption_key.read().map_err(|_| ShardError::DecryptionError("Failed to acquire read lock for decryption key".to_string()))?;
        let nonce = self.generate_unique_nonce()?;
        let decrypted_data = secure_vault.decrypt_transaction(&decompressed_data, &decryption_key, &nonce).map_err(ShardError::DecryptionError)?;
        serde_json::from_slice(&decrypted_data).map_err(ShardError::SerializationError)
    }

    fn generate_unique_nonce(&self) -> Result<Nonce, ShardError> {
        let mut nonce_counter = self.nonce_counter.write().map_err(|_| ShardError::NonceGenerationError)?;
        let nonce_value = *nonce_counter;
        *nonce_counter += 1;
        let nonce_bytes = nonce_value.to_be_bytes();
        let nonce = Nonce::from_slice(&nonce_bytes[..12]);
        nonce.map_or(Err(ShardError::NonceGenerationError), |n| Ok(n))
    }

    pub async fn prune_state(&mut self) {
        if self.last_prune_time.elapsed() >= PRUNE_INTERVAL {
            self.transactions.write().unwrap().retain(|transaction| {
                let transaction: Transaction = match serde_json::from_slice(transaction) {
                    Ok(tx) => tx,
                    Err(e) => {
                        error!("Failed to deserialize transaction during pruning: {}", e);
                        return false;
                    }
                };
                transaction.timestamp() > Instant::now() - TRANSACTION_RETENTION_PERIOD
            });

            self.last_prune_time = Instant::now();
            info!("Shard {} state pruned.", self.shard_id);
        }
    }

    async fn synchronize_state(&self) {
        let shard_states: Vec<ShardState> = futures::future::join_all(self.shard_channels.iter().map(|(&shard_id, tx)| {
            let request = ShardMessage::StateRequest { shard_id: self.shard_id };
            let message = NetworkMessage::ShardMessage(request);
            tx.send(message).then(|result| async {
                match result {
                    Ok(_) => {
                        match self.incoming_messages.recv().await {
                            Some(NetworkMessage::ShardMessage(ShardMessage::StateResponse(state))) => Ok(state),
                            Some(_) => Err(ShardError::MessageSendError("Unexpected response type".to_string())),
                            None => Err(ShardError::MessageSendError("Failed to receive state response".to_string())),
                        }
                    },
                    Err(e) => Err(ShardError::MessageSendError(format!("Failed to send state request: {}", e))),
                }
            })
        })).await.into_iter().filter_map(Result::ok).collect();

        let mut transactions = self.transactions.write().map_err(|_| ShardError::SerializationError("Failed to acquire write lock for transactions".to_string())).unwrap();
        for state in shard_states {
            if state.shard_id != self.shard_id {
                self.merge_state(&mut transactions, state);
            }
        }

        info!("State synchronization completed for shard {}", self.shard_id);
    }


    fn merge_state(&self, transactions: &mut VecDeque<Vec<u8>>, other_state: ShardState) {
        for transaction in other_state.transactions {
            let compressed_transaction = compress_data(&transaction).map_err(|e| {
                warn!("Failed to compress transaction during merge: {}", e);
                ShardError::CompressionError(e)
            }).unwrap();
            if !transactions.contains(&compressed_transaction) {
                transactions.push_back(compressed_transaction);
            }
        }
    }

    pub fn verify_transaction(&self, transaction: &Transaction) -> Result<(), ShardError> {
        if transaction.amount() <= 0.0 {
            Err(ShardError::InvalidTransaction("Transaction amount must be positive".to_string()))
        } else {
            // Add more transaction verification logic here
            Ok(())
        }
    }

    fn compress_and_store_transaction(&mut self, transaction: Transaction) -> Result<(), ShardError> {
        let compressed_transaction = compress_data(&transaction).map_err(ShardError::CompressionError)?;
        let mut transactions = self.transactions.write().map_err(|_| ShardError::SerializationError("Failed to acquire write lock for transactions".to_string()))?;
        transactions.push_back(compressed_transaction);
        Ok(())
    }

    fn calculate_shard_for_transaction(&self, transaction: &Transaction) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        transaction.hash(&mut hasher);
        hasher.finish() % self.total_shards
    }

    async fn handle_shard_message(&mut self, message: ShardMessage) -> Result<(), ShardError> {
        match message {
            ShardMessage::StateRequest { shard_id } => {
                let transactions = self.transactions.read().map_err(|_| ShardError::SerializationError("Failed to acquire read lock for transactions".to_string()))?;
                let state = ShardState {
                    shard_id: self.shard_id,
                    transactions: transactions.iter().map(|tx| {
                        serde_json::from_slice(tx).map_err(|e| {
                            warn!("Failed to deserialize transaction during state request: {}", e);
                            ShardError::SerializationError(e)
                        })
                    }).collect::<Result<_, _>>()?,
                };
                let response = ShardMessage::StateResponse(state);
                self.send_message_to_shard(shard_id, NetworkMessage::ShardMessage(response)).await?;
            }
            ShardMessage::StateResponse(state) => {
                if state.shard_id != self.shard_id {
                    warn!("Received state response from unexpected shard: {}", state.shard_id);
                    return Ok(());
                }
                let mut transactions = self.transactions.write().map_err(|_| ShardError::SerializationError("Failed to acquire write lock for transactions".to_string()))?;
                self.merge_state(&mut transactions, state);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ShardMessage {
    StateRequest { shard_id: u64 },
    StateResponse(ShardState),
}
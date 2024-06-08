use crate::chain::shard::ShardState;
use crate::chain::transaction::Transaction;
use crate::consensus::qup::{QUPBlock, QUPBlockHeader, QUPUsefulWork, QUPVote};
use crate::qup::crypto::{QUPCrypto, QUPSignature};
use crate::utils::error::NetworkError;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use zstd::stream::{decode_all, encode_all};

#[derive(Debug, Serialize, Deserialize)]
pub enum ShardMessage {
    TransactionMessage(Transaction),
    StateRequest {
        shard_id: u64,
    },
    StateResponse(ShardState),
    QUPShardBlockProposal(QUPBlock),
    QUPShardBlockCommit {
        block_hash: String,
        signature: QUPSignature,
    },
    QUPShardVote(QUPVote),
    QUPShardUsefulWork(QUPUsefulWork),
    QKDKeyRequestShard,
    QKDKeyResponseShard(QKDKey),
    QKDKeyConfirmationShard,
    QuantumStateDistributionShard(QuantumState),
    QuantumStateMeasurementResultsShard(Vec<bool>),
    ShardSyncRequest {
        shard_id: u64,
        sync_hash: String,
    },
    ShardSyncResponse {
        shard_id: u64,
        sync_data: Vec<u8>,
    },
    CrossShardTransactionMessage {
        transaction: Transaction,
        source_shard_id: u64,
        target_shard_id: u64,
    },
    ShardBlockProposalMessage {
        block: Block,
        shard_id: u64,
    },
    ShardBlockCommitMessage {
        block_hash: String,
        shard_id: u64,
    },
}

impl ShardMessage {
    pub fn serialize(&self, crypto: &QUPCrypto) -> Result<Vec<u8>, NetworkError> {
        let serialized_data = bincode::serialize(self)
            .map_err(|e| NetworkError::SerializationFailed(e.to_string()))?;
        let compressed_data = encode_all(&serialized_data[..], 3)
            .map_err(|e| NetworkError::CompressionFailed(e.to_string()))?;
        let encrypted_data = crypto.encrypt(&compressed_data)?;
        Ok(encrypted_data)
    }

    pub fn deserialize(data: &[u8], crypto: &QUPCrypto) -> Result<Self, NetworkError> {
        let decrypted_data = crypto.decrypt(data)?;
        let decompressed_data = decode_all(&decrypted_data)
            .map_err(|e| NetworkError::DecompressionFailed(e.to_string()))?;
        bincode::deserialize(&decompressed_data)
            .map_err(|e| NetworkError::DeserializationFailed(e.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct ShardMessageHandler {
    message_sender: Option<Sender<(u64, ShardMessage)>>,
    crypto: QUPCrypto,
}

impl ShardMessageHandler {
    pub fn new(crypto: QUPCrypto) -> Self {
        ShardMessageHandler {
            message_sender: None,
            crypto,
        }
    }

    pub async fn set_message_sender(&mut self, sender: Sender<(u64, ShardMessage)>) {
        self.message_sender = Some(sender);
    }

    pub async fn send_message(
        &self,
        shard_id: u64,
        message: ShardMessage,
    ) -> Result<(), NetworkError> {
        if let Some(sender) = &self.message_sender {
            let serialized_message = message.serialize(&self.crypto)?;
            sender
                .send((shard_id, serialized_message))
                .await
                .map_err(|e| {
                    NetworkError::MessageSendingFailed(format!(
                        "Failed to send shard message: {}",
                        e
                    ))
                })?;
            Ok(())
        } else {
            Err(NetworkError::MessageSendingFailed(
                "Shard message sender not initialized".to_string(),
            ))
        }
    }

    pub async fn send_transaction(
        &self,
        shard_id: u64,
        transaction: Transaction,
    ) -> Result<(), NetworkError> {
        let message = ShardMessage::TransactionMessage(transaction);
        self.send_message(shard_id, message).await
    }

    pub async fn request_shard_state(&self, shard_id: u64) -> Result<(), NetworkError> {
        let message = ShardMessage::StateRequest { shard_id };
        self.send_message(shard_id, message).await
    }

    pub async fn send_shard_state(
        &self,
        shard_id: u64,
        state: ShardState,
    ) -> Result<(), NetworkError> {
        let message = ShardMessage::StateResponse(state);
        self.send_message(shard_id, message).await
    }

    pub async fn handle_message(&mut self, shard_id: u64, serialized_message: Vec<u8>) {
            let message = match ShardMessage::deserialize(&serialized_message, &self.crypto) {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Failed to deserialize shard message: {}", e);
                    return;
                }
            };

            match message {
                ShardMessage::TransactionMessage(transaction) => {
                    self.handle_transaction_message(shard_id, transaction)
                        .await;
                }
                ShardMessage::StateRequest { shard_id } => {
                    self.handle_state_request(shard_id).await;
                }
                ShardMessage::StateResponse(state) => {
                    self.handle_state_response(shard_id, state).await;
                }
                ShardMessage::QUPShardBlockProposal {
                    block_header,
                    transactions,
                    useful_work,
                    signature,
                } => {
                    self.handle_qup_shard_block_proposal(
                        shard_id,
                        block_header,
                        transactions,
                        useful_work,
                        signature,
                    )
                    .await;
                }
                ShardMessage::QUPShardBlockCommit {
                    block_header,
                    signature,
                } => {
                    self.handle_qup_shard_block_commit(shard_id, block_header, signature)
                        .await;
                }
                ShardMessage::QUPShardVote(vote) => {
                    self.handle_qup_shard_vote(shard_id, vote).await;
                }
                ShardMessage::QUPShardUsefulWork(useful_work) => {
                    self.handle_qup_shard_useful_work(shard_id, useful_work)
                        .await;
                }
                ShardMessage::QKDKeyRequestShard => {
                    self.handle_qkd_key_request_shard(shard_id).await;
                }
                ShardMessage::QKDKeyResponseShard(key) => {
                    self.handle_qkd_key_response_shard(shard_id, key).await;
                }
                ShardMessage::QKDKeyConfirmationShard => {
                    self.handle_qkd_key_confirmation_shard(shard_id).await;
                }
                ShardMessage::QuantumStateDistributionShard(state) => {
                    self.handle_quantum_state_distribution_shard(shard_id, state)
                        .await;
                }
                ShardMessage::QuantumStateMeasurementResultsShard(results) => {
                    self.handle_quantum_state_measurement_results_shard(shard_id, results)
                        .await;
                }
                ShardMessage::ShardSyncRequest { shard_id, sync_hash } => {
                    self.handle_shard_sync_request(shard_id, sync_hash).await;
                }
                ShardMessage::ShardSyncResponse { shard_id, sync_data } => {
                    self.handle_shard_sync_response(shard_id, sync_data).await;
                }
                ShardMessage::CrossShardTransactionMessage {
                    transaction,
                    source_shard_id,
                    target_shard_id,
                } => {
                    self.handle_cross_shard_transaction(transaction, source_shard_id, target_shard_id)
                        .await;
                }
                ShardMessage::ShardBlockProposalMessage { block, shard_id } => {
                    self.handle_shard_block_proposal(block, shard_id).await;
                }
                ShardMessage::ShardBlockCommitMessage { block_hash, shard_id } => {
                    self.handle_shard_block_commit(block_hash, shard_id).await;
                }
            }
        }

    async fn handle_cross_shard_transaction(
        &mut self,
        transaction: Transaction,
        source_shard_id: u64,
        target_shard_id: u64,
    ) {
        // Validate and process the cross-shard transaction
        // ...

        // Forward the transaction to the target shard
        let message = ShardMessage::CrossShardTransactionMessage {
            transaction,
            source_shard_id,
            target_shard_id,
        };
        if let Err(e) = self.send_message(target_shard_id, message).await {
            error!("Failed to forward cross-shard transaction: {}", e);
        }
    }

    async fn handle_shard_block_proposal(&mut self, block: Block, shard_id: u64) {
        // Validate and process the proposed shard block
        // ...

        // Propagate the block to other nodes in the shard
        let message = ShardMessage::ShardBlockProposalMessage {
            block,
            shard_id,
        };
        if let Err(e) = self.broadcast_message_within_shard(shard_id, message).await {
            error!("Failed to propagate shard block proposal: {}", e);
        }
    }

    async fn handle_shard_block_commit(&mut self, block_hash: String, shard_id: u64) {
        // Validate and commit the shard block
        // ...

        // Propagate the block commit to other nodes in the shard
        let message = ShardMessage::ShardBlockCommitMessage {
            block_hash,
            shard_id,
        };
        if let Err(e) = self.broadcast_message_within_shard(shard_id, message).await {
            error!("Failed to propagate shard block commit: {}", e);
        }
    }

    async fn handle_shard_sync_request(&mut self, shard_id: u64, sync_hash: String) {
        // Retrieve the shard state
        let shard_state = self.get_shard_state(shard_id).await.unwrap();

        // Check if the sync hash matches the current shard state
        if shard_state.hash() == sync_hash {
            info!("Shard {} is already in sync", shard_id);
            return;
        }

        // Serialize the shard state
        let serialized_state = bincode::serialize(&shard_state).unwrap();

        // Send the shard sync response
        let sync_response = ShardMessage::ShardSyncResponse {
            shard_id,
            sync_data: serialized_state,
        };
        if let Err(e) = self.send_message(shard_id, sync_response).await {
            error!("Failed to send shard sync response for shard {}: {}", shard_id, e);
        }
    }

    async fn handle_shard_sync_response(&mut self, shard_id: u64, sync_data: Vec<u8>) {
        // Deserialize the shard state
        let shard_state: ShardState = bincode::deserialize(&sync_data).unwrap();

        // Update the local shard state
        self.update_shard_state(shard_id, shard_state).await.unwrap();

        info!("Shard {} state synced successfully", shard_id);
    }

    async fn handle_transaction_message(&mut self, shard_id: u64, transaction: Transaction) {
        info!("Received transaction message for shard {}", shard_id);
        // Process the transaction and update the shard state
        match self.process_transaction(shard_id, transaction).await {
            Ok(_) => info!("Transaction processed successfully"),
            Err(e) => error!("Failed to process transaction: {}", e),
        }
    }

    async fn handle_state_request(&mut self, shard_id: u64) {
        info!("Received state request for shard {}", shard_id);
        // Retrieve the current shard state and send it back
        match self.get_shard_state(shard_id).await {
            Ok(state) => {
                if let Err(e) = self.send_shard_state(shard_id, state).await {
                    error!("Failed to send shard state: {}", e);
                }
            }
            Err(e) => error!("Failed to retrieve shard state: {}", e),
        }
    }

    async fn handle_state_response(&mut self, shard_id: u64, state: ShardState) {
        info!("Received state response for shard {}", shard_id);
        // Update the local shard state with the received state
        match self.update_shard_state(shard_id, state).await {
            Ok(_) => info!("Shard state updated successfully"),
            Err(e) => error!("Failed to update shard state: {}", e),
        }
    }

    async fn process_transaction(&mut self, shard_id: u64, transaction: Transaction) -> Result<(), NetworkError> {
        // Validate the transaction
        if !transaction.validate() {
            return Err(NetworkError::InvalidTransaction);
        }

        // Retrieve the current shard state
        let mut shard_state = self.get_shard_state(shard_id).await?;

        // Apply the transaction to the shard state
        match shard_state.apply_transaction(transaction) {
            Ok(()) => {
                // Update the shard state
                self.update_shard_state(shard_id, shard_state).await?;
                Ok(())
            }
            Err(e) => Err(NetworkError::TransactionApplicationFailed(e.to_string())),
        }
    }

    async fn get_shard_state(&mut self, shard_id: u64) -> Result<ShardState, NetworkError> {
        // Retrieve the shard state from the state manager
        match self.state_manager.get_shard_state(shard_id).await {
            Ok(state) => Ok(state),
            Err(e) => Err(NetworkError::ShardStateRetrievalFailed(e.to_string())),
        }
    }

    async fn update_shard_state(&mut self, shard_id: u64, state: ShardState) -> Result<(), NetworkError> {
        // Update the shard state in the state manager
        match self.state_manager.update_shard_state(shard_id, state).await {
            Ok(()) => Ok(()),
            Err(e) => Err(NetworkError::ShardStateUpdateFailed(e.to_string())),
        }
    }

    async fn handle_qup_shard_block_proposal(&mut self, shard_id: u64, block: QUPBlock) {
        // Verify the block proposal signature
        if let Err(e) = self.crypto.verify_block_proposal_signature(&block) {
            error!("QUP shard block proposal signature verification failed: {}", e);
            return;
        }

        // Process the block proposal
        if let Err(e) = self.process_qup_shard_block(shard_id, block).await {
            error!("Processing QUP shard block proposal failed: {}", e);
        }
    }

    async fn handle_qup_shard_block_commit(
        &mut self,
        shard_id: u64,
        block_hash: String,
        signature: QUPSignature,
    ) {
        // Verify the block commit signature
        if let Err(e) = self.crypto.verify_block_commit_signature(&block_hash, &signature) {
            error!("QUP shard block commit signature verification failed: {}", e);
            return;
        }

        // Commit the shard block
        if let Err(e) = self.commit_qup_shard_block(shard_id, block_hash).await {
            error!("Committing QUP shard block failed: {}", e);
        }
    }

    async fn process_qup_shard_block(&mut self, shard_id: u64, block: QUPBlock) -> Result<(), NetworkError> {
        // Validate the block
        if let Err(e) = block.validate() {
            return Err(NetworkError::InvalidBlock(e.to_string()));
        }

        // Apply the block to the shard state
        let mut shard_state = self.get_shard_state(shard_id).await?;
        if let Err(e) = shard_state.apply_block(&block) {
            return Err(NetworkError::BlockApplicationFailed(e.to_string()));
        }

        // Update the shard state
        self.update_shard_state(shard_id, shard_state).await?;

        Ok(())
    }

    async fn commit_qup_shard_block(&mut self, shard_id: u64, block_header: QUPBlockHeader) -> Result<(), NetworkError> {
        // Retrieve the shard state
        let mut shard_state = self.get_shard_state(shard_id).await?;

        // Commit the block header to the shard state
        if let Err(e) = shard_state.commit_block_header(block_header) {
            return Err(NetworkError::BlockCommitFailed(e.to_string()));
        }

        // Update the shard state
        self.update_shard_state(shard_id, shard_state).await?;

        Ok(())
    }

    async fn handle_qup_shard_vote(&mut self, shard_id: u64, vote: QUPVote) {
        // Verify the vote signature
        if let Err(e) = self.crypto.verify_vote_signature(&vote) {
            error!("QUP shard vote signature verification failed: {}", e);
            return;
        }

        // Process the vote
        if let Err(e) = self.process_qup_shard_vote(shard_id, vote).await {
            error!("Processing QUP shard vote failed: {}", e);
        }
    }

    async fn handle_qup_shard_useful_work(&mut self, shard_id: u64, useful_work: QUPUsefulWork) {
        // Verify the useful work
        if let Err(e) = self.crypto.verify_useful_work(&useful_work) {
            error!("QUP shard useful work verification failed: {}", e);
            return;
        }

        // Process the useful work
        if let Err(e) = self.process_qup_shard_useful_work(shard_id, useful_work).await {
            error!("Processing QUP shard useful work failed: {}", e);
        }
    }

    async fn handle_qkd_key_request_shard(&mut self, shard_id: u64) {
        // Generate a new QKD key pair for the shard
        let (private_key, public_key) = self.crypto.generate_qkd_key_pair().unwrap();

        // Store the private key for the shard
        self.qkd_private_keys.insert(shard_id, private_key);

        // Send the QKD public key to the shard nodes
        let qkd_key_response = ShardMessage::QKDKeyResponseShard(public_key);
        if let Err(e) = self.send_message(shard_id, qkd_key_response).await {
            error!("Failed to send QKD key response to shard {}: {}", shard_id, e);
        }
    }

    async fn handle_qkd_key_response_shard(&mut self, shard_id: u64, public_key: QKDPublicKey) {
        // Store the QKD public key received from the shard
        self.qkd_public_keys.insert(shard_id, public_key);

        // Send the QKD key confirmation to the shard
        let qkd_key_confirmation = ShardMessage::QKDKeyConfirmationShard;
        if let Err(e) = self.send_message(shard_id, qkd_key_confirmation).await {
            error!("Failed to send QKD key confirmation to shard {}: {}", shard_id, e);
        }
    }

    async fn handle_qkd_key_confirmation_shard(&mut self, shard_id: u64) {
        // QKD key exchange completed for the shard
        info!("QKD key exchange completed for shard {}", shard_id);

        // Perform any additional setup or actions required for secure communication with the shard
        // using the established QKD keys
        // ...
    }

    async fn handle_quantum_state_distribution_shard(&mut self, shard_id: u64, state: QuantumState) {
        // Retrieve the QKD keys for the shard
        let private_key = self.qkd_private_keys.get(&shard_id).unwrap();
        let public_key = self.qkd_public_keys.get(&shard_id).unwrap();

        // Decrypt and deserialize the quantum state using the QKD keys
        let serialized_state = self.crypto.decrypt(&state.data, private_key).unwrap();
        let deserialized_state = bincode::deserialize(&serialized_state).unwrap();

        // Process the quantum state
        if let Err(e) = self.process_quantum_state(shard_id, deserialized_state).await {
            error!("Processing quantum state for shard {} failed: {}", shard_id, e);
        }
    }

    async fn handle_quantum_state_measurement_results_shard(&mut self, shard_id: u64, results: Vec<bool>) {
        // Retrieve the QKD keys for the shard
        let private_key = self.qkd_private_keys.get(&shard_id).unwrap();
        let public_key = self.qkd_public_keys.get(&shard_id).unwrap();

        // Encrypt and serialize the measurement results using the QKD keys
        let serialized_results = bincode::serialize(&results).unwrap();
        let encrypted_results = self.crypto.encrypt(&serialized_results, public_key).unwrap();

        // Send the encrypted measurement results to the shard
        let measurement_results_message = ShardMessage::QuantumStateMeasurementResultsShard(encrypted_results);
        if let Err(e) = self.send_message(shard_id, measurement_results_message).await {
            error!("Failed to send quantum state measurement results to shard {}: {}", shard_id, e);
        }
    }

    async fn process_qup_shard_vote(&mut self, shard_id: u64, vote: QUPVote) -> Result<(), NetworkError> {
        // Verify the vote signature
        if let Err(e) = self.crypto.verify_vote_signature(&vote) {
            return Err(NetworkError::InvalidSignature(format!("Invalid QUP shard vote signature: {}", e)));
        }
    
        // Process the QUP shard vote
        match self.consensus.process_shard_vote(shard_id, vote).await {
            Ok(_) => {
                info!("QUP shard vote processed successfully for shard {}", shard_id);
                Ok(())
            }
            Err(e) => {
                error!("Processing QUP shard vote failed for shard {}: {}", shard_id, e);
                Err(NetworkError::ConsensusError(format!("Failed to process QUP shard vote: {}", e)))
            }
        }
    }
    
    async fn process_qup_shard_useful_work(&mut self, shard_id: u64, useful_work: QUPUsefulWork) -> Result<(), NetworkError> {
        // Verify the useful work
        if let Err(e) = self.crypto.verify_useful_work(&useful_work) {
            return Err(NetworkError::InvalidUsefulWork(format!("Invalid QUP shard useful work: {}", e)));
        }
    
        // Process the QUP shard useful work
        match self.consensus.process_shard_useful_work(shard_id, useful_work).await {
            Ok(_) => {
                info!("QUP shard useful work processed successfully for shard {}", shard_id);
                Ok(())
            }
            Err(e) => {
                error!("Processing QUP shard useful work failed for shard {}: {}", shard_id, e);
                Err(NetworkError::ConsensusError(format!("Failed to process QUP shard useful work: {}", e)))
            }
        }
    }
    
    async fn process_quantum_state(&mut self, shard_id: u64, state: QuantumState) -> Result<(), NetworkError> {
        // Verify the quantum state
        if let Err(e) = self.crypto.verify_quantum_state(&state) {
            return Err(NetworkError::InvalidQuantumState(format!("Invalid quantum state: {}", e)));
        }
    
        // Process the quantum state
        match self.quantum_state_processor.process_state(shard_id, state).await {
            Ok(_) => {
                info!("Quantum state processed successfully for shard {}", shard_id);
                Ok(())
            }
            Err(e) => {
                error!("Processing quantum state failed for shard {}: {}", shard_id, e);
                Err(NetworkError::QuantumStateProcessingFailed(format!("Failed to process quantum state: {}", e)))
            }
        }
    }

}

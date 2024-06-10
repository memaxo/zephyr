use crate::chain::blockchain::Blockchain;
use crate::chain::transaction::Transaction;
use crate::chain::common::{BlockCommon, BlockFields};
use crate::parallel::parallel_map::parallel_map;
use crate::qup::consensus::QUPConsensus;
use crate::qup::crypto::{PostQuantumSignature, QUPCrypto};
use crate::secure_core::secure_vault::SecureVault;
use crate::smart_contract::SmartContract;
use crate::state::State;
use log::{debug, error, info, warn};
use merkle::MerkleTree;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use crate::qup::crypto::{QUPCrypto, Hash};
use thiserror::Error;

const TARGET_BLOCK_TIME: u64 = 600; // Target block time in seconds (10 minutes)
const MAX_MINING_ATTEMPTS: u64 = 1_000_000; // Maximum number of mining attempts before giving up


#[derive(Error, Debug)]
pub enum BlockError {
    #[error("Duplicate transaction detected: {0:?}")]
    DuplicateTransaction(Transaction),
    #[error("Invalid transactions: {0}")]
    InvalidTransactions(String),
    #[error("Failed to construct Merkle tree: {0}")]
    MerkleTreeConstructionError(String),
    #[error("Failed to mine block: {0}")]
    MiningError(String),
    #[error("Invalid smart contract: {0}")]
    InvalidSmartContract(String),
    #[error("Smart contract execution failed: {0}")]
    SmartContractExecutionError(String),
    #[error("State update failed: {0}")]
    StateUpdateError(String),
    #[error("Invalid useful work: {0}")]
    InvalidUsefulWork(String),
    #[error("Post-quantum signature error: {0}")]
    PostQuantumSignatureError(String),
    #[error("Maximum mining attempts reached")]
    MaxMiningAttemptsReached,
}

impl Block {
    fn update_qup_state(&self, qup_state: &mut QUPState) -> Result<(), BlockError> {
        // Implement the logic to update QUP-specific state
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub common: BlockFields,
    pub difficulty: u64,
    pub nonce: u64,
    pub merkle_root: String,
    pub smart_contracts: Vec<Arc<SmartContract>>,
    pub hv_dimension: u64,
    pub state_root: String,
    pub qup_block_header: QUPBlockHeader,
    pub qup_specific_data: QUPSpecificData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsefulWork {
    pub problem_type: String,
    pub solution: Vec<u8>,
}

impl Block {
    fn calculate_merkle_root(&self) -> Result<String, BlockError> {
        let tx_hashes: Vec<_> = parallel_map(&self.transactions, |tx| tx.calculate_hash_with_qup());

        MerkleTree::new(&tx_hashes)
            .map(|tree| tree.root_hash())
            .map_err(|e| BlockError::MerkleTreeConstructionError(e.to_string()))
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.timestamp.to_le_bytes());
        for tx in &self.transactions {
            hasher.update(tx.calculate_hash().as_bytes());
        }
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.difficulty.to_le_bytes());
        hasher.update(self.merkle_root.as_bytes());
        for sc in &self.smart_contracts {
            hasher.update(sc.code.as_bytes());
            hasher.update(sc.state.to_string().as_bytes());
        }
        hasher.update(self.state_root.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify_transactions(&self, secure_vault: &SecureVault) -> Result<(), BlockError> {
        use rayon::ThreadPoolBuilder;
        use std::sync::Mutex;

        let tx_hashes = Arc::new(Mutex::new(HashSet::new()));
        let num_threads = std::cmp::min(self.transactions.len(), num_cpus::get());
        let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

        let verification_results: Vec<_> = pool.install(|| {
            self.transactions
                .par_iter()
                .map(|tx| {
                    let tx_hash = tx.calculate_hash();
                    let mut tx_hashes = tx_hashes.lock().unwrap();
                    if tx_hashes.contains(&tx_hash) {
                        Err(BlockError::DuplicateTransaction(tx.clone()))
                    } else {
                        tx_hashes.insert(tx_hash);
                        drop(tx_hashes); // Release the lock before verifying the signature
                        tx.verify_signature(secure_vault).map_err(|e| {
                            BlockError::InvalidTransactions(format!(
                                "Transaction signature verification failed: {}",
                                e
                            ))
                        })
                    }
                })
                .collect()
        });

        if verification_results.iter().any(|r| r.is_err()) {
            let errors: Vec<String> = verification_results
                .into_iter()
                .filter_map(|r| r.err())
                .map(|e| format!("{}", e))
                .collect();
            Err(BlockError::InvalidTransactions(errors.join("; ")))
        } else {
            Ok(())
        }
    }

    pub fn sign(&mut self, qup_crypto: &QUPCrypto) -> Result<(), BlockError> {
        let hash = self.calculate_hash();
        let signature = qup_crypto.sign_block(&hash).map_err(|e| {
            BlockError::PostQuantumSignatureError(format!("Failed to sign block: {}", e))
        })?;
        self.validator_signature = Some(signature);
        // Verify history proof if present
        if let Some(proof) = &self.qup_block_header.history_proof {
            qup_state.verify_history_proof(proof).map_err(|e| {
                BlockError::HistoryProofVerificationError(format!(
                    "Failed to verify history proof: {}",
                    e
                ))
            })?;
        }
        Ok(())
    }

    pub fn verify_signature(&self, qup_crypto: &QUPCrypto, qup_state: &QUPState) -> Result<(), BlockError> {
        if let Some(signature) = &self.validator_signature {
            let hash = self.calculate_hash();
            qup_crypto.verify_block_signature(&hash, signature).map_err(|e| {
                BlockError::PostQuantumSignatureError(format!(
                    "Failed to verify block signature: {}",
                    e
                ))
            })?;
            
            // Verify history proof if present
            if let Some(proof) = &self.history_proof {
                qup_state.verify_history_proof(proof).map_err(|e| {
                    BlockError::HistoryProofVerificationError(format!(
                        "Failed to verify history proof: {}",
                        e
                    ))
                })?;
            }
        } else {
            Err(BlockError::PostQuantumSignatureError(
                "Block signature not found".to_string(),
            ))
        }
    }

    pub fn validate(&self, qup_consensus: &QUPConsensus, qup_state: &QUPState) -> Result<(), BlockError> {
        self.verify_transactions(qup_consensus.secure_vault())?;
        self.verify_signature(qup_consensus.qup_crypto(), qup_state)?;
        if let Some(useful_work) = &self.useful_work {
            qup_consensus.verify_useful_work(useful_work).map_err(|e| {
                BlockError::InvalidUsefulWork(format!("Failed to verify useful work: {}", e))
            })?;
        }
        Ok(())
    }

    pub fn validate_smart_contracts(&self) -> Result<(), BlockError> {
        for sc in &self.smart_contracts {
            if sc.code.contains("malicious_code") {
                return Err(BlockError::InvalidSmartContract(
                    "Smart contract contains malicious code".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn mine_block(&mut self) -> Result<(), BlockError> {
        let target_hash = "0".repeat(self.difficulty as usize);
        let mut attempts = 0;

        while !self.hash.starts_with(&target_hash) {
            if attempts >= MAX_MINING_ATTEMPTS {
                error!("Maximum mining attempts reached. Giving up.");
                return Err(BlockError::MaxMiningAttemptsReached);
            }

            self.nonce += 1;
            self.hash = self.calculate_hash();
            attempts += 1;
        }

        debug!(
            "Block successfully mined with hash: {} after {} attempts",
            self.hash, attempts
        );
        Ok(())
    }

    pub fn mine(&mut self) -> Result<(), BlockError> {
        self.validate_smart_contracts()?;
        self.mine_block()?;
        Ok(())
    }

    fn execute_smart_contracts(
        &self,
        blockchain: &Blockchain,
        state: &mut State,
    ) -> Result<(), BlockError> {
        let mut rollback_states = Vec::new();

        for sc in &self.smart_contracts {
            let current_state = state.clone();
            rollback_states.push(current_state);

            match sc.execute(blockchain, state) {
                Ok(_) => debug!("Smart contract executed successfully"),
                Err(e) => {
                    error!(
                        "Smart contract execution failed: {}. Rolling back state changes.",
                        e
                    );

                    // Rollback state changes
                    for rollback_state in rollback_states.into_iter().rev() {
                        *state = rollback_state;
                    }

                    return Err(BlockError::SmartContractExecutionError(e.to_string()));
                }
            }
        }

        Ok(())
    }


    fn update_state(&self, state: &mut State) -> Result<(), BlockError> {
        let mut rollback_states = Vec::new();

        for tx in &self.transactions {
            let current_state = state.clone();
            rollback_states.push(current_state);

            match tx.apply_to_state(state) {
                Ok(_) => debug!("Transaction applied to state successfully"),
                Err(e) => {
                    error!(
                        "Transaction failed to apply to state: {}. Rolling back state changes.",
                        e
                    );

                    // Rollback state changes
                    for rollback_state in rollback_states.into_iter().rev() {
                        *state = rollback_state;
                    }

                    return Err(BlockError::StateUpdateError(e.to_string()));
                }
            }
        }

        Ok(())
    }

    pub fn add_to_blockchain(
        &self,
        blockchain: &mut Blockchain,
        secure_vault: &SecureVault,
        state: &mut State,
        qup_state: &QUPState,
    ) -> Result<(), BlockError> {
        self.verify_transactions(secure_vault)?;
        if let Err(e) = blockchain.validate_chain().await {
            return Err(BlockError::InvalidTransactions(format!(
                "Failed to add block to the blockchain. Chain is invalid: {}",
                e
            )));
            self.update_state(state)?;
            self.update_qup_state(qup_state)?;
            self.execute_smart_contracts(blockchain, state)?;
            blockchain.add_block(self.clone());
            info!("Block added to the blockchain");
            Ok(())
        } else {
            Err(BlockError::InvalidTransactions(
                "Failed to add block to the blockchain. Chain is invalid".to_string(),
            ))
        }
    }
}

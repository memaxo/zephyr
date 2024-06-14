use crate::chain::transaction::Transaction;
use crate::qup::config::QUPConfig;
use crate::qup::crypto::{QUPSignature, Hash, QuantumHash, QuantumSignature};
use crate::utils::hashing::calculate_hash;
use crate::chain::common::{BlockCommon, BlockFields};
use crate::storage::block_storage::BlockStorage;
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use crate::qup::classical_node::ClassicalNode;
use crate::qup::quantum_node::QuantumNode;
use crate::storage::block_storage::BlockStorageError;
use crate::qup::types::{UsefulWorkProblem, UsefulWorkSolution};
use crate::qup::error::Error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use crossbeam_utils::thread;
use smallvec::SmallVec;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QUPBlock {
    pub common: BlockFields,
    pub height: u64,
    pub prev_block_hash: Hash,
    pub useful_work_problem: Option<Box<UsefulWorkProblem>>,
    pub useful_work_solution: Option<Box<UsefulWorkSolution>>,
    pub history_proof: Arc<SmallVec<[Hash; 4]>>,
    pub validator_signature: QUPSignature,
}

impl BlockCommon for QUPBlock {
    fn verify_signature(&self, qup_crypto: &QUPCrypto, qup_state: &QUPState) -> Result<(), BlockError> {
        if let Some(signature) = &self.common.validator_signature {
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
            Ok(())
        } else {
            Err(BlockError::PostQuantumSignatureError(
                "Block signature not found".to_string(),
            ))
        }
    }

    fn validate(&self, qup_consensus: &QUPConsensus, qup_state: &QUPState) -> Result<(), BlockError> {
        // Validate transactions
        for tx in &self.common.transactions {
            tx.validate(qup_state)?;
        }

        // Validate block height
        if self.height != qup_state.get_block_height() + 1 {
            return Err(BlockError::InvalidBlockHeight);
        }

        // Validate previous block hash
        if self.prev_block_hash != qup_state.get_block_hash() {
            return Err(BlockError::InvalidPrevBlockHash);
        }

        // Validate useful work problem and solution
        if let Some(problem) = &self.useful_work_problem {
            if !qup_consensus.is_valid_useful_work_problem(problem) {
                return Err(BlockError::InvalidUsefulWorkProblem);
            }
        }
        if let Some(solution) = &self.useful_work_solution {
            if !qup_consensus.is_valid_useful_work_solution(solution) {
                return Err(BlockError::InvalidUsefulWorkSolution);
            }
        }

        // Validate history proof
        if let Some(proof) = &self.history_proof {
            if !qup_state.verify_history_proof(proof)? {
                return Err(BlockError::InvalidHistoryProof);
            }
        }

        // Additional consensus-specific validations
        if !qup_consensus.is_valid_block(self, qup_state) {
            return Err(BlockError::InvalidBlock);
        }

        Ok(())
    }

    fn apply(&self, state: &mut QUPState) -> Result<(), Error> {
        self.update_state(state)?;

        // Apply transactions
        for tx in &self.common.transactions {
            tx.apply(state)?;
        }

        // Distribute rewards
        self.distribute_rewards(state)?;

        Ok(())
    }
}

impl QUPBlock {
    pub fn new(
        height: u64,
        timestamp: u64,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        useful_work_problem: Option<UsefulWorkProblem>,
        useful_work_solution: Option<UsefulWorkSolution>,
        history_proof: Vec<Hash>,
        validator: &QUPValidator,
    ) -> Self {
        let mut block = QUPBlock {
            height,
            timestamp,
            prev_block_hash,
            transactions,
            useful_work_problem: useful_work_problem.map(Box::new),
            useful_work_solution: useful_work_solution.map(Box::new),
            history_proof: Arc::new(SmallVec::from(history_proof)),
            validator_signature: QUPSignature::default(),
        };
        block.sign(validator);
        block
    }

    pub fn save(&self, storage: &BlockStorage) -> Result<(), BlockStorageError> {
        storage.save_block(self)
    }

    pub fn load(hash: &Hash, storage: &BlockStorage) -> Result<Self, BlockStorageError> {
        storage.load_block(hash)
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = hash::Hasher::new();
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.prev_block_hash);

        for tx in &self.transactions {
            hasher.update(&tx.hash());
        }

        if let Some(problem) = &self.useful_work_problem {
            hasher.update(&serde_json::to_vec(problem).unwrap());
        }

        if let Some(solution) = &self.useful_work_solution {
            hasher.update(&serde_json::to_vec(solution).unwrap());
        }

        for proof in &self.history_proof {
            hasher.update(proof);
        }

        hasher.finalize()
    }

    pub fn sign(&mut self, validator: &QUPValidator) {
        let block_hash = self.hash();
        self.validator_signature = validator.sign(&block_hash);
    }

    pub fn verify_signature(&self, validator: &QUPValidator) -> bool {
        let block_hash = self.hash();
        validator.verify(&block_hash, &self.validator_signature)
    }

    fn update_state(&self, state: &mut QUPState) -> Result<(), Error> {
        state.set_block_height(self.height)?;
        state.set_block_timestamp(self.timestamp)?;
        state.set_block_hash(self.hash())?;
        Ok(())
    }

    fn distribute_rewards(&self, state: &mut QUPState) -> Result<(), Error> {
        let config = state.get_config();
        let validator_reward = config.block_reward * config.validator_reward_ratio;
        let delegator_reward = config.block_reward - validator_reward;

        state.add_balance(&self.validator_signature.signer, validator_reward)?;

        // Distribute delegator_reward among delegators based on their stake
        let total_stake = state.get_total_stake();
        for (delegator, stake) in state.get_delegators() {
            let reward = delegator_reward * stake / total_stake;
            state.add_balance(&delegator, reward)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModelBlock {
    pub id: String,
    pub model_parameters: Vec<f64>,
    pub previous_hash: QuantumHash,
    pub hash: QuantumHash,
    pub signature: QuantumSignature,
}

impl ModelBlock {
    pub fn new(
        id: String,
        model_parameters: Vec<f64>,
        previous_hash: QuantumHash,
        signature: QuantumSignature,
    ) -> Self {
        let mut block = ModelBlock {
            id,
            model_parameters,
            previous_hash,
            hash: QuantumHash::default(),
            signature,
        };
        block.calculate_hash();
        block
    }

    pub fn calculate_hash(&mut self) {
        let mut hasher = hash::Hasher::new();
        hasher.update(&self.id.as_bytes());
        hasher.update(&serde_json::to_vec(&self.model_parameters).unwrap());
        hasher.update(&self.previous_hash);
        self.hash = QuantumHash::from_bytes(&hasher.finalize());
    }

    pub fn verify_signature(&self, validator: &QUPValidator) -> bool {
        validator.verify(&self.hash, &self.signature)
    }

    pub fn validate(&self, previous_block: &ModelBlock) -> Result<(), Error> {
        // Validate the previous hash
        if self.previous_hash != previous_block.hash {
            return Err(Error::InvalidPrevBlockHash);
        }

        // Additional validations for model blocks
        // ...

        Ok(())
    }

    pub fn apply(&self, state: &mut QUPState) -> Result<(), Error> {
        // Update the state with the model parameters
        state.set_model_parameters(&self.model_parameters)?;

        // Additional state updates for model blocks
        // ...

        Ok(())
    }
}
use crate::chain::transaction::Transaction;
use crate::qup::config::QUPConfig;
use crate::qup::crypto::{hash, QUPSignature};
use crate::qup::state::QUPState;
use crate::qup::validator::QUPValidator;
use crate::qup::types::{UsefulWorkProblem, UsefulWorkSolution};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QUPBlock {
    pub height: u64,
    pub timestamp: u64,
    pub prev_block_hash: Hash,
    pub transactions: Vec<Transaction>,
    pub useful_work_problem: Option<UsefulWorkProblem>,
    pub useful_work_solution: Option<UsefulWorkSolution>,
    pub history_proof: Vec<Hash>,
    pub validator_signature: QUPSignature,
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
            useful_work_problem,
            useful_work_solution,
            history_proof,
            validator_signature: QUPSignature::default(),
        };
        block.sign(validator);
        block
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
            hasher.update(&bincode::serialize(problem).unwrap());
        }

        if let Some(solution) = &self.useful_work_solution {
            hasher.update(&bincode::serialize(solution).unwrap());
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

    pub fn validate(&self, state: &QUPState, config: &QUPConfig) -> Result<(), Error> {
        // Check the block height
        if self.height != state.get_block_height() + 1 {
            return Err(Error::InvalidBlockHeight);
        }

        // Check the block timestamp
        if self.timestamp < state.get_block_timestamp() {
            return Err(Error::InvalidBlockTimestamp);
        }

        // Check the previous block hash
        if self.prev_block_hash != state.get_block_hash() {
            return Err(Error::InvalidPrevBlockHash);
        }

        // Validate the transactions
        for tx in &self.transactions {
            tx.validate(state)?;
        }

        // Ensure the block follows the QUP consensus rules
        if !state.is_valid_block(self, config) {
            return Err(Error::InvalidBlock);
        }

        Ok(())
    }

    pub fn apply(&self, state: &mut QUPState) -> Result<(), Error> {
        // Update the state with the block height, timestamp, and hash
        state.set_block_height(self.height);
        state.set_block_timestamp(self.timestamp);
        state.set_block_hash(self.hash());

        // Apply the transactions to the state
        for tx in &self.transactions {
            tx.apply(state)?;
        }

        // Distribute rewards to the validator and delegators
        let validator_reward = config.block_reward * config.validator_reward_ratio;
        let delegator_reward = config.block_reward - validator_reward;

        state.add_balance(&self.validator_signature.signer, validator_reward)?;
        // Distribute delegator_reward among delegators based on their stake

        Ok(())
    }
}

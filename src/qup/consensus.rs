use crate::chain::transaction::Transaction;
use crate::consensus::ConsensusMessage;
use crate::error::ConsensusError;
use crate::hdcmodels::HDCModel;
use crate::network::NetworkMessage;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::crypto::{verify_signature, QUPKeyPair};
use crate::qup::state::QUPState;
use std::sync::Arc;

pub struct QUPConsensus {
    pub config: Arc<QUPConfig>,
    pub state: Arc<QUPState>,
    pub key_pair: QUPKeyPair,
    pub hdc_model: HDCModel,
}

impl QUPConsensus {
    pub fn new(
        config: Arc<QUPConfig>,
        state: Arc<QUPState>,
        key_pair: QUPKeyPair,
        hdc_model: HDCModel,
    ) -> Self {
        QUPConsensus {
            config,
            state,
            key_pair,
            hdc_model,
        }
    }

    pub fn process_message(&mut self, message: ConsensusMessage) -> Result<(), ConsensusError> {
        match message {
            ConsensusMessage::Propose(block) => self.process_propose(block),
            ConsensusMessage::Vote(vote) => self.process_vote(vote),
            ConsensusMessage::Commit(block_hash) => self.process_commit(block_hash),
            ConsensusMessage::Vote(vote) => self.process_vote(vote),
            ConsensusMessage::Commit(block_hash) => self.process_commit(block_hash),
        }
    }

    fn process_propose(&mut self, block: QUPBlock) -> Result<(), ConsensusError> {
        // Validate the block
        if !self.validate_block(&block)? {
            return Err(ConsensusError::InvalidBlock);
        }

        // Evaluate the block using the HDC model
        let block_vector = self.hdc_model.encode_block(&block);
        let similarity = self.hdc_model.evaluate_similarity(&block_vector);

        // Check if the block meets the similarity threshold
        if similarity < self.config.similarity_threshold {
            return Err(ConsensusError::InsufficientSimilarity);
        }

        // Broadcast the block to other validators
        let message = NetworkMessage::BlockProposal(block.clone());
        self.config.network.broadcast(message)?;

        // Add the block to the local pool of proposed blocks
        self.state.add_proposed_block(block)?;

        Ok(())
    }

    pub fn process_vote(&mut self, vote: QUPVote) -> Result<(), ConsensusError> {
        // Verify the vote signature
        if !verify_vote_signature(&vote) {
            return Err(ConsensusError::InvalidSignature);
        }

        // Add the vote to the state
        self.state.add_vote(vote.clone())?;

        // Check if the block has reached quorum
        if self.state.has_quorum(&vote.block_hash)? {
            let block = self.state.get_proposed_block(&vote.block_hash)?;
            self.commit_block(block)?;
        }

        Ok(())
    }

    pub fn process_commit(&mut self, block_hash: Hash) -> Result<(), ConsensusError> {
        // Retrieve the block from the local pool of proposed blocks
        let block = self.state.get_proposed_block(&block_hash)?;

        // Apply the block to the state
        self.state.apply_block(&block)?;

        // Optimize the block using the HDC model
        let optimized_block = self.hdc_model.optimize_block(&block);

        // Broadcast the optimized block to other nodes
        let message = NetworkMessage::BlockCommit(optimized_block);
        self.config.network.broadcast(message)?;

        Ok(())
    }

    pub fn cast_vote(&self, block_hash: Hash) -> Result<QUPVote, ConsensusError> {
        let vote = QUPVote {
            voter: self.key_pair.public_key.to_bytes().to_vec(),
            block_hash,
            signature: self.key_pair.sign(&block_hash.to_bytes()),
        };

        // Broadcast the vote to other validators
        let message = NetworkMessage::Vote(vote.clone());
        self.config.network.broadcast(message)?;

        Ok(vote)
    }

    pub fn has_quorum(&self, block_hash: &Hash) -> Result<bool, ConsensusError> {
        let votes = self.state.get_votes(block_hash)?;
        let total_stake: u64 = self.state.get_total_stake();
        let quorum_stake: u64 = (total_stake as f64 * self.config.consensus_config.quorum_threshold) as u64;

        let mut accumulated_stake: u64 = 0;
        for vote in votes {
            let voter_stake = self.state.get_validator_stake(&vote.voter)?;
            accumulated_stake += voter_stake;
            if accumulated_stake >= quorum_stake {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn propose_block(
        &self,
        transactions: Vec<Transaction>,
    ) -> Result<QUPBlock, ConsensusError> {
        // Generate useful work problem
        let useful_work_problem = self.generate_useful_work_problem();

        // Solve useful work problem
        let useful_work_solution = self.solve_useful_work_problem(&useful_work_problem);

        // Generate history proof
        let history_proof = self.generate_history_proof();

        // Create a new block with the given transactions, useful work problem, solution, and history proof
        let mut block = QUPBlock::new(
            self.state.get_block_height()? + 1,
            self.state.get_block_timestamp()?,
            self.state.get_block_hash()?,
            transactions,
            Some(useful_work_problem),
            Some(useful_work_solution),
            history_proof,
            &self.key_pair,
        );

        // Sign the block using the validator's private key
        block.sign(&self.key_pair);

        // Broadcast the block proposal to other validators
        let message = NetworkMessage::BlockProposal(block.clone());
        self.config.network.broadcast(message)?;

        Ok(block)
    }

    fn generate_useful_work_problem(&self) -> UsefulWorkProblem {
        // Generate a useful work problem
        // This can be customized based on the specific requirements of the useful work problem
        UsefulWorkProblem::Knapsack(KnapsackProblem {
            capacity: 50,
            weights: vec![10, 20, 30, 40],
            values: vec![60, 100, 120, 160],
        })
    }

    fn solve_useful_work_problem(&self, problem: &UsefulWorkProblem) -> UsefulWorkSolution {
        // Solve the useful work problem
        // This can be customized based on the specific requirements of the useful work problem
        match problem {
            UsefulWorkProblem::Knapsack(knapsack_problem) => {
                UsefulWorkSolution::Knapsack(KnapsackSolution {
                    selected_items: vec![true, false, true, false],
                })
            }
            UsefulWorkProblem::VertexCover(vertex_cover_problem) => {
                UsefulWorkSolution::VertexCover(VertexCoverSolution {
                    vertex_cover: vec![0, 2],
                })
            }
        }
    }

    fn generate_history_proof(&self) -> Vec<Hash> {
        // Generate a history proof
        // This can be customized based on the specific requirements of the history proof
        vec![self.state.get_block_hash()?]
    }

    pub fn validate_block(&self, block: &QUPBlock) -> Result<bool, ConsensusError> {
        // Verify the block signature
        let signer = block.proposer;
        let signature = block
            .signature
            .as_ref()
            .ok_or(ConsensusError::MissingSignature)?;
        let block_data = block.hash().as_bytes();
        if !verify_signature(&signer, signature, block_data)? {
            return Ok(false);
        }

        // Check if the block follows the QUP consensus rules
        if !self.state.is_valid_block(block)? {
            return Ok(false);
        }

        // Validate useful work solution
        if let Some(problem) = &block.useful_work_problem {
            if let Some(solution) = &block.useful_work_solution {
                if !self.validate_useful_work_solution(problem, solution)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // Validate history proof
        if !self.validate_history_proof(&block.history_proof)? {
            return Ok(false);
        }

        Ok(true)
    }

    fn validate_useful_work_solution(
        &self,
        problem: &UsefulWorkProblem,
        solution: &UsefulWorkSolution,
    ) -> Result<bool, ConsensusError> {
        // Implement the logic to validate the useful work solution
        match problem {
            UsefulWorkProblem::Knapsack(knapsack_problem) => {
                // Validate the knapsack solution
                let total_weight: u64 = solution
                    .as_knapsack()
                    .selected_items
                    .iter()
                    .enumerate()
                    .filter(|(_, &selected)| selected)
                    .map(|(i, _)| knapsack_problem.weights[i])
                    .sum();
                if total_weight > knapsack_problem.capacity {
                    return Ok(false);
                }
                Ok(true)
            }
            UsefulWorkProblem::VertexCover(vertex_cover_problem) => {
                // Validate the vertex cover solution
                let vertex_cover = solution.as_vertex_cover().vertex_cover.clone();
                if !is_valid_vertex_cover(&vertex_cover_problem.graph, &vertex_cover) {
                    return Ok(false);
                }
                Ok(true)
            }
        }
    }

    fn validate_history_proof(&self, history_proof: &[Hash]) -> Result<bool, ConsensusError> {
        // Implement the logic to validate the history proof
        // For example, check if the history proof contains valid hashes of previous blocks
        for hash in history_proof {
            if !self.state.is_valid_block_hash(hash)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn commit_block(&mut self, block: QUPBlock) -> Result<(), ConsensusError> {
        // Apply the block to the state
        self.state.apply_block(&block)?;

        // Distribute rewards to validators and delegators
        self.distribute_rewards(&block)?;

        // Optimize the block using the HDC model
        let optimized_block = self.hdc_model.optimize_block(&block);

        // Broadcast the optimized block to other nodes
        let message = NetworkMessage::BlockCommit(optimized_block);
        self.config.network.broadcast(message)?;

        Ok(())
    }

    fn distribute_rewards(&mut self, block: &QUPBlock) -> Result<(), ConsensusError> {
        // Calculate the rewards for validators and delegators based on the block
        let rewards = self.config.reward_scheme.calculate_rewards(block)?;

        // Distribute the rewards to validators and delegators
        for (address, reward) in rewards {
            self.state.add_balance(&address, reward)?;
        }

        Ok(())
    }
}

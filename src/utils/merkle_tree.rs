use crate::crypto::hash::{hash_data, Hash};
use crate::qup::types::QUPTransaction;
use blake3::Hasher;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

pub struct MerkleTree {
    root: Hash,
    leaves: Vec<Hash>,
}

impl MerkleTree {
    pub fn new(transactions: &[QUPTransaction]) -> Result<Self, MerkleTreeError> {
        let leaves: Vec<Hash> = transactions
            .par_iter()
            .map(|tx| hash_transaction(tx))
            .collect();

        if leaves.is_empty() {
            return Err(MerkleTreeError::EmptyLeaves);
        }

        let root = Self::build_merkle_tree(&leaves);

        Ok(Self { root, leaves })
    }

    fn build_merkle_tree(leaves: &[Hash]) -> Hash {
        let mut current_level = leaves.to_vec();

        while current_level.len() > 1 {
            current_level = current_level
                .par_chunks(2)
                .map(|chunk| {
                    if chunk.len() == 2 {
                        hash_pair(&chunk[0], &chunk[1])
                    } else {
                        chunk[0].clone()
                    }
                })
                .collect();
        }

        current_level[0].clone()
    }

    pub fn verify_transaction(&self, transaction: &QUPTransaction, proof: &[Hash]) -> bool {
        let leaf = hash_transaction(transaction);
        self.verify_leaf(&leaf, proof)
    }

    pub fn verify_leaf(&self, leaf: &Hash, proof: &[Hash]) -> bool {
        let mut computed_hash = leaf.clone();

        for sibling_hash in proof {
            computed_hash = if computed_hash <= *sibling_hash {
                hash_pair(&computed_hash, sibling_hash)
            } else {
                hash_pair(sibling_hash, &computed_hash)
            };
        }

        computed_hash == self.root
    }

    pub fn generate_proof(&self, transaction: &QUPTransaction) -> Option<Vec<Hash>> {
        let leaf = hash_transaction(transaction);
        let leaf_index = self.leaves.iter().position(|l| *l == leaf)?;
        let proof = Self::generate_proof_for_index(leaf_index, &self.leaves);
        Some(proof)
    }

    fn generate_proof_for_index(index: usize, leaves: &[Hash]) -> Vec<Hash> {
        let mut proof = Vec::new();
        let mut current_index = index;
        let mut current_level = leaves.to_vec();

        while current_level.len() > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < current_level.len() {
                proof.push(current_level[sibling_index].clone());
            }

            current_index /= 2;
            current_level = current_level
                .chunks(2)
                .map(|chunk| hash_pair(&chunk[0], &chunk[1]))
                .collect();
        }

        proof
    }

    pub fn get_root(&self) -> &Hash {
        &self.root
    }
}

fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = Hasher::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().as_bytes().to_vec()
}

pub fn hash_transaction(transaction: &QUPTransaction) -> Hash {
    hash_data(&transaction.serialize())
}

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("Leaves cannot be empty")]
    EmptyLeaves,
}

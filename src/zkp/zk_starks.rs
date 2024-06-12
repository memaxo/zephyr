use crate::zkp_crate::math::{FieldElement, PolynomialCommitment};
use crate::chain::blockchain::blockchain_utils::MerkleTree;

pub struct ZkStarksProof {
    pub commitments: Vec<PolynomialCommitment>,
    pub evaluations: Vec<FieldElement>,
    pub fri_proof: FriProof,
}

pub struct FriProof {
    pub commit_phase_merkle_roots: Vec<FieldElement>,
    pub query_round_proofs: Vec<FriQueryRoundProof>,
    pub final_poly_merkle_root: FieldElement,
}

pub struct FriQueryRoundProof {
    pub merkle_proof: Vec<FieldElement>,
    pub polynomial_values: Vec<FieldElement>,
}

pub struct AggregatedFriProof {
    pub aggregated_commitments: Vec<PolynomialCommitment>,
    pub aggregated_evaluations: Vec<FieldElement>,
    pub aggregated_fri_proof: FriProof,
}

impl AggregatedFriProof {
    pub fn new(proofs: Vec<ZkStarksProof>) -> Self {
        let mut aggregated_commitments = Vec::new();
        let mut aggregated_evaluations = Vec::new();
        let mut aggregated_fri_proof = FriProof {
            commit_phase_merkle_roots: Vec::new(),
            query_round_proofs: Vec::new(),
            final_poly_merkle_root: FieldElement::zero(&BigUint::from(1u32) << 256),
        };

        for proof in proofs {
            aggregated_commitments.extend(proof.commitments);
            aggregated_evaluations.extend(proof.evaluations);
            aggregated_fri_proof.commit_phase_merkle_roots.extend(proof.fri_proof.commit_phase_merkle_roots);
            aggregated_fri_proof.query_round_proofs.extend(proof.fri_proof.query_round_proofs);
        }

        AggregatedFriProof {
            aggregated_commitments,
            aggregated_evaluations,
            aggregated_fri_proof,
        }
    }
}
    pub security_level: usize,
    pub num_queries: usize,
    pub field_size: FieldElement,
    pub generator: FieldElement,
}

impl ZkStarksVerifier {
    pub fn new(security_level: usize, num_queries: usize, field_size: FieldElement) -> Self {
        let generator = FieldElement::generator(field_size);
        ZkStarksVerifier {
            security_level,
            num_queries,
            field_size,
            generator,
        }
    }

    pub fn verify(&self, proof: &ZkStarksProof) -> bool {
        let domain_size = 1 << self.security_level;
        let domain = FieldElement::get_domain(domain_size, self.field_size);

        // Verify polynomial commitments
        for commitment in &proof.commitments {
            if !commitment.verify(&self.generator) {
                return false;
            }
        }

        // Verify FRI proof
        let mut merkle_roots = proof.fri_proof.commit_phase_merkle_roots.clone();
        let mut current_poly_values = proof.evaluations.clone();
        let mut current_domain = domain.clone();
        let mut current_generator = self.generator;

        for query_round_proof in &proof.fri_proof.query_round_proofs {
            let merkle_root = merkle_roots.last().unwrap().clone();
            let mut hasher = Hasher::new();
            hasher.update(&merkle_root);
            let rand_seed = hasher.finalize();

            let mut query_indices = Vec::new();
            for _ in 0..self.num_queries {
                let index = FieldElement::from_bytes(&rand_seed) % current_domain.len();
                query_indices.push(index.to_usize());
            }

            let mut query_values = Vec::new();
            for &index in &query_indices {
                query_values.push(current_poly_values[index]);
            }

            let mut current_merkle_proof = query_round_proof.merkle_proof.clone();
            current_merkle_proof.reverse();
            let mut current_hash = Hasher::new();
            for &value in &query_values {
                current_hash.update(&value);
            }
            let mut current_leaf_hash = current_hash.finalize();

            for proof_node in &current_merkle_proof {
                let mut hasher = Hasher::new();
                hasher.update(&current_leaf_hash);
                hasher.update(proof_node);
                current_leaf_hash = hasher.finalize();
            }

            if current_leaf_hash != merkle_root {
                return false;
            }

            current_poly_values = query_round_proof.polynomial_values.clone();
            current_domain = current_domain.iter().step_by(2).cloned().collect();
            current_generator = current_generator.pow(&FieldElement::from(2));
        }

        let final_merkle_root = MerkleTree::new(&current_poly_values).root();
        final_merkle_root == proof.fri_proof.final_poly_merkle_root
    }
}

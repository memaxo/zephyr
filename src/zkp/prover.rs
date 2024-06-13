use rayon::prelude::*;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use crate::zkp::math::{FieldElement, PolynomialCommitment};
use crate::zkp::constraint_system::{ConstraintSystem, ConstraintSystemImpl, PlonkConstraint};
use crate::zkp::transcript::Transcript;

pub struct Prover {
    pub constraint_system: ConstraintSystemImpl,
}

impl Prover {
    pub fn new(constraint_system: ConstraintSystemImpl) -> Self {
        Prover { constraint_system }
    }

    pub fn create_proof(&self) -> Proof {
        let mut transcript = Transcript::new();

        // Parallelize the evaluation of constraints
        let evaluations: Vec<_> = self.constraint_system.plonk_constraints.par_iter()
            .map(|constraint| {
                let lhs_value = self.constraint_system.evaluate(&constraint.lhs);
                let rhs_value = self.constraint_system.evaluate(&constraint.rhs);
                (lhs_value, rhs_value)
            })
            .collect();

        // Collect the commitments and evaluations
        let commitments: Vec<_> = evaluations.par_iter()
            .map(|(lhs_value, rhs_value)| {
                let commitment = PolynomialCommitment::new(vec![lhs_value.clone(), rhs_value.clone()], &self.constraint_system.settings).unwrap();
                (commitment, lhs_value.clone(), rhs_value.clone())
            })
            .collect();

        // Generate range proof for confidential value
        let confidential_value = FieldElement::from(42); // Replace with your confidential value
        let (range_proof, commitment) = RangeProof::prove_single(
            &BulletproofGens::new(64, 1), // Adjust parameters as needed
            &PedersenGens::default(),
            &mut transcript,
            confidential_value.clone(), 
            &mut rand::thread_rng(),
        ).expect("Range proof creation failed");
        
        // Enforce range proof in constraint system
        let value_var = self.constraint_system.alloc_variable(confidential_value);
        self.constraint_system.enforce_range_proof(value_var, range_proof, &PedersenGens::default(), &BulletproofGens::new(64, 1));

        // Create the proof
        Proof {
            commitments: commitments.iter().map(|(commitment, _, _)| commitment.clone()).collect(),
            evaluations: commitments.iter().map(|(_, lhs_value, rhs_value)| (lhs_value.clone(), rhs_value.clone())).collect(),
            range_proof_commitment: commitment,
            transcript,
        }
    }
}

pub struct Proof {
    pub commitments: Vec<PolynomialCommitment>,
    pub evaluations: Vec<(FieldElement, FieldElement)>,
    pub range_proof_commitment: RangeProof,
    pub transcript: Transcript,
}

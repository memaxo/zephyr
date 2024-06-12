use rayon::prelude::*;
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

        // Create the proof
        Proof {
            commitments: commitments.iter().map(|(commitment, _, _)| commitment.clone()).collect(),
            evaluations: commitments.iter().map(|(_, lhs_value, rhs_value)| (lhs_value.clone(), rhs_value.clone())).collect(),
            transcript,
        }
    }
}

pub struct Proof {
    pub commitments: Vec<PolynomialCommitment>,
    pub evaluations: Vec<(FieldElement, FieldElement)>,
    pub transcript: Transcript,
}

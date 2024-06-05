use crate::hdcmodels::optimization_problems::csp::{Constraint, Variable, CSP};
use crate::hdcmodels::optimization_problems::csp_encoder::CSPEncoder;
use crate::hdcmodels::optimization_problems::csp_solver::CSPSolver;
use crate::qup::crypto::{QUPKeyPair, QUPSignature};
use std::collections::HashMap;

pub struct CSPQUPIntegration {
    solver: CSPSolver,
    key_pair: QUPKeyPair,
}

impl CSPQUPIntegration {
    pub fn new(solver: CSPSolver, key_pair: QUPKeyPair) -> Self {
        CSPQUPIntegration { solver, key_pair }
    }

    pub fn solve<V: Variable, C: Constraint<V>>(&self, csp: &CSP<V, C>) -> Option<CSPSolution> {
        if let Some(solution) = self.solver.solve(csp) {
            let signature = self.sign_solution(&solution);
            Some(CSPSolution {
                assignment: solution,
                signature,
            })
        } else {
            None
        }
    }

    pub fn verify_solution<V: Variable, C: Constraint<V>>(
        &self,
        csp: &CSP<V, C>,
        solution: &CSPSolution,
    ) -> bool {
        if !self.verify_signature(&solution.assignment, &solution.signature) {
            return false;
        }

        let encoded_csp = self.solver.encoder.encode(csp);
        let encoded_solution = self.encode_solution(&solution.assignment);

        self.solver
            .hdc_model
            .verify(&encoded_csp, &encoded_solution)
    }

    fn sign_solution(&self, solution: &HashMap<usize, usize>) -> QUPSignature {
        let encoded_solution = self.encode_solution(solution);
        self.key_pair.sign(&encoded_solution)
    }

    fn verify_signature(&self, solution: &HashMap<usize, usize>, signature: &QUPSignature) -> bool {
        let encoded_solution = self.encode_solution(solution);
        self.key_pair
            .public_key
            .verify(&encoded_solution, signature)
    }

    fn encode_solution(&self, solution: &HashMap<usize, usize>) -> Vec<f64> {
        let mut encoded_solution = vec![0.0; self.solver.hdc_model.dimension];

        for (&index, &value) in solution {
            encoded_solution[index * solution.len() + value] = 1.0;
        }

        encoded_solution
    }
}

pub struct CSPSolution {
    assignment: HashMap<usize, usize>,
    signature: QUPSignature,
}

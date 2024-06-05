use crate::hdcmodels::optimization_problems::knapsack::{
    KnapsackEncoder, KnapsackProblem, KnapsackSolution, KnapsackSolver,
};
use crate::qup::crypto::{QUPKeyPair, QUPSignature};

pub struct KnapsackQUPIntegration {
    solver: KnapsackSolver,
    key_pair: QUPKeyPair,
}

impl KnapsackQUPIntegration {
    pub fn new(solver: KnapsackSolver, key_pair: QUPKeyPair) -> Self {
        KnapsackQUPIntegration { solver, key_pair }
    }

    pub fn solve(&self, problem: &KnapsackProblem) -> KnapsackSolution {
        let solution = self.solver.solve(problem);
        let signature = self.sign_solution(&solution);
        solution.set_signature(signature);
        solution
    }

    pub fn verify_solution(&self, problem: &KnapsackProblem, solution: &KnapsackSolution) -> bool {
        if !solution.verify_signature(&self.key_pair.public_key) {
            return false;
        }

        let encoded_problem = self.solver.encoder.encode(problem);
        let encoded_solution = self.encode_solution(solution);

        self.solver
            .hdc_model
            .verify(&encoded_problem, &encoded_solution)
    }

    fn sign_solution(&self, solution: &KnapsackSolution) -> QUPSignature {
        let encoded_solution = self.encode_solution(solution);
        self.key_pair.sign(&encoded_solution)
    }

    fn encode_solution(&self, solution: &KnapsackSolution) -> Vec<f64> {
        let mut encoded_solution = Vec::new();

        // Encode the selected items
        for &item_index in &solution.selected_items {
            encoded_solution.push(item_index as f64);
        }

        // Encode the total value
        encoded_solution.push(solution.total_value as f64);

        encoded_solution
    }
}

impl KnapsackSolution {
    fn set_signature(&mut self, signature: QUPSignature) {
        self.signature = Some(signature);
    }

    fn verify_signature(&self, public_key: &QUPPublicKey) -> bool {
        if let Some(signature) = &self.signature {
            let encoded_solution = self.encode();
            public_key.verify(&encoded_solution, signature)
        } else {
            false
        }
    }

    fn encode(&self) -> Vec<f64> {
        let mut encoded_solution = Vec::new();

        // Encode the selected items
        for &item_index in &self.selected_items {
            encoded_solution.push(item_index as f64);
        }

        // Encode the total value
        encoded_solution.push(self.total_value as f64);

        encoded_solution
    }
}

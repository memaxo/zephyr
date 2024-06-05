use crate::hdcmodels::optimization_problems::vertex_cover::{
    VertexCoverEncoder, VertexCoverProblem, VertexCoverSolution, VertexCoverSolver,
};
use crate::qup::crypto::{QUPKeyPair, QUPSignature};

pub struct VertexCoverQUPIntegration {
    solver: VertexCoverSolver,
    key_pair: QUPKeyPair,
}

impl VertexCoverQUPIntegration {
    pub fn new(solver: VertexCoverSolver, key_pair: QUPKeyPair) -> Self {
        VertexCoverQUPIntegration { solver, key_pair }
    }

    pub fn solve(&self, problem: &VertexCoverProblem) -> VertexCoverSolution {
        let solution = self.solver.solve(problem);
        let signature = self.sign_solution(&solution);
        solution.set_signature(signature);
        solution
    }

    pub fn verify_solution(
        &self,
        problem: &VertexCoverProblem,
        solution: &VertexCoverSolution,
    ) -> bool {
        if !solution.verify_signature(&self.key_pair.public_key) {
            return false;
        }

        let encoded_problem = self.solver.encoder.encode(problem);
        let encoded_solution = self.encode_solution(solution);

        self.solver
            .hdc_model
            .verify(&encoded_problem, &encoded_solution)
    }

    fn sign_solution(&self, solution: &VertexCoverSolution) -> QUPSignature {
        let encoded_solution = self.encode_solution(solution);
        self.key_pair.sign(&encoded_solution)
    }

    fn encode_solution(&self, solution: &VertexCoverSolution) -> Vec<f64> {
        let mut encoded_solution = vec![0.0; self.solver.hdc_model.dimension];

        // Set the selected vertices to 1.0 in the encoded solution
        for &vertex in &solution.vertex_cover {
            encoded_solution[vertex] = 1.0;
        }

        encoded_solution
    }
}

impl VertexCoverSolution {
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
        let mut encoded_solution = vec![0.0; self.vertex_cover.len()];

        // Set the selected vertices to 1.0 in the encoded solution
        for &vertex in &self.vertex_cover {
            encoded_solution[vertex] = 1.0;
        }

        encoded_solution
    }
}

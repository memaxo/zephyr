use crate::hdcmodels::optimization_problems::knapsack::{
    KnapsackEncoder, KnapsackProblem, KnapsackSolution,
};
use crate::hdcmodels::HDCModel;

pub struct KnapsackSolver {
    hdc_model: HDCModel,
    encoder: KnapsackEncoder,
}

impl KnapsackSolver {
    pub fn new(hdc_model: HDCModel, dimension: usize) -> Self {
        let encoder = KnapsackEncoder::new(dimension);
        KnapsackSolver { hdc_model, encoder }
    }

    pub fn solve(&self, problem: &KnapsackProblem) -> KnapsackSolution {
        let encoded_problem = self.encoder.encode(problem);
        let output = self.hdc_model.predict(&encoded_problem);

        self.decode_solution(&output, problem)
    }

    fn decode_solution(&self, output: &[f64], problem: &KnapsackProblem) -> KnapsackSolution {
        let mut selected_items = Vec::new();
        let mut total_value = 0;
        let mut remaining_capacity = problem.capacity;

        // Interpret the HDC model's output as item selections
        for (i, &value) in output.iter().enumerate() {
            if value > 0.5 && remaining_capacity >= problem.weights[i] {
                selected_items.push(i);
                total_value += problem.values[i];
                remaining_capacity -= problem.weights[i];
            }
        }

        KnapsackSolution {
            selected_items,
            total_value,
        }
    }
}

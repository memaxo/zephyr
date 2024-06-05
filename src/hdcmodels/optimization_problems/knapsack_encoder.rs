use crate::hdcmodels::encoding::{encode_natural_language, encode_transactional_data};
use crate::hdcmodels::optimization_problems::knapsack::KnapsackProblem;

pub struct KnapsackEncoder {
    dimension: usize,
}

impl KnapsackEncoder {
    pub fn new(dimension: usize) -> Self {
        KnapsackEncoder { dimension }
    }

    pub fn encode(&self, problem: &KnapsackProblem) -> Vec<f64> {
        let mut encoded_problem = Vec::new();

        // Encode the problem description
        let description = format!(
            "Knapsack problem with capacity {} and {} items",
            problem.capacity,
            problem.weights.len()
        );
        let encoded_description = encode_natural_language(&description, self.dimension);
        encoded_problem.extend(encoded_description);

        // Encode the item weights and values
        let item_data: Vec<(u64, u64)> = problem
            .weights
            .iter()
            .zip(problem.values.iter())
            .map(|(&weight, &value)| (weight, value))
            .collect();
        let encoded_items = encode_transactional_data(&item_data, self.dimension);
        encoded_problem.extend(encoded_items);

        encoded_problem
    }
}

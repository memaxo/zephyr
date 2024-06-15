use crate::qup::types::{UsefulWorkProblem, UsefulWorkSolution};
use crate::qup::error::CryptoError;
use std::collections::HashSet;

pub fn verify_useful_work(problem: &UsefulWorkProblem, solution: &UsefulWorkSolution) -> Result<bool, CryptoError> {
    // Verify the useful work solution based on the problem
    match problem {
        UsefulWorkProblem::Knapsack(problem) => {
            // Verify the knapsack solution
            let total_weight: u64 = problem.weights.iter().zip(&solution.selected_items).map(|(w, &s)| if s { *w } else { 0 }).sum();
            let total_value: u64 = problem.values.iter().zip(&solution.selected_items).map(|(v, &s)| if s { *v } else { 0 }).sum();
            Ok(total_weight <= problem.capacity && total_value == solution.total_value)
        }
        UsefulWorkProblem::VertexCover(problem) => {
            // Verify the vertex cover solution
            let mut covered_edges = HashSet::new();
            for &vertex in &solution.vertex_cover {
                for &neighbor in &problem.graph[vertex] {
                    covered_edges.insert((vertex, neighbor));
                    covered_edges.insert((neighbor, vertex));
                }
            }
            Ok(covered_edges.len() == problem.graph.iter().map(|neighbors| neighbors.len()).sum::<usize>())
        }
        UsefulWorkProblem::ScientificSimulation(problem) => {
            // Verify the scientific simulation result
            // Placeholder: Implement the actual verification logic
            Ok(true)
        }
        UsefulWorkProblem::Cryptanalysis(problem) => {
            // Verify the cryptanalysis result
            // Placeholder: Implement the actual verification logic
            Ok(true)
        }
    }
}

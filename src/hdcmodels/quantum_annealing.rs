use crate::hdcmodels::HDCModel;
use crate::optimization_problems::OptimizationProblem;
use ndarray::Array1;
use rand::Rng;
use std::f64::consts::E;

pub struct QuantumAnnealer {
    hdc_model: HDCModel,
    temperature: f64,
    annealing_steps: usize,
}

impl QuantumAnnealer {
    pub fn new(hdc_model: HDCModel, temperature: f64, annealing_steps: usize) -> Self {
        QuantumAnnealer {
            hdc_model,
            temperature,
            annealing_steps,
        }
    }

    pub fn anneal<P: OptimizationProblem>(&self, problem: &P) -> Vec<f64> {
        let initial_solution = self.hdc_model.generate_random_solution(problem.dimension());
        let mut current_solution = initial_solution;
        let mut best_solution = current_solution.clone();
        let mut best_energy = problem.evaluate_energy(&best_solution);

        for step in 0..self.annealing_steps {
            let neighbor_solution = self.generate_neighbor_solution(&current_solution);
            let current_energy = problem.evaluate_energy(&current_solution);
            let neighbor_energy = problem.evaluate_energy(&neighbor_solution);

            if neighbor_energy < current_energy {
                current_solution = neighbor_solution;
                if neighbor_energy < best_energy {
                    best_solution = current_solution.clone();
                    best_energy = neighbor_energy;
                }
            } else {
                let acceptance_probability =
                    ((current_energy - neighbor_energy) / self.temperature).exp();
                if rand::thread_rng().gen_bool(acceptance_probability) {
                    current_solution = neighbor_solution;
                }
            }

            self.temperature *= 1.0 - (step as f64 / self.annealing_steps as f64).powf(2.0);
        }

        best_solution
    }

    fn generate_neighbor_solution(&self, solution: &[f64]) -> Vec<f64> {
        let mut neighbor_solution = solution.to_vec();
        let index = rand::thread_rng().gen_range(0..solution.len());
        let perturbation = rand::thread_rng().gen_range(-1.0..1.0);
        neighbor_solution[index] += perturbation;
        neighbor_solution
    }
}

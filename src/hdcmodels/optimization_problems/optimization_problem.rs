use crate::chain::block::Block;
use crate::hdcmodels::optimization_problems::problem_solution::ProblemSolution;

pub trait OptimizationProblem {
    fn solve(&self, block: &Block) -> Result<ProblemSolution, String>;
    fn validate(&self, block: &Block, solution: &ProblemSolution) -> Result<bool, String>;
}

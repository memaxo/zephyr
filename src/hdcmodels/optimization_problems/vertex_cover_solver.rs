use crate::hdcmodels::optimization_problems::vertex_cover::{
    VertexCoverEncoder, VertexCoverProblem, VertexCoverSolution,
};
use crate::hdcmodels::HDCModel;

pub struct VertexCoverSolver {
    hdc_model: HDCModel,
    encoder: VertexCoverEncoder,
}

impl VertexCoverSolver {
    pub fn new(hdc_model: HDCModel, dimension: usize) -> Self {
        let encoder = VertexCoverEncoder::new(dimension);
        VertexCoverSolver { hdc_model, encoder }
    }

    pub fn solve(&self, problem: &VertexCoverProblem) -> VertexCoverSolution {
        let encoded_problem = self.encoder.encode(problem);
        let output = self.hdc_model.predict(&encoded_problem);

        self.decode_solution(&output, problem)
    }

    fn decode_solution(&self, output: &[f64], problem: &VertexCoverProblem) -> VertexCoverSolution {
        let mut vertex_cover = Vec::new();
        let mut remaining_graph = problem.graph.clone();

        // Interpret the HDC model's output as vertex selections
        for (vertex, &value) in output.iter().enumerate() {
            if value > 0.5 {
                vertex_cover.push(vertex);
                self.remove_covered_edges(&mut remaining_graph, vertex);
            }
        }

        // Ensure all edges are covered
        while !self.is_all_edges_covered(&remaining_graph) {
            let (vertex, _) = self.find_max_degree_vertex(&remaining_graph);
            vertex_cover.push(vertex);
            self.remove_covered_edges(&mut remaining_graph, vertex);
        }

        VertexCoverSolution { vertex_cover }
    }

    fn is_all_edges_covered(&self, graph: &[Vec<usize>]) -> bool {
        graph.iter().all(|edges| edges.is_empty())
    }

    fn find_max_degree_vertex(&self, graph: &[Vec<usize>]) -> (usize, usize) {
        graph
            .iter()
            .enumerate()
            .max_by_key(|(_, edges)| edges.len())
            .map(|(vertex, edges)| (vertex, edges.len()))
            .unwrap_or((0, 0))
    }

    fn remove_covered_edges(&self, graph: &mut [Vec<usize>], vertex: usize) {
        for neighbor in &graph[vertex] {
            graph[*neighbor].retain(|&v| v != vertex);
        }
        graph[vertex].clear();
    }
}

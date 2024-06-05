use crate::hdcmodels::encoding::{encode_natural_language, encode_transactional_data};
use crate::hdcmodels::optimization_problems::vertex_cover::VertexCoverProblem;

pub struct VertexCoverEncoder {
    dimension: usize,
}

impl VertexCoverEncoder {
    pub fn new(dimension: usize) -> Self {
        VertexCoverEncoder { dimension }
    }

    pub fn encode(&self, problem: &VertexCoverProblem) -> Vec<f64> {
        let mut encoded_problem = Vec::new();

        // Encode the problem description
        let description = format!(
            "Vertex Cover problem with {} vertices and {} edges",
            problem.graph.len(),
            problem.graph.iter().map(|edges| edges.len()).sum::<usize>() / 2
        );
        let encoded_description = encode_natural_language(&description, self.dimension);
        encoded_problem.extend(encoded_description);

        // Encode the graph structure
        let mut edge_data = Vec::new();
        for (u, edges) in problem.graph.iter().enumerate() {
            for &v in edges {
                edge_data.push((u as u64, v as u64));
            }
        }
        let encoded_edges = encode_transactional_data(&edge_data, self.dimension);
        encoded_problem.extend(encoded_edges);

        encoded_problem
    }
}

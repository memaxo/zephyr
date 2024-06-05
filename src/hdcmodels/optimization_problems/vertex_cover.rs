use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexCoverProblem {
    pub graph: Vec<Vec<usize>>,
}

impl VertexCoverProblem {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        VertexCoverProblem { graph }
    }

    pub fn solve(&self) -> VertexCoverSolution {
        let num_vertices = self.graph.len();
        let mut vertex_cover = Vec::new();

        // Create a copy of the graph to track uncovered edges
        let mut remaining_graph = self.graph.clone();

        // Iterate until all edges are covered
        while !self.is_all_edges_covered(&remaining_graph) {
            // Find the vertex with the highest degree (most uncovered edges)
            let (max_degree_vertex, _) = self.find_max_degree_vertex(&remaining_graph);

            // Add the vertex to the vertex cover
            vertex_cover.push(max_degree_vertex);

            // Remove the covered edges from the remaining graph
            self.remove_covered_edges(&mut remaining_graph, max_degree_vertex);
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexCoverSolution {
    pub vertex_cover: Vec<usize>,
}

impl VertexCoverSolution {
    pub fn new(vertex_cover: Vec<usize>) -> Self {
        VertexCoverSolution { vertex_cover }
    }
}

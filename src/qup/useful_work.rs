pub struct KnapsackProblem {
    pub capacity: u64,
    pub weights: Vec<u64>,
    pub values: Vec<u64>,
}

pub struct KnapsackSolution {
    pub selected_items: Vec<bool>,
}

pub struct VertexCoverProblem {
    pub graph: Vec<Vec<usize>>,
}

pub struct VertexCoverSolution {
    pub vertex_cover: Vec<usize>,
}

pub enum UsefulWorkProblem {
    Knapsack(KnapsackProblem),
    VertexCover(VertexCoverProblem),
}

pub enum UsefulWorkSolution {
    Knapsack(KnapsackSolution),
    VertexCover(VertexCoverSolution),
}

impl UsefulWorkSolution {
    pub fn as_knapsack(&self) -> &KnapsackSolution {
        if let UsefulWorkSolution::Knapsack(solution) = self {
            solution
        } else {
            panic!("Not a Knapsack solution");
        }
    }

    pub fn as_vertex_cover(&self) -> &VertexCoverSolution {
        if let UsefulWorkSolution::VertexCover(solution) = self {
            solution
        } else {
            panic!("Not a Vertex Cover solution");
        }
    }
}

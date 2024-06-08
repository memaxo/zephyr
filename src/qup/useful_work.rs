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

pub struct TravelingSalesmanProblem {
    pub cities: Vec<(f64, f64)>,
}

pub struct TravelingSalesmanSolution {
    pub tour: Vec<usize>,
}

pub struct JobSchedulingProblem {
    pub jobs: Vec<(u64, u64)>,
}

pub struct JobSchedulingSolution {
    pub schedule: Vec<usize>,
}

pub struct BinPackingProblem {
    pub capacity: u64,
    pub items: Vec<u64>,
}

pub struct BinPackingSolution {
    pub bins: Vec<Vec<usize>>,
}

pub enum UsefulWorkProblem {
    Knapsack(KnapsackProblem),
    VertexCover(VertexCoverProblem),
    TravelingSalesman(TravelingSalesmanProblem),
    JobScheduling(JobSchedulingProblem),
    BinPacking(BinPackingProblem),
}

pub enum UsefulWorkSolution {
    Knapsack(KnapsackSolution),
    VertexCover(VertexCoverSolution),
    TravelingSalesman(TravelingSalesmanSolution),
    JobScheduling(JobSchedulingSolution),
    BinPacking(BinPackingSolution),
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

    pub fn as_traveling_salesman(&self) -> &TravelingSalesmanSolution {
        if let UsefulWorkSolution::TravelingSalesman(solution) = self {
            solution
        } else {
            panic!("Not a Traveling Salesman solution");
        }
    }

    pub fn as_job_scheduling(&self) -> &JobSchedulingSolution {
        if let UsefulWorkSolution::JobScheduling(solution) = self {
            solution
        } else {
            panic!("Not a Job Scheduling solution");
        }
    }

    pub fn as_bin_packing(&self) -> &BinPackingSolution {
        if let UsefulWorkSolution::BinPacking(solution) = self {
            solution
        } else {
            panic!("Not a Bin Packing solution");
        }
    }
}
use crate::crypto::hash::hash;
use crate::crypto::merkle_tree::MerkleTree;
use crate::crypto::signature::Signature;

pub struct UsefulWorkProof {
    pub problem: UsefulWorkProblem,
    pub solution: UsefulWorkSolution,
    pub signature: Signature,
}

impl UsefulWorkProof {
    pub fn generate(
        problem: UsefulWorkProblem, 
        solution: UsefulWorkSolution,
        private_key: &[u8; 32],
    ) -> Self {
        let problem_hash = hash(&bincode::serialize(&problem).unwrap());
        let solution_hash = hash(&bincode::serialize(&solution).unwrap());
        let data_to_sign = [problem_hash.as_ref(), solution_hash.as_ref()].concat();
        let signature = Signature::sign(&data_to_sign, private_key);

        UsefulWorkProof {
            problem,
            solution,
            signature,
        }
    }

    pub fn verify(&self, public_key: &[u8; 32]) -> bool {
        let problem_hash = hash(&bincode::serialize(&self.problem).unwrap());
        let solution_hash = hash(&bincode::serialize(&self.solution).unwrap());
        let data_to_verify = [problem_hash.as_ref(), solution_hash.as_ref()].concat();
        
        self.signature.verify(&data_to_verify, public_key)
    }
}

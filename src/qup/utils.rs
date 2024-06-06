use crate::crypto::hash::{Hash, Hasher};
use crate::crypto::signature::Signature;
use crate::qup::types::{
    QUPBlockHeader, QUPTransaction, QUPVote, UsefulWorkProblem, UsefulWorkSolution,
};
use rand::Rng;

pub fn calculate_block_hash(block_header: &QUPBlockHeader) -> Hash {
    let mut hasher = Hasher::new();
    hasher.update(&block_header.version.to_le_bytes());
    hasher.update(&block_header.prev_block_hash);
    hasher.update(&block_header.merkle_root);
    hasher.update(&block_header.timestamp.to_le_bytes());
    hasher.update(&block_header.difficulty.to_le_bytes());
    hasher.update(&block_header.nonce.to_le_bytes());
    hasher.finalize()
}

pub fn calculate_transaction_hash(transaction: &QUPTransaction) -> Hash {
    let mut hasher = Hasher::new();
    hasher.update(&transaction.from);
    hasher.update(&transaction.to);
    hasher.update(&transaction.amount.to_le_bytes());
    hasher.update(&transaction.signature);
    hasher.finalize()
}

pub fn verify_transaction_signature(transaction: &QUPTransaction) -> bool {
    let message = calculate_transaction_hash(transaction);
    let public_key = get_public_key_from_address(&transaction.from);
    Signature::verify(&message, &transaction.signature, &public_key)
}

pub fn verify_block_signature(
    block_header: &QUPBlockHeader,
    signature: &Signature,
    public_key: &[u8],
) -> bool {
    let message = calculate_block_hash(block_header);
    Signature::verify(&message, signature, public_key)
}

pub fn verify_vote_signature(vote: &QUPVote) -> bool {
    let message = vote.block_hash;
    let public_key = get_public_key_from_address(&vote.voter);
    Signature::verify(&message, &vote.signature, &public_key)
}

pub fn generate_random_useful_work_problem() -> UsefulWorkProblem {
    let problem_type = rand::thread_rng().gen_range(0..2);
    match problem_type {
        0 => UsefulWorkProblem::Knapsack(generate_random_knapsack_problem()),
        1 => UsefulWorkProblem::VertexCover(generate_random_vertex_cover_problem()),
        _ => panic!("Invalid useful work problem type"),
    }
}

pub fn generate_random_knapsack_problem() -> KnapsackProblem {
    let mut rng = rand::thread_rng();
    let capacity = rng.gen_range(50..100);
    let num_items = rng.gen_range(5..20);
    let weights = (0..num_items).map(|_| rng.gen_range(1..50)).collect();
    let values = (0..num_items).map(|_| rng.gen_range(1..100)).collect();

    KnapsackProblem {
        capacity,
        weights,
        values,
    }
}

pub fn generate_random_vertex_cover_problem() -> VertexCoverProblem {
    let mut rng = rand::thread_rng();
    let num_vertices = rng.gen_range(5..20);
    let mut graph = vec![vec![]; num_vertices];

    for i in 0..num_vertices {
        for j in (i + 1)..num_vertices {
            if rng.gen_bool(0.5) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    VertexCoverProblem { graph }
}

pub fn solve_useful_work_problem(problem: &UsefulWorkProblem) -> UsefulWorkSolution {
    match problem {
        UsefulWorkProblem::Knapsack(knapsack_problem) => {
            UsefulWorkSolution::Knapsack(solve_knapsack_problem(knapsack_problem))
        }
        UsefulWorkProblem::VertexCover(vertex_cover_problem) => {
            UsefulWorkSolution::VertexCover(solve_vertex_cover_problem(vertex_cover_problem))
        }
    }
}

pub fn solve_knapsack_problem(problem: &KnapsackProblem) -> KnapsackSolution {
    let n = problem.weights.len();
    let capacity = problem.capacity as usize;
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            if problem.weights[i - 1] as usize <= w {
                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - problem.weights[i - 1] as usize] + problem.values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    let mut selected_items = vec![false; n];
    let mut w = capacity;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i - 1][w] {
            selected_items[i - 1] = true;
            w -= problem.weights[i - 1] as usize;
        }
    }

    KnapsackSolution { selected_items }
}

pub fn solve_vertex_cover_problem(problem: &VertexCoverProblem) -> VertexCoverSolution {
    let mut vertex_cover = Vec::new();
    let mut visited = vec![false; problem.graph.len()];

    for u in 0..problem.graph.len() {
        if !visited[u] {
            for &v in &problem.graph[u] {
                if !visited[v] {
                    visited[u] = true;
                    visited[v] = true;
                    vertex_cover.push(u);
                    vertex_cover.push(v);
                    break;
                }
            }
        }
    }

    VertexCoverSolution { vertex_cover }
}

fn get_public_key_from_address(address: &[u8]) -> Vec<u8> {
    // Assuming the address is derived from the public key using a hash function
    // Here, we will reverse the process to get the public key from the address
    // This is a placeholder implementation and should be replaced with the actual logic

    // For example, if the address is the first 20 bytes of the SHA-256 hash of the public key:
    use sha2::{Sha256, Digest};

    // Placeholder public key (this should be replaced with the actual logic to retrieve the public key)
    let public_key = vec![0u8; 33]; // Assuming a 33-byte compressed public key

    // Verify that the address matches the derived address from the public key
    let derived_address = &Sha256::digest(&public_key)[..20];
    if derived_address == address {
        public_key
    } else {
        vec![] // Return an empty vector if the address does not match
    }
}

// Add more utility functions as needed

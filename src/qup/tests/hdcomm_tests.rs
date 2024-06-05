use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::optimization_problems::knapsack::{KnapsackProblem, KnapsackSolution};
use crate::optimization_problems::vertex_cover::{VertexCoverProblem, VertexCoverSolution};
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::hdcomm::{HDCommunication, UsefulWork, UsefulWorkProblem, UsefulWorkSolution};
use crate::qup::state::QUPState;
use crate::qup::transaction::Transaction;
use std::sync::Arc;

#[test]
fn test_encode_decode_block() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let hdcomm = HDCommunication::new(config);

    let block = QUPBlock {
        height: 1,
        timestamp: 1623456789,
        prev_block_hash: [1u8; 32].into(),
        transactions: vec![
            Transaction {
                sender: [2u8; 20].into(),
                recipient: [3u8; 20].into(),
                amount: 1000,
                fee: 10,
                data: vec![],
            },
            Transaction {
                sender: [4u8; 20].into(),
                recipient: [5u8; 20].into(),
                amount: 2000,
                fee: 20,
                data: vec![],
            },
        ],
        useful_work: Some(UsefulWork {
            problem: UsefulWorkProblem::Knapsack(KnapsackProblem {
                capacity: 10,
                weights: vec![3, 4, 5, 6],
                values: vec![4, 5, 6, 7],
            }),
            solution: UsefulWorkSolution::Knapsack(KnapsackSolution {
                selected_items: vec![1, 3],
            }),
        }),
        ..Default::default()
    };

    let encoded_block = hdcomm.encode_block(&block);
    let decoded_block = hdcomm.decode_block(&encoded_block);

    assert_eq!(block, decoded_block);
}

#[test]
fn test_similarity() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let hdcomm = HDCommunication::new(config);

    let block1 = QUPBlock {
        height: 1,
        timestamp: 1623456789,
        prev_block_hash: [1u8; 32].into(),
        transactions: vec![Transaction {
            sender: [2u8; 20].into(),
            recipient: [3u8; 20].into(),
            amount: 1000,
            fee: 10,
            data: vec![],
        }],
        ..Default::default()
    };

    let block2 = QUPBlock {
        height: 2,
        timestamp: 1623456790,
        prev_block_hash: block1.hash(),
        transactions: vec![Transaction {
            sender: [4u8; 20].into(),
            recipient: [5u8; 20].into(),
            amount: 2000,
            fee: 20,
            data: vec![],
        }],
        ..Default::default()
    };

    let similarity = hdcomm.similarity(&block1, &block2);
    assert!(similarity > 0.5);
}

#[test]
fn test_useful_work() {
    let config = Arc::new(QUPConfig::default());
    let state = Arc::new(QUPState::new(config.clone()));
    let hdcomm = HDCommunication::new(config);

    // Generate useful work
    let useful_work = hdcomm.generate_useful_work(&state);

    // Verify useful work
    assert!(hdcomm.verify_useful_work(&state, &useful_work));

    // Apply useful work
    let mut new_state = state.clone();
    hdcomm.apply_useful_work(&mut new_state, &useful_work);

    // Verify state update
    match &useful_work.problem {
        UsefulWorkProblem::Knapsack(problem) => {
            let total_weight: u64 = useful_work
                .solution
                .as_knapsack()
                .selected_items
                .iter()
                .map(|&i| problem.weights[i])
                .sum();
            let total_value: u64 = useful_work
                .solution
                .as_knapsack()
                .selected_items
                .iter()
                .map(|&i| problem.values[i])
                .sum();

            assert_eq!(new_state.get_knapsack_optimal_value(), total_value);
        }
        UsefulWorkProblem::VertexCover(problem) => {
            let vertex_cover = useful_work.solution.as_vertex_cover().vertex_cover.clone();
            assert!(is_valid_vertex_cover(&problem.graph, &vertex_cover));
        }
    }
}

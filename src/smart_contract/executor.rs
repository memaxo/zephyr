use crate::smart_contract::contract::{Operation, Subtask, SubtaskStatus};
use crate::smart_contract::types::{Contract, ContractState};
use crate::utils::error::Result;

impl Operation {
    pub fn create_subtask(problem_id: u64, subtask_data: String) -> Self {
        Operation::CreateSubtask(problem_id, subtask_data)
    }

    pub fn assign_subtask(subtask_id: u64, node_address: String) -> Self {
        Operation::AssignSubtask(subtask_id, node_address)
    }

    pub fn submit_subtask_solution(subtask_id: u64, solution: String) -> Self {
        Operation::SubmitSubtaskSolution(subtask_id, solution)
    }
}

pub fn execute_operation(
    &mut self,
    operation: &Operation,
    contract: &mut Contract,
    state: &mut ContractState,
) -> Result<()> {
    match operation {
        // ... (Existing operation handling) ...
        Operation::CreateSubtask(problem_id, subtask_data) => {
            let subtask = Subtask::new(*problem_id, subtask_data.clone());
            contract.subtasks.push(subtask);
            // ... (Update state, emit events) ...
        }
        Operation::AssignSubtask(subtask_id, node_address) => {
            if let Some(subtask) = contract.subtasks.iter_mut().find(|s| s.id == *subtask_id) {
                subtask.node_address = Some(node_address.clone());
                subtask.status = SubtaskStatus::Assigned;
                // ... (Update state, emit events) ...
            }
        }
        Operation::SubmitSubtaskSolution(subtask_id, solution) => {
            if let Some(subtask) = contract.subtasks.iter_mut().find(|s| s.id == *subtask_id) {
                // ... (Verify solution, update subtask status) ...
                subtask.solution = Some(solution.clone());
                subtask.status = SubtaskStatus::Completed;
                // ... (Trigger dependent subtasks if any) ...
                // ... (Distribute rewards) ...
            }
        }
        // ... (Other operations) ...
    }
    Ok(())
}

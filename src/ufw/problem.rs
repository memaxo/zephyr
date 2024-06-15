use std::collections::HashMap;
use uuid::Uuid;
pub struct Problem {
    pub id: Uuid,
    pub domain: String,
    pub difficulty: u8,
    pub data: ProblemData,
    pub subtasks: Option<Vec<Subtask>>,
    pub dependency_graph: Option<HashMap<Uuid, Vec<Uuid>>>, // Map of subtask ID to its dependencies
}

pub struct Subtask {
    pub id: Uuid,
    pub data: ProblemData, // Data specific to this subtask
    pub dependencies: Vec<Uuid>, // IDs of subtasks this subtask depends on
}

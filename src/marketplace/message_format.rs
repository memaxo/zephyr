use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskAssignmentNotification {
    pub task_id: u64,
    pub assigned_node_id: String,
    pub details: String,
}

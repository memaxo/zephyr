use std::collections::HashMap;
use crate::types::NodeId;

pub struct ShardCommittee {
    pub members: Vec<NodeId>,
    pub shard_assignments: HashMap<u64, NodeId>,
}

impl ShardCommittee {
    pub fn new(members: Vec<NodeId>) -> Self {
        ShardCommittee {
            members,
            shard_assignments: HashMap::new(),
        }
    }

    pub fn assign_shard(&mut self, shard_id: u64, node_id: NodeId) {
        self.shard_assignments.insert(shard_id, node_id);
    }

    pub fn get_shard_assignee(&self, shard_id: &u64) -> Option<&NodeId> {
        self.shard_assignments.get(shard_id)
    }
}

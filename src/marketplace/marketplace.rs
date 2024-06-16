use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use std::collections::HashMap;

pub struct Marketplace {
    tasks: HashMap<u64, Task>,
    bids: HashMap<u64, Vec<Bid>>,
    pub fn assign_task(&mut self, task_id: u64) -> Result<String, String> {
        if let Some(bids) = self.bids.get(&task_id) {
            if bids.is_empty() {
                return Err("No bids available for this task".to_string());
            }

            // For simplicity, we assign the task to the bid with the highest proposed reward
            let best_bid = bids.iter().max_by_key(|bid| bid.proposed_reward).unwrap();
            Ok(best_bid.node_id.clone())
        } else {
            Err("Task not found".to_string())
        }
    }
}

impl Marketplace {
    pub fn new() -> Self {
        Marketplace {
            tasks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), String> {
        task.validate()?;
        self.tasks.insert(task.id, task);
        Ok(())
    }

    pub fn get_task(&self, task_id: u64) -> Option<&Task> {
        self.tasks.get(&task_id)
    }

    pub fn add_bid(&mut self, task_id: u64, bid: Bid) -> Result<(), String> {
        if let Some(task) = self.tasks.get(&task_id) {
            if bid.proposed_time > task.deadline {
                return Err("Bid proposed time is past the task deadline".to_string());
            }
            self.bids.entry(task_id).or_insert_with(Vec::new).push(bid);
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn get_bids(&self, task_id: u64) -> Option<&Vec<Bid>> {
        self.bids.get(&task_id)
    }
}

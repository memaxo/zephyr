use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use std::collections::HashMap;

pub struct Marketplace {
    tasks: HashMap<u64, Task>,
    bids: HashMap<u64, Vec<Bid>>,
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

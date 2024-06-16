use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use std::collections::HashMap;
use crate::smart_contract::SmartContract;

pub struct Marketplace {
    tasks: HashMap<u64, Task>,
    bids: HashMap<u64, Vec<Bid>>,
    fn get_reputation_score(&self, node_id: &str) -> f64 {
        // Placeholder for actual reputation score retrieval logic
        // For now, return a dummy value
        1.0
    }

    fn get_capability_score(&self, proof_of_capability: &str) -> f64 {
        // Placeholder for actual capability score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_time_score(&self, proposed_time: &DateTime<Utc>, deadline: &DateTime<Utc>) -> f64 {
        // Calculate time score based on how close the proposed time is to the deadline
        let duration = *deadline - *proposed_time;
        let total_duration = *deadline - Utc::now();
        (total_duration.num_seconds() - duration.num_seconds()) as f64 / total_duration.num_seconds() as f64
    }

    fn break_tie(&self, bid1: &Bid, bid2: &Bid) -> bool {
        // Placeholder for tie-breaking logic
        // For now, use random selection
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_bool(0.5)
    }

impl Marketplace {
    pub fn assign_task(&mut self, task_id: u64) -> Result<(), String> {
        if let Some(bids) = self.bids.get(&task_id) {
            if let Some(best_bid) = self.select_best_bid(bids) {
                let task = self.tasks.get(&task_id).ok_or("Task not found")?;
                SmartContract::assign_task(task, &best_bid)?;
                self.bids.remove(&task_id); // Remove bids after assignment
                Ok(())
            } else {
                Err("No valid bids found".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }
    }

    fn select_best_bid(&self, bids: &[Bid]) -> Option<Bid> {
        // Calculate a score for each bid based on a weighted combination of factors
        let mut best_bid: Option<Bid> = None;
        let mut highest_score = f64::MIN;

        for bid in bids {
            let reputation_score = self.get_reputation_score(&bid.node_id);
            let capability_score = self.get_capability_score(&bid.proof_of_capability);
            let time_score = self.get_time_score(&bid.proposed_time, &self.tasks[&bid.task_id].deadline);

            // Adjust weights as needed
            let score = 0.5 * reputation_score + 0.3 * capability_score + 0.2 * time_score;

            if score > highest_score {
                highest_score = score;
                best_bid = Some(bid.clone());
            } else if (score - highest_score).abs() < f64::EPSILON {
                // Tie-breaking mechanism
                if self.break_tie(&bid, &best_bid.as_ref().unwrap()) {
                    best_bid = Some(bid.clone());
                }
            }
        }

        best_bid
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
        if self.tasks.contains_key(&task.id) {
            return Err("Task ID already exists".to_string());
        }
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
            if bid.proposed_reward > task.reward {
                return Err("Bid proposed reward exceeds task reward".to_string());
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

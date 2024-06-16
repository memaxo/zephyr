use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use crate::smart_contract::types::{SmartContract, Task as SCTask, Bid as SCBid, Reputation as SCReputation};
use crate::chain::blockchain::Blockchain;
use crate::qup::QUP;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Marketplace {
    tasks: Mutex<HashMap<u64, Task>>,
    bids: Mutex<HashMap<u64, Vec<Bid>>>,
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
    pub fn assign_task(&self, task_id: u64, blockchain: &Blockchain, qup: &QUP) -> Result<(), String> {
        let bids = self.bids.lock().unwrap();
        if let Some(bids) = bids.get(&task_id) {
            let task = self.tasks.lock().unwrap().get(&task_id).ok_or("Task not found")?.clone();
            if let Some(best_bid) = self.select_best_bid(&task, bids) {
                let task = self.tasks.get(&task_id).ok_or("Task not found")?;
                let sc_task = SCTask {
                    id: task.id,
                    description: task.description.clone(),
                    resources: task.resources.clone(),
                    reward: task.reward,
                    deadline: task.deadline,
                    creator: task.creator.clone(),
                };
                let sc_bid = SCBid {
                    node_id: best_bid.node_id.clone(),
                    proposed_time: best_bid.proposed_time,
                    proposed_reward: best_bid.proposed_reward,
                    proof_of_capability: best_bid.proof_of_capability.clone(),
                };
                SmartContract::assign_task(&sc_task, &sc_bid)?;

                // Record the task assignment on the blockchain
                blockchain.record_task_assignment(task.id, &best_bid.node_id)?;

                // Update reputation
                let mut reputation = SCReputation::new();
                reputation.update_reputation(&best_bid.node_id, 1.0);
                self.bids.remove(&task_id); // Remove bids after assignment
                qup.send_task_assignment_notification(&task.creator, &best_bid.node_id)?;
                Ok(())
            } else {
                Err("No valid bids found".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }
    }

    fn select_best_bid(&self, task: &Task, bids: &[Bid]) -> Option<Bid> {
        // Calculate a score for each bid based on a weighted combination of factors
        let mut best_bid: Option<Bid> = None;
        let mut highest_score = f64::MIN;

        let default_weights = HashMap::from([
            ("reputation".to_string(), 0.4),
            ("capability".to_string(), 0.3),
            ("time".to_string(), 0.2),
            ("reward".to_string(), 0.1),
        ]);

        let weights = task.weights.as_ref().unwrap_or(&default_weights);

        for bid in bids {
            let reputation_score = self.get_reputation_score(&bid.node_id);
            let capability_score = self.get_capability_score(&bid.proof_of_capability);
            let time_score = self.get_time_score(&bid.proposed_time, &task.deadline);
            let reward_score = 1.0 / bid.proposed_reward as f64; // Lower reward is better

            let score = weights["reputation"] * reputation_score
                + weights["capability"] * capability_score
                + weights["time"] * time_score
                + weights["reward"] * reward_score;

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

    pub fn add_task(&self, task: Task, blockchain: &Blockchain) -> Result<(), String> {
        task.validate()?;
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(&task.id) {
            return Err("Task ID already exists".to_string());
        }
        tasks.insert(task.id, task);
        blockchain.record_task_submission(&task)?;
        Ok(())
    }

    pub fn get_task(&self, task_id: u64) -> Option<&Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(&task_id)
    }

    pub fn add_bid(&self, task_id: u64, bid: Bid, blockchain: &Blockchain) -> Result<(), String> {
        let tasks = self.tasks.lock().unwrap();
        let mut bids = self.bids.lock().unwrap();
        if let Some(task) = tasks.get(&task_id) {
            if bid.proposed_time > task.deadline {
                return Err("Bid proposed time is past the task deadline".to_string());
            }
            if bid.proposed_reward > task.reward {
                return Err("Bid proposed reward exceeds task reward".to_string());
            }
            let task_version = task.version;
            let task = tasks.get_mut(&task_id).unwrap();
            if task.version != task_version {
                return Err("Task has been modified, please retry".to_string());
            }
            task.increment_version();
            bids.entry(task_id).or_insert_with(Vec::new).push(bid);
            blockchain.record_bid_submission(task_id, &bid)?;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn get_bids(&self, task_id: u64) -> Option<&Vec<Bid>> {
        let bids = self.bids.lock().unwrap();
        bids.get(&task_id)
    }
}

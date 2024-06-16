use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use crate::smart_contract::types::{SmartContract, Task as SCTask, Bid as SCBid, Reputation as SCReputation, Blockchain};
use crate::chain::blockchain::Blockchain;
use crate::qup::QUP;
use crate::marketplace::message_format::TaskAssignmentNotification;
use log::{error, info};

pub struct Marketplace {
    tasks: RwLock<HashMap<u64, Task>>,
    bids: RwLock<HashMap<u64, Vec<Bid>>>,
    round_robin_counter: AtomicUsize,
    reputation: Mutex<HashMap<String, f64>>,
    fn update_reputation(&self, node_id: &str, score_change: f64, success: bool) {
        let mut reputation = self.reputation.lock().unwrap();
        let decay_factor = 0.9;
        let weight = if success { 1.0 } else { -2.0 };
        let old_reputation = *reputation.get(node_id).unwrap_or(&0.0);
        let new_reputation = old_reputation * decay_factor + (weight * score_change);
        reputation.insert(node_id.to_string(), new_reputation.max(0.0)); // Ensure non-negative reputation
    }

    fn send_with_retry(&self, qup: &QUP, notification: &TaskAssignmentNotification) -> Result<(), String> {
        let mut attempts = 0;
        let max_attempts = 5;
        let mut delay = Duration::from_secs(1);

        while attempts < max_attempts {
            match qup.send_task_assignment_notification(&notification) {
                Ok(_) => {
                    info!("Task assignment notification sent successfully");
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to send task assignment notification: {}. Retrying in {:?}...", e, delay);
                    thread::sleep(delay);
                    delay *= 2; // Exponential backoff
                    attempts += 1;
                }
            }
        }

        Err("Max retry attempts reached".to_string())
    }

impl Marketplace {
    fn get_reputation_score(&self, node_id: &str) -> f64 {
        1.0
    }

    fn get_capability_score(&self, proof_of_capability: &str) -> f64 {
        1.0
    }

    fn get_time_score(&self, proposed_time: &DateTime<Utc>, deadline: &DateTime<Utc>) -> f64 {
        let duration = *deadline - *proposed_time;
        let total_duration = *deadline - Utc::now();
        (total_duration.num_seconds() - duration.num_seconds()) as f64 / total_duration.num_seconds() as f64
    }

    fn get_geographical_score(&self, node_id: &str) -> f64 {
        // Placeholder for actual geographical score calculation logic
        1.0
    }

    fn get_price_performance_score(&self, proposed_reward: u64, performance_score: f64) -> f64 {
        performance_score / proposed_reward as f64
    }

    fn break_tie(&self, bid1: &Bid, bid2: &Bid) -> bool {
        let counter = self.round_robin_counter.fetch_add(1, Ordering::SeqCst);
        counter % 2 == 0
    }

    fn get_historical_performance_score(&self, node_id: &str) -> f64 {
        1.0
    }

    fn get_node_stake(&self, node_id: &str) -> f64 {
        1.0
    }

    fn get_custom_metric_score(&self, custom_metric: &str) -> f64 {
        1.0
    }
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
    pub fn assign_task(&self, task_id: u64, blockchain: &Blockchain, qup: &QUP, current_block: u64, bid_expiration_blocks: u64) -> Result<(), String> {
        let task = self.tasks.read().unwrap().get(&task_id).ok_or("Task not found")?.clone();
        let current_block = blockchain.get_current_block_number()?;
        let bids = self.bids.read().unwrap();
        if current_block > bid_expiration_blocks {
            return None;
        }
        if let Some(bids) = bids.get(&task_id) {
            let valid_bids: Vec<Bid> = bids.iter().filter(|bid| current_block <= bid_expiration_blocks).cloned().collect();
            if let Some(best_bid) = self.select_best_bid(&task, &valid_bids) {
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

                // Update reputation for successful task completion
                self.update_reputation(&best_bid.node_id, task.reward as f64, true);
                self.bids.write().unwrap().remove(&task_id); // Remove bids after assignment
                let notification = TaskAssignmentNotification {
                    task_id: task.id,
                    assigned_node_id: best_bid.node_id.clone(),
                    details: format!("Task {} assigned to node {}", task.id, best_bid.node_id),
                };

                if let Err(e) = self.send_with_retry(&qup, &notification) {
                    error!("Failed to send task assignment notification: {}", e);
                    // Optionally notify the user or system administrator
                }
                Ok(())
            } else {
                Err("No valid bids found".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }
        reputation: Mutex::new(HashMap::new()),
    }

    fn select_best_bid(&self, task: &Task, bids: &[Bid]) -> Option<Bid> {
        // Calculate a score for each bid based on a weighted combination of factors
        let mut best_bid: Option<Bid> = None;
        let mut highest_score = f64::MIN;

        let default_weights = HashMap::from([
            ("reputation".to_string(), 0.3),
            ("capability".to_string(), 0.2),
            ("time".to_string(), 0.1),
            ("reward".to_string(), 0.1),
            ("geographical".to_string(), 0.1),
            ("price_performance".to_string(), 0.2),
        ]);

        let weights = task.weights.as_ref().unwrap_or(&default_weights);

        for bid in bids {
            let reputation_score = self.get_reputation_score(&bid.node_id);
            let capability_score = self.get_capability_score(&bid.proof_of_capability);
            let time_score = self.get_time_score(&bid.proposed_time, &task.deadline);
            let reward_score = 1.0 / bid.proposed_reward as f64; // Lower reward is better
            let geographical_score = self.get_geographical_score(&bid.node_id);
            let price_performance_score = self.get_price_performance_score(bid.proposed_reward, capability_score);

            let score = weights["reputation"] * reputation_score
                + weights["capability"] * capability_score
                + weights["time"] * time_score
                + weights["reward"] * reward_score
                + weights["geographical"] * geographical_score
                + weights["price_performance"] * price_performance_score;

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
        Self {
            tasks: RwLock::new(HashMap::new()),
            bids: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(0),
        }
    }

    pub fn add_task(&self, task: Task, blockchain: &Blockchain) -> Result<(), String> {
        task.validate()?;
        let current_block = blockchain.get_current_block_number()?;
        if current_block > task.deadline {
            return Err("Task deadline has passed".to_string());
        }

        let mut tasks = self.tasks.write().unwrap();
        if tasks.contains_key(&task.id) {
            return Err("Task ID already exists".to_string());
        }
        tasks.insert(task.id, task.clone());
        blockchain.record_task_submission(&task)?;
        Ok(())
    }

    pub fn get_task(&self, task_id: u64) -> Option<&Task> {
        let tasks = self.tasks.read().unwrap();
        tasks.get(&task_id)
    }

    pub fn add_bid(&self, task_id: u64, bid: Bid, blockchain: &Blockchain, minimum_stake: u64, current_block: u64, bid_expiration_blocks: u64) -> Result<(), String> {
        let tasks = self.tasks.read().unwrap();
        let current_block = blockchain.get_current_block_number()?;
        if current_block > bid_expiration_blocks {
            return Err("Bid has expired".to_string());
        }

        let mut bids = self.bids.write().unwrap();
        if let Some(task) = tasks.get(&task_id) {
            if bid.proposed_time > task.deadline {
                return Err("Bid proposed time is past the task deadline".to_string());
            }
            if bid.proposed_reward > task.reward {
                return Err("Bid proposed reward exceeds task reward".to_string());
            }
            if bid.proposed_reward < minimum_stake {
                return Err("Bid proposed reward is below the minimum stake".to_string());
            }
            if bids.get(&task_id).map_or(false, |bids| bids.iter().any(|b| b.node_id == bid.node_id)) {
                return Err("Duplicate bid from the same node".to_string());
            }
            let task_version = task.version;
            let mut task = tasks.get_mut(&task_id).unwrap();
            if task.version != task_version {
                return Err("Task has been modified, please retry".to_string());
            }
            task.increment_version();
            bids.entry(task_id).or_insert_with(Vec::new).push(bid.clone());
            blockchain.record_bid_submission(task_id, &bid)?;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn get_bids(&self, task_id: u64) -> Option<&Vec<Bid>> {
        let bids = self.bids.read().unwrap();
        bids.get(&task_id)
    }
}

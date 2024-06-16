use crate::marketplace::task::Task;
use crate::marketplace::bid::Bid;
use crate::smart_contract::types::{SmartContract, Task as SCTask, Bid as SCBid, Reputation as SCReputation, Blockchain};
use crate::chain::blockchain::Blockchain;
use crate::qup::QUP;
use crate::marketplace::message_format::TaskAssignmentNotification;
use crate::did::did::{DID, DIDDocument};
use crate::did::did_resolver::DIDResolver;
use log::{error, info};

pub struct Marketplace {
    tasks: RwLock<HashMap<u64, Task>>,
    bids: RwLock<HashMap<u64, Vec<Bid>>>,
    round_robin_counter: AtomicUsize,
    reputation: Mutex<HashMap<String, f64>>,
    qup: Arc<QUP>,
    did_resolver: Arc<dyn DIDResolver>,
}

impl Marketplace {
    fn update_reputation(&self, node_id: &str, score_change: f64, success: bool) {
        let mut reputation = self.reputation.lock().unwrap();
        let decay_factor = 0.9;
        let weight = if success { 1.0 } else { -2.0 };
        let old_reputation = *reputation.get(node_id).unwrap_or(&0.0);
        let new_reputation = old_reputation * decay_factor + (weight * score_change);
        reputation.insert(node_id.to_string(), new_reputation.max(0.0)); // Ensure non-negative reputation
    pub fn new(qup: Arc<QUP>, did_resolver: Arc<dyn DIDResolver>) -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
            bids: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(0),
            reputation: Mutex::new(HashMap::new()),
            qup,
            did_resolver,
        }
    }

    fn remove_expired_bids(&self, current_block: u64, bid_expiration_blocks: u64) {
        let mut bids = self.bids.write().unwrap();
        for (_, bid_list) in bids.iter_mut() {
            bid_list.retain(|bid| {
                let bid_age = current_block - bid.submission_time.timestamp() as u64;
                bid_age <= bid_expiration_blocks
            });
            qup,
            did_resolver,
        }
    }

    fn send_with_retry(&self, qup: &QUP, notification: &TaskAssignmentNotification) -> Result<(), String> {
        let mut attempts = 0;
        let max_attempts = 5;
        let base_delay = Duration::from_secs(1);
        let max_delay = Duration::from_secs(32);

        while attempts < max_attempts {
            match qup.send_task_assignment_notification(&notification) {
                Ok(_) => {
                    info!("Task assignment notification sent successfully");
                    return Ok(());
                }
                Err(e) => {
                    if is_transient_error(&e) {
                        let jitter: u64 = rand::thread_rng().gen_range(0..1000);
                        let delay = (base_delay * 2u32.pow(attempts)).min(max_delay) + Duration::from_millis(jitter);
                        error!("Transient error: {}. Retrying in {:?}...", e, delay);
                        thread::sleep(delay);
                        attempts += 1;
                    } else {
                        error!("Permanent error: {}. Not retrying.", e);
                        return Err(format!("Permanent error: {}", e));
                    }
                }
            }
        }

        Err("Max retry attempts reached".to_string())
    }

    fn is_transient_error(error: &str) -> bool {
        // Placeholder for actual transient error detection logic
        // For now, assume all errors are transient
        true
    }

impl Marketplace {
    fn get_reputation_score(&self, node_id: &str) -> f64 {
        let historical_performance = self.get_historical_performance_score(node_id);
        let recent_performance = self.get_recent_performance_score(node_id);
        let feedback = self.get_feedback_score(node_id);

        0.4 * historical_performance + 0.5 * recent_performance + 0.1 * feedback
    }

    fn get_historical_performance_score(&self, node_id: &str) -> f64 {
        let completion_rate = self.get_completion_rate(node_id);
        let timeliness = self.get_timeliness(node_id);
        let quality = self.get_quality(node_id);

        // Apply decay function (exponential decay)
        let decay_factor = 0.9;
        let recent_weight = 0.5;
        let historical_weight = 0.5;

        let recent_performance = recent_weight * (completion_rate + timeliness + quality) / 3.0;
        let historical_performance = historical_weight * (completion_rate + timeliness + quality) / 3.0 * decay_factor;

        recent_performance + historical_performance
    }

    fn get_completion_rate(&self, node_id: &str) -> f64 {
        // Placeholder for actual completion rate calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_timeliness(&self, node_id: &str) -> f64 {
        // Placeholder for actual timeliness calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_quality(&self, node_id: &str) -> f64 {
        // Placeholder for actual quality calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_recent_performance_score(&self, node_id: &str) -> f64 {
        // Placeholder for actual recent performance score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_feedback_score(&self, node_id: &str) -> f64 {
        // Placeholder for actual feedback score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_capability_score(&self, proof_of_capability: &str) -> f64 {
        // Placeholder for actual capability score calculation logic
        // For now, return a dummy value
        let hardware_spec_score = self.get_hardware_spec_score(proof_of_capability);
        let benchmark_score = self.get_benchmark_score(proof_of_capability);
        let past_task_score = self.get_past_task_score(proof_of_capability);

        let weights = HashMap::from([
            ("hardware_spec".to_string(), 0.4),
            ("benchmark".to_string(), 0.3),
            ("past_task".to_string(), 0.3),
        ]);

        weights["hardware_spec"] * hardware_spec_score
            + weights["benchmark"] * benchmark_score
            + weights["past_task"] * past_task_score
    }

    fn get_hardware_spec_score(&self, proof_of_capability: &str) -> f64 {
        // Placeholder for actual hardware spec score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_benchmark_score(&self, proof_of_capability: &str) -> f64 {
        // Placeholder for actual benchmark score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_past_task_score(&self, proof_of_capability: &str) -> f64 {
        // Placeholder for actual past task score calculation logic
        // For now, return a dummy value
        1.0
    }

    fn get_time_score(&self, proposed_time: &DateTime<Utc>, deadline: &DateTime<Utc>) -> f64 {
        let duration = *deadline - *proposed_time;
        let total_duration = *deadline - Utc::now();
        (total_duration.num_seconds() - duration.num_seconds()) as f64 / total_duration.num_seconds() as f64
    }

    fn get_geographical_score(&self, node_id: &str) -> f64 {
        let distance = self.calculate_distance(node_id);
        let latency = self.measure_latency(node_id);

        // Inverse proportionality: closer and lower latency = higher score
        let distance_score = 1.0 / (1.0 + distance);
        let latency_score = 1.0 / (1.0 + latency);

        (distance_score + latency_score) / 2.0
    }

    fn calculate_distance(&self, node_id: &str) -> f64 {
        // Placeholder for actual distance calculation logic
        // For now, return a dummy value
        1.0
    }

    fn measure_latency(&self, node_id: &str) -> f64 {
        // Placeholder for actual latency measurement logic
        // For now, return a dummy value
        1.0
    }

    fn get_price_performance_score(&self, proposed_reward: u64, performance_score: f64) -> f64 {
        performance_score / proposed_reward as f64
    }

    fn break_tie(&self, bid1: &Bid, bid2: &Bid) -> bool {
        // Random approach for tie-breaking
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            return true;
        }

        // Optional additional tie-breakers
        let uptime1 = self.get_node_uptime(&bid1.node_id);
        let uptime2 = self.get_node_uptime(&bid2.node_id);
        if uptime1 != uptime2 {
            return uptime1 > uptime2;
        }

        let age1 = self.get_node_age(&bid1.node_id);
        let age2 = self.get_node_age(&bid2.node_id);
        age1 > age2
    }

    fn get_historical_performance_score(&self, node_id: &str) -> f64 {
        1.0
    }

    fn get_node_uptime(&self, node_id: &str) -> u64 {
        // Placeholder for actual node uptime retrieval logic
        // For now, return a dummy value
        100
    }

    fn get_node_age(&self, node_id: &str) -> u64 {
        // Placeholder for actual node age retrieval logic
        // For now, return a dummy value
        50
    }

    fn get_node_stake(&self, node_id: &str) -> f64 {
        let staked_amount = self.get_staked_amount(node_id);
        let stake_duration = self.get_stake_duration(node_id);

        // Weights for staked amount and stake duration
        let staked_amount_weight = 0.7;
        let stake_duration_weight = 0.3;

        staked_amount_weight * staked_amount + stake_duration_weight * stake_duration
    }

    fn get_staked_amount(&self, node_id: &str) -> f64 {
        // Placeholder for actual staked amount retrieval logic
        // For now, return a dummy value
        1.0
    }

    fn get_stake_duration(&self, node_id: &str) -> f64 {
        // Placeholder for actual stake duration retrieval logic
        // For now, return a dummy value
        1.0
    }

    fn get_custom_metric_score(&self, custom_metric: &str, node_id: &str) -> f64 {
        // Placeholder for actual custom metric score calculation logic
        // For now, return a dummy value
        let metric_value = self.verify_custom_metric(custom_metric, node_id);
        metric_value
    }

    fn verify_custom_metric(&self, custom_metric: &str, node_id: &str) -> f64 {
        // Placeholder for actual custom metric verification logic
        // This could involve checking attestations or on-chain data
        // For now, return a dummy value
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
        // Round-robin approach for tie-breaking
        let counter = self.round_robin_counter.fetch_add(1, Ordering::SeqCst);
        if counter % 2 == 0 {
            return true;
        }

        // Optional additional tie-breakers
        let uptime1 = self.get_node_uptime(&bid1.node_id);
        let uptime2 = self.get_node_uptime(&bid2.node_id);
        if uptime1 != uptime2 {
            return uptime1 > uptime2;
        }

        let age1 = self.get_node_age(&bid1.node_id);
        let age2 = self.get_node_age(&bid2.node_id);
        age1 > age2
    }

impl Marketplace {
    pub fn assign_task(&self, task_id: u64, blockchain: &Blockchain, qup: &QUP, current_block: u64, bid_expiration_blocks: u64) -> Result<(), String> {
        let task = self.tasks.read().unwrap().get(&task_id).ok_or("Task not found")?.clone();
        let current_block = blockchain.get_current_block_number()?;
        let bids = self.bids.read().unwrap();
        if let Some(bids) = bids.get(&task_id) {
            let valid_bids: Vec<Bid> = bids.iter().filter(|bid| {
                let bid_age = current_block - bid.submission_time.timestamp() as u64;
                bid_age <= bid_expiration_blocks
            }).cloned().collect();
            if let Some(best_bid) = self.select_best_bid(&task, &valid_bids) {
                if let Some(problem_proposal) = &task.problem_proposal {
                    let solution = self.qup.solve_useful_work_problem(&problem_proposal.problem);
                    if self.qup.validate_useful_work_solution(&solution) {
                        self.qup.state.accept_problem_proposal(problem_proposal.clone());
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
                        Err("Invalid useful work solution".to_string())
                    }
                } else {
                    Err("No problem proposal found for the task".to_string())
                }
            } else {
                Err("No valid bids found".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }

    fn select_best_bid(&self, task: &Task, bids: &[Bid]) -> Option<Bid> {
        // Calculate a score for each bid based on a weighted combination of factors
        let mut best_bid: Option<Bid> = None;
        let mut highest_score = f64::MIN;

        let default_weights = HashMap::from([
            ("reputation".to_string(), 0.3),
            ("capability".to_string(), 0.2),
            ("time".to_string(), 0.1),
            ("geographical".to_string(), 0.1),
            ("price_performance".to_string(), 0.3),
        ]);

        let weights = task.weights.as_ref().unwrap_or(&default_weights);

        let mut rng = rand::thread_rng();
        let mut bid_scores: Vec<(Bid, f64)> = bids.iter().map(|bid| {
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

            (bid.clone(), score)
        }).collect();

        // Collusion detection: flag identical bids from multiple nodes
        let mut bid_patterns: HashMap<String, Vec<String>> = HashMap::new();
        for (bid, _) in &bid_scores {
            let key = format!("{}-{}", bid.proposed_reward, bid.proposed_time);
            bid_patterns.entry(key).or_default().push(bid.node_id.clone());
        }
        for (key, nodes) in bid_patterns {
            if nodes.len() > 1 {
                error!("Potential collusion detected: identical bids from nodes {:?}", nodes);
            }
        }

        // Fair task assignment: weighted random selection
        bid_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        let total_score: f64 = bid_scores.iter().map(|(_, score)| *score).sum();
        let mut cumulative_score = 0.0;
        let random_value: f64 = rng.gen_range(0.0..total_score);
        for (bid, score) in bid_scores {
            cumulative_score += score;
            if cumulative_score >= random_value {
                best_bid = Some(bid);
                break;
            }
        }


        best_bid
    }
}

impl Marketplace {
    pub fn new(qup: Arc<QUP>, did_resolver: Arc<dyn DIDResolver>) -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
            bids: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(0),
        }
    }

    pub fn add_task(&self, task: Task, blockchain: &Blockchain, qup: &QUP) -> Result<(), String> {
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
        if let Some(problem_proposal) = &task.problem_proposal {
            let proposal = qup.validator.propose_useful_work_problem(problem_proposal.problem.clone());
            qup.state.add_problem_proposal(proposal);
        }
        Ok(())
    }

    pub fn get_task(&self, task_id: u64) -> Option<&Task> {
        let tasks = self.tasks.read().unwrap();
        let reputation_score = self.get_reputation_score(&bid.node_id);
        let min_reputation_score = 0.5; // Example threshold

        if reputation_score < min_reputation_score {
            return Err("Node reputation too low to submit bid".to_string());
        }

        // Rate limiting: allow only a certain number of bids per node per time period
        let max_bids_per_node = 5;
        let mut node_bid_count: HashMap<String, usize> = HashMap::new();
        for (_, bid_list) in self.bids.read().unwrap().iter() {
            for b in bid_list {
                *node_bid_count.entry(b.node_id.clone()).or_default() += 1;
            }
        }
        if *node_bid_count.get(&bid.node_id).unwrap_or(&0) >= max_bids_per_node {
            return Err("Node has reached the maximum number of bids allowed".to_string());
        }
        tasks.get(&task_id)
    }

    pub fn add_bid(&self, task_id: u64, bid: Bid, blockchain: &Blockchain, qup: &QUP, minimum_stake: u64, bid_expiration_blocks: u64) -> Result<(), String> {
        let tasks = self.tasks.read().unwrap();
        let current_block = blockchain.get_current_block_number()?;
        if current_block > bid_expiration_blocks {
            return Err("Bid has expired".to_string());
        }

        let mut bids = self.bids.write().unwrap();
        if let Some(task) = tasks.get(&task_id) {
            let current_block = blockchain.get_current_block_number()?;
            if current_block > bid_expiration_blocks {
                return Err("Bid has expired".to_string());
            }
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
            if let Some(problem_proposal) = &task.problem_proposal {
                let proposal = qup.validator.propose_useful_work_problem(problem_proposal.problem.clone());
                qup.state.add_problem_proposal(proposal);
            }
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

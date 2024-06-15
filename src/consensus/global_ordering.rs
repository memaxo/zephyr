use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LamportTimestamp {
    pub counter: u64,
    pub node_id: String,
    pub fn get_load(&self) -> f64 {
        // Placeholder for actual network load calculation
        0.5
    }
}

pub fn calculate_reward(base_reward: u64, utility_points: u64, stake: u64, total_up: u64, config: &QUPConfig) -> u64 {
    let utility_point_multiplier = config.useful_work_reward_multiplier;
    let stake_factor = stake as f64 / total_up as f64;
    let dynamic_adjustment = if total_up > config.target_utility_points {
        1.0 - ((total_up - config.target_utility_points) as f64 / total_up as f64)
    } else {
        1.0 + ((config.target_utility_points - total_up) as f64 / total_up as f64)
    };

    let reward = base_reward + (utility_points * utility_point_multiplier) as u64;
    (reward as f64 * stake_factor * dynamic_adjustment) as u64
}

impl LamportTimestamp {
    pub fn new(counter: u64, node_id: String) -> Self {
        LamportTimestamp { counter, node_id }
    }
}

pub fn assign_timestamp(transaction: &mut Transaction, node_id: String, counter: &mut u64) {
    *counter += 1;
    transaction.timestamp = LamportTimestamp::new(*counter, node_id);
}

pub fn compare_timestamps(ts1: &LamportTimestamp, ts2: &LamportTimestamp) -> Ordering {
    match ts1.counter.cmp(&ts2.counter) {
        Ordering::Equal => ts1.node_id.cmp(&ts2.node_id),
        other => other,
    }
}

pub fn resolve_conflicts(conflicting_transactions: Vec<Transaction>) -> Transaction {
    // Example conflict resolution: choose the transaction with the smallest hash
    conflicting_transactions.into_iter().min_by_key(|tx| tx.hash()).unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorClock {
    pub clock: HashMap<String, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        VectorClock {
            clock: HashMap::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        let counter = self.clock.entry(node_id.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node_id, counter) in &other.clock {
            let entry = self.clock.entry(node_id.clone()).or_insert(0);
            *entry = (*entry).max(*counter);
        }
    }
}

pub fn assign_vector_clock(transaction: &mut Transaction, node_id: &str, clock: &mut VectorClock) {
    clock.increment(node_id);
    transaction.vector_clock = clock.clone();
}

pub fn compare_vector_clocks(vc1: &VectorClock, vc2: &VectorClock) -> Ordering {
    let mut vc1_dominates = false;
    let mut vc2_dominates = false;

    for (node_id, counter1) in &vc1.clock {
        if let Some(counter2) = vc2.clock.get(node_id) {
            if counter1 > counter2 {
                vc1_dominates = true;
            } else if counter1 < counter2 {
                vc2_dominates = true;
            }
        } else {
            vc1_dominates = true;
        }
    }

    for (node_id, counter2) in &vc2.clock {
        if vc1.clock.get(node_id).is_none() {
            vc2_dominates = true;
        }
    }

    match (vc1_dominates, vc2_dominates) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub timestamp: LamportTimestamp,
    pub vector_clock: VectorClock,
    // Other transaction fields...
}

impl Transaction {
    pub fn new(id: String) -> Self {
        Transaction {
            id,
            timestamp: LamportTimestamp::new(0, String::new()),
            vector_clock: VectorClock::new(),
            // Initialize other fields...
        }
    }

    pub fn hash(&self) -> String {
        // Placeholder for actual hash computation
        format!("{:x}", md5::compute(&self.id))
    }
}

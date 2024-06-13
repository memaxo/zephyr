use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StakingInfo {
    pub amount: u64,
    pub lock_period: u64,
    pub start_time: u64,
}

pub struct Staking {
    pub stakes: HashMap<String, StakingInfo>,
    pub slashing_conditions: Vec<SlashingCondition>,
}

pub struct SlashingCondition {
    pub condition: Box<dyn Fn(&StakingInfo) -> bool>,
    pub penalty: u64,
}

impl Staking {
    pub fn new() -> Self {
        Staking {
            stakes: HashMap::new(),
            slashing_conditions: Vec::new(),
        }
    }

    pub fn stake(&mut self, validator: String, amount: u64, lock_period: u64) {
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let staking_info = StakingInfo {
            amount,
            lock_period,
            start_time,
        };
        self.stakes.insert(validator, staking_info);
    }

    pub fn unstake(&mut self, validator: &str) -> Result<u64, String> {
        if let Some(staking_info) = self.stakes.get(validator) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if current_time >= staking_info.start_time + staking_info.lock_period {
                let amount = staking_info.amount;
                self.stakes.remove(validator);
                Ok(amount)
            } else {
                Err("Lock period not yet expired".to_string())
            }
        } else {
            Err("Validator not found".to_string())
        }
    }

    pub fn calculate_rewards(&self, validator: &str, reward_rate: f64) -> Result<u64, String> {
        if let Some(staking_info) = self.stakes.get(validator) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let staking_duration = current_time - staking_info.start_time;
            let rewards = (staking_info.amount as f64 * reward_rate * staking_duration as f64) as u64;
            Ok(rewards)
        } else {
            Err("Validator not found".to_string())
        }
    }

    pub fn slash(&mut self, validator: &str) -> Result<u64, String> {
        if let Some(staking_info) = self.stakes.get_mut(validator) {
            for condition in &self.slashing_conditions {
                if (condition.condition)(staking_info) {
                    let penalty = condition.penalty;
                    staking_info.amount = staking_info.amount.saturating_sub(penalty);
                    return Ok(penalty);
                }
            }
            Err("No slashing condition met".to_string())
        } else {
            Err("Validator not found".to_string())
        }
    }

    pub fn add_slashing_condition<F>(&mut self, condition: F, penalty: u64)
    where
        F: Fn(&StakingInfo) -> bool + 'static,
    {
        self.slashing_conditions.push(SlashingCondition {
            condition: Box::new(condition),
            penalty,
        });
    }
}

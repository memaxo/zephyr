use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::info;

pub struct Governance {
    proposals: Arc<Mutex<HashMap<String, Proposal>>>,
}

pub struct Proposal {
    new_version: String,
    new_code: String,
    votes: HashMap<String, bool>,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn propose_upgrade(&self, proposal_id: &str, new_version: String, new_code: String) {
        let mut proposals = self.proposals.lock().unwrap();
        proposals.insert(proposal_id.to_string(), Proposal {
            new_version,
            new_code,
            votes: HashMap::new(),
        });
        info!("Proposed upgrade {} to version {}", proposal_id, new_version);
    }

    pub fn vote(&self, proposal_id: &str, voter: &str, approve: bool) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();
        let proposal = proposals.get_mut(proposal_id).ok_or("Proposal not found")?;
        proposal.votes.insert(voter.to_string(), approve);
        info!("Voter {} voted {} on proposal {}", voter, approve, proposal_id);
        Ok(())
    }

    pub fn execute_upgrade(&self, proposal_id: &str) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();
        let proposal = proposals.remove(proposal_id).ok_or("Proposal not found")?;

        let total_votes = proposal.votes.len();
        let approve_votes = proposal.votes.values().filter(|&&v| v).count();

        if approve_votes > total_votes / 2 {
            info!("Executing upgrade to version {}", proposal.new_version);
            // Version Compatibility Check
            let current_version = context.get_contract_version()?;
            if current_version != self.config.expected_version {
                return Err("Incompatible contract version".to_string());
            }

            // Data Migration (if necessary)
            if let Err(e) = context.migrate_data(&self.config.new_schema) {
                return Err(format!("Data migration failed: {}", e));
            }

            // Emit Event
            context.emit_event("AssetsLocked", format!("{} assets locked by {}", amount, sender))?;

            // Error Handling
            if let Err(e) = context.finalize_transaction() {
                return Err(format!("Transaction finalization failed: {}", e));
            }
            Ok(())
        } else {
            Err("Upgrade proposal rejected".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propose_and_vote() {
        let governance = Governance::new();
        governance.propose_upgrade("proposal_1", "2.0".to_string(), "new_code".to_string());
        governance.vote("proposal_1", "voter_1", true).unwrap();
        governance.vote("proposal_1", "voter_2", false).unwrap();
        governance.vote("proposal_1", "voter_3", true).unwrap();
        let result = governance.execute_upgrade("proposal_1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_rejected_proposal() {
        let governance = Governance::new();
        governance.propose_upgrade("proposal_2", "2.0".to_string(), "new_code".to_string());
        governance.vote("proposal_2", "voter_1", false).unwrap();
        governance.vote("proposal_2", "voter_2", false).unwrap();
        let result = governance.execute_upgrade("proposal_2");
        assert!(result.is_err());
    }
}

use crate::qup::types::{NodeId, AgentId, StakeAmount, ChallengeId, VerificationResult};
use crate::qup::state::QUPState;
use std::collections::HashMap;

pub struct VerificationGame {
    challenges: HashMap<ChallengeId, Challenge>,
}

struct Challenge {
    agent: AgentId,
    node: NodeId,
    task_id: String,
    stake: StakeAmount,
}

impl VerificationGame {
    pub fn new() -> Self {
        VerificationGame {
            challenges: HashMap::new(),
        }
    }

    pub fn initiate_challenge(
        &mut self,
        challenge_id: ChallengeId,
        agent: AgentId,
        node: NodeId,
        task_id: String,
        stake: StakeAmount,
    ) {
        let challenge = Challenge {
            agent,
            node,
            task_id,
            stake,
        };
        self.challenges.insert(challenge_id, challenge);
    }

    pub fn resolve_challenge(
        &mut self,
        challenge_id: ChallengeId,
        result: VerificationResult,
        state: &mut QUPState,
    ) {
        if let Some(challenge) = self.challenges.remove(&challenge_id) {
            match result {
                VerificationResult::Pass => {
                    // The node passes the verification
                    // Reward the node and return the stake to the agent
                    self.reward_node(&challenge.node, challenge.stake, state);
                    self.return_stake_to_agent(&challenge.agent, challenge.stake, state);
                }
                VerificationResult::Fail => {
                    // The node fails the verification
                    // Penalize the node and reward the agent
                    self.penalize_node(&challenge.node, challenge.stake, state);
                    self.reward_agent(&challenge.agent, challenge.stake, state);
                }
            }
        }
    }

    fn reward_node(&self, node_id: &NodeId, amount: StakeAmount, state: &mut QUPState) {
        let node_stake = state.get_stake_mut(node_id);
        *node_stake += amount;
    }

    fn return_stake_to_agent(&self, agent_id: &AgentId, amount: StakeAmount, state: &mut QUPState) {
        let agent_balance = state.get_balance_mut(agent_id);
        *agent_balance += amount;
    }

    fn penalize_node(&self, node_id: &NodeId, amount: StakeAmount, state: &mut QUPState) {
        let node_stake = state.get_stake_mut(node_id);
        *node_stake -= amount;
    }

    fn reward_agent(&self, agent_id: &AgentId, amount: StakeAmount, state: &mut QUPState) {
        let agent_balance = state.get_balance_mut(agent_id);
        *agent_balance += amount;
    }
}
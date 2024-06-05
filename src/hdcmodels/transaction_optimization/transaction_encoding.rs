use crate::chain::state::state_manager::StateManager;
use crate::chain::transaction::Transaction;
use crate::hdcmodels::encoding::encode_transactional_data;
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;
use std::sync::Arc;

pub struct TransactionEncoder {
    hdc_model: HDCModel,
}

impl TransactionEncoder {
    pub fn new(hdc_model: HDCModel) -> Self {
        TransactionEncoder { hdc_model }
    }

    pub fn encode_transaction(&self, transaction: &Transaction) -> Vec<f64> {
        encode_transactional_data(&[transaction.clone()], self.hdc_model.dimension)
    }

    pub fn encode_state_transition(&self, state_transition: &StateTransition) -> Vec<f64> {
        let mut encoded_transactions = Vec::new();
        for transaction in &state_transition.transactions {
            let encoded_transaction = self.encode_transaction(transaction);
            encoded_transactions.push(encoded_transaction);
        }

        // Combine the encoded transactions using HDC operations
        let mut encoded_state_transition = vec![0.0; self.hdc_model.dimension];
        for encoded_transaction in &encoded_transactions {
            for (i, &value) in encoded_transaction.iter().enumerate() {
                encoded_state_transition[i] += value;
            }
        }

        // Normalize the encoded state transition vector
        let magnitude = encoded_state_transition
            .iter()
            .map(|&x| x * x)
            .sum::<f64>()
            .sqrt();
        encoded_state_transition
            .iter_mut()
            .for_each(|x| *x /= magnitude);

        encoded_state_transition
    }

    pub fn find_similar_transactions(
        &self,
        transaction: &Transaction,
        transaction_pool: &[Transaction],
        top_n: usize,
    ) -> Vec<(Transaction, f64)> {
        let transaction_vector = self.encode_transaction(transaction);
        let mut similarities: Vec<(Transaction, f64)> = transaction_pool
            .iter()
            .map(|t| {
                let t_vector = self.encode_transaction(t);
                let similarity = cosine_similarity(&transaction_vector, &t_vector);
                (t.clone(), similarity)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_n);
        similarities
    }

    pub fn find_similar_state_transitions(
        &self,
        state_transition: &StateTransition,
        state_transition_pool: &[StateTransition],
        top_n: usize,
    ) -> Vec<(StateTransition, f64)> {
        let state_transition_vector = self.encode_state_transition(state_transition);
        let mut similarities: Vec<(StateTransition, f64)> = state_transition_pool
            .iter()
            .map(|st| {
                let st_vector = self.encode_state_transition(st);
                let similarity = cosine_similarity(&state_transition_vector, &st_vector);
                (st.clone(), similarity)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_n);
        similarities
    }
}

pub struct StateTransition {
    pub transactions: Vec<Transaction>,
    pub state_manager: Arc<StateManager>,
}

impl StateTransition {
    pub fn new(transactions: Vec<Transaction>, state_manager: Arc<StateManager>) -> Self {
        StateTransition {
            transactions,
            state_manager,
        }
    }
}

use crate::hdcmodels::transaction_optimization::transaction_encoding::{
    StateTransition, Transaction, TransactionEncoder,
};
use crate::hdcmodels::HDCModel;
use crate::qup::optimizer::QuantumSecureOptimizer;

pub struct TransactionOptimizer {
    encoder: TransactionEncoder,
    optimizer: QuantumSecureOptimizer,
}

impl TransactionOptimizer {
    pub fn new(hdc_model: HDCModel, optimizer: QuantumSecureOptimizer) -> Self {
        let encoder = TransactionEncoder::new(hdc_model);
        TransactionOptimizer { encoder, optimizer }
    }

    pub fn optimize_transaction(&self, transaction: &Transaction) -> Transaction {
        // Encode the transaction using the TransactionEncoder
        let encoded_transaction = self.encoder.encode_transaction(transaction);

        // Optimize the encoded transaction using the QuantumSecureOptimizer
        let optimized_encoded_transaction = self.optimizer.optimize(&encoded_transaction);

        // Decode the optimized encoded transaction back into a Transaction
        let optimized_transaction = self
            .decoder
            .decode_transaction(&optimized_encoded_transaction);

        optimized_transaction
    }

    pub fn optimize_state_transition(&self, state_transition: &StateTransition) -> StateTransition {
        let mut optimized_transactions = Vec::new();
        for transaction in &state_transition.transactions {
            let optimized_transaction = self.optimize_transaction(transaction);
            optimized_transactions.push(optimized_transaction);
        }

        StateTransition {
            transactions: optimized_transactions,
        }
    }

    pub fn identify_performance_improvements(
        &self,
        transactions: &[Transaction],
    ) -> Vec<Transaction> {
        let mut optimized_transactions = Vec::new();
        for transaction in transactions {
            let optimized_transaction = self.optimize_transaction(transaction);
            optimized_transactions.push(optimized_transaction);
        }
        optimized_transactions
    }

    pub fn identify_cost_reductions(&self, transactions: &[Transaction]) -> Vec<Transaction> {
        let mut optimized_transactions = Vec::new();
        for transaction in transactions {
            let optimized_transaction = self.optimize_transaction(transaction);
            optimized_transactions.push(optimized_transaction);
        }
        optimized_transactions
    }

    pub fn identify_security_enhancements(&self, transactions: &[Transaction]) -> Vec<Transaction> {
        let mut optimized_transactions = Vec::new();
        for transaction in transactions {
            let optimized_transaction = self.optimize_transaction(transaction);
            optimized_transactions.push(optimized_transaction);
        }
        optimized_transactions
    }
}

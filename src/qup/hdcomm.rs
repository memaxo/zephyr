use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;
use crate::qup::block::QUPBlock;
use crate::qup::config::QUPConfig;
use crate::qup::state::QUPState;
use crate::qup::transaction::Transaction;
use std::sync::Arc;

pub struct HDCommunication {
    pub config: Arc<QUPConfig>,
    pub hdc_model: HDCModel,
}

impl HDCommunication {
    pub fn new(config: Arc<QUPConfig>, hdc_model: HDCModel) -> Self {
        HDCommunication { config, hdc_model }
    }

    pub fn encode_block(&self, block: &QUPBlock) -> Vec<f64> {
        let mut encoded_data = Vec::new();

        // Encode the block height
        encoded_data.extend(encode_data(&block.height.to_le_bytes()));

        // Encode the block timestamp
        encoded_data.extend(encode_data(&block.timestamp.to_le_bytes()));

        // Encode the previous block hash
        encoded_data.extend(encode_data(&block.prev_block_hash));

        // Encode the transactions
        for tx in &block.transactions {
            encoded_data.extend(self.encode_transaction(tx));
        }

        encoded_data
    }

    pub fn decode_block(&self, encoded_block: &[f64]) -> QUPBlock {
        let mut decoded_data = encoded_block.to_vec();

        // Decode the block height
        let height = u64::from_le_bytes(decode_data(&mut decoded_data).try_into().unwrap());

        // Decode the block timestamp
        let timestamp = u64::from_le_bytes(decode_data(&mut decoded_data).try_into().unwrap());

        // Decode the previous block hash
        let prev_block_hash = decode_data(&mut decoded_data);

        // Decode the transactions
        let mut transactions = Vec::new();
        while !decoded_data.is_empty() {
            let tx = self.decode_transaction(&mut decoded_data);
            transactions.push(tx);
        }

        QUPBlock {
            height,
            timestamp,
            prev_block_hash,
            transactions,
            hdc_encoded_block: encoded_block.to_vec(),
        }
    }

    pub fn evaluate_block_similarity(&self, block1: &QUPBlock, block2: &QUPBlock) -> f64 {
        let encoded_block1 = &block1.hdc_encoded_block;
        let encoded_block2 = &block2.hdc_encoded_block;

        cosine_similarity(encoded_block1, encoded_block2)
    }

    pub fn optimize_block(&self, block: &QUPBlock) -> QUPBlock {
        let encoded_block = self.encode_block(block);
        let optimized_encoded_block = self.hdc_model.optimize(&encoded_block);

        self.decode_block(&optimized_encoded_block)
    }

    fn encode_transaction(&self, tx: &Transaction) -> Vec<f64> {
        let mut encoded_data = Vec::new();

        // Encode the transaction sender
        encoded_data.extend(encode_data(&tx.sender));

        // Encode the transaction recipient
        encoded_data.extend(encode_data(&tx.recipient));

        // Encode the transaction amount
        encoded_data.extend(encode_data(&tx.amount.to_le_bytes()));

        // Encode the transaction fee
        encoded_data.extend(encode_data(&tx.fee.to_le_bytes()));

        encoded_data
    }

    fn decode_transaction(&self, encoded_tx: &mut Vec<f64>) -> Transaction {
        // Decode the transaction sender
        let sender = decode_data(encoded_tx);

        // Decode the transaction recipient
        let recipient = decode_data(encoded_tx);

        // Decode the transaction amount
        let amount = u64::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());

        // Decode the transaction fee
        let fee = u64::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());

        Transaction {
            sender,
            recipient,
            amount,
            fee,
        }
    }
}

use crate::hdcmodels::encoding::{decode_data, encode_data, encode_transactional_data};
use crate::qup::transaction::{Transaction, TransactionFee};
use crate::qup::block::QUPBlock;

pub fn encode_transaction(tx: &Transaction) -> Vec<f64> {
    encode_transactional_data(&[tx.clone()], 128) // Assuming dimension is 128
}

pub fn decode_transaction(encoded_tx: &mut Vec<f64>) -> Transaction {
    // Assuming decode_data can handle the encoded transaction data
    let sender = decode_data(encoded_tx);
    let recipient = decode_data(encoded_tx);
    let amount = u64::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());
    let fee = TransactionFee::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());

    Transaction { sender, recipient, amount, fee }
}

pub fn encode_block(block: &QUPBlock) -> Vec<f64> {
    let mut encoded_data = Vec::new();

    // Encode the block height
    encoded_data.extend(encode_data(&block.height.to_le_bytes()));

    // Encode the block timestamp
    encoded_data.extend(encode_data(&block.timestamp.to_le_bytes()));

    // Encode the previous block hash
    encoded_data.extend(encode_data(&block.prev_block_hash));

    // Encode the transactions using the refactored encode_transaction
    let transactions_encoded: Vec<f64> = block.transactions.iter()
        .flat_map(|tx| encode_transaction(tx))
        .collect();

    encoded_data.extend(transactions_encoded);
    encoded_data
}

pub fn decode_block(encoded_block: &[f64]) -> QUPBlock {
    let mut decoded_data = encoded_block.to_vec();

    // Decode the block height
    let height = u64::from_le_bytes(decode_data(&mut decoded_data).try_into().unwrap());

    // Decode the block timestamp
    let timestamp = u64::from_le_bytes(decode_data(&mut decoded_data).try_into().unwrap());

    // Decode the previous block hash
    let prev_block_hash = decode_data(&mut decoded_data);

    // Decode the transactions using the refactored decode_transaction
    let mut transactions = Vec::new();
    while !decoded_data.is_empty() {
        transactions.push(decode_transaction(&mut decoded_data));
    }

    QUPBlock { height, timestamp, prev_block_hash, transactions, hdc_encoded_block: encoded_block.to_vec() }
}

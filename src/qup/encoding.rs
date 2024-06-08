use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::qup::transaction::{Transaction, TransactionFee};
use crate::qup::block::QUPBlock;

pub fn encode_transaction(tx: &Transaction) -> Vec<f64> {
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

pub fn decode_transaction(encoded_tx: &mut Vec<f64>) -> Transaction {
    // Decode the transaction sender
    let sender = decode_data(encoded_tx);

    // Decode the transaction recipient
    let recipient = decode_data(encoded_tx);

    // Decode the transaction amount
    let amount = u64::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());

    // Decode the transaction fee
    let fee = TransactionFee::from_le_bytes(decode_data(encoded_tx).try_into().unwrap());

    Transaction {
        sender,
        recipient,
        amount,
        fee,
    }
}

pub fn encode_block(block: &QUPBlock) -> Vec<f64> {
    let mut encoded_data = Vec::new();

    // Encode the block height
    encoded_data.extend(encode_data(&block.height.to_le_bytes()));

    // Encode the block timestamp
    encoded_data.extend(encode_data(&block.timestamp.to_le_bytes()));

    // Encode the previous block hash
    encoded_data.extend(encode_data(&block.prev_block_hash));

    // Encode the transactions
    for tx in &block.transactions {
        encoded_data.extend(encode_transaction(tx));
    }

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

    // Decode the transactions
    let mut transactions = Vec::new();
    while !decoded_data.is_empty() {
        let tx = decode_transaction(&mut decoded_data);
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

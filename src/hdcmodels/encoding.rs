use crate::hdcmodels::encoding::centralized::*;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use openai_rs::GPT3;
use regex::Regex;

// Rust Code Encoding
pub fn encode_rust_code(code: &str, dimension: usize) -> Vec<f64> {
    let ast = syn::parse_file(code).expect("Failed to parse code");
    let ast_features = extract_ast_features(&ast);
    let codebert_embeddings = CodeBERT::encode(&ast_features);
    let gnn_embeddings = GraphNeuralNetwork::encode(&ast);
    let combined_embeddings = combine_embeddings(&codebert_embeddings, &gnn_embeddings);
    let final_encoding = dimensionality_reduction(&combined_embeddings, dimension);
    final_encoding
}

use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

pub fn encode_natural_language(text: &str, dimension: usize) -> Vec<f64> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::Bert).create_model().unwrap();
    let embeddings = model.encode(&[text]).unwrap();
    let embedding = &embeddings[0];

    // Perform dimensionality reduction to get final encoding vector
    let final_encoding = dimensionality_reduction(&vec![embedding.to_vec()], dimension);
    final_encoding[0].clone()
}

use smart_contract_optimizer::{optimize_gas, optimize_bytecode};

// Smart Contract Encoding
pub fn encode_smart_contract(contract: &str, dimension: usize) -> Vec<f64> {
    // 1. Tokenize contract using domain-specific tokenizer for smart contract language
    // 2. Extract function signatures and parameter types
    // 3. Perform control flow analysis to identify execution paths
    // 4. Combine token embeddings, function features, and control flow patterns
    // 5. Perform dimensionality reduction to get final encoding vector
    let mut encoded_contract = vec![0.0; dimension]; // Placeholder

    // Gas optimization
    encoded_contract = optimize_gas(encoded_contract);

    // Bytecode optimization
    encoded_contract = optimize_bytecode(encoded_contract);

    encoded_contract
}

use std::collections::HashSet;

pub fn encode_transactional_data(data: &[Transaction], dimension: usize) -> Vec<f64> {
    let mut encoded_data = Vec::new();
    let mut unique_senders = HashSet::new();
    let mut unique_receivers = HashSet::new();

    // Extract unique senders and receivers for one-hot encoding
    for transaction in data {
        unique_senders.insert(&transaction.sender);
        unique_receivers.insert(&transaction.receiver);
    }

    for transaction in data {
        let mut transaction_encoding = Vec::new();

        // One-hot encoding for sender
        for sender in &unique_senders {
            if sender == &&transaction.sender {
                transaction_encoding.push(1.0);
            } else {
                transaction_encoding.push(0.0);
            }
        }

        // One-hot encoding for receiver
        for receiver in &unique_receivers {
            if receiver == &&transaction.receiver {
                transaction_encoding.push(1.0);
            } else {
                transaction_encoding.push(0.0);
            }
        }

        // Normalization for amount
        let normalized_amount = (transaction.amount - data.iter().map(|t| t.amount).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            / (data.iter().map(|t| t.amount).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - data.iter().map(|t| t.amount).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        transaction_encoding.push(normalized_amount);

        // Timestamp encoding
        let timestamp_encoding = transaction.timestamp as f64;
        transaction_encoding.push(timestamp_encoding);

        encoded_data.push(transaction_encoding);
    }

    // Flatten the encoded data
    let flattened_data: Vec<f64> = encoded_data.into_iter().flatten().collect();

    // Perform dimensionality reduction to get final encoding vector
    let final_encoding = dimensionality_reduction(&vec![flattened_data], dimension);
    final_encoding[0].clone()
}

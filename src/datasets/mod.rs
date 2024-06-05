use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::hdcmodels::encoding::{encode_natural_language, encode_rust_code};

pub fn load_dataset(block_id: u64) -> Result<(Vec<String>, Vec<String>), DatasetError> {
    let rust_code_snippets = load_rust_code_snippets(block_id)?;
    let natural_language_descriptions = load_natural_language_descriptions(block_id)?;

    if rust_code_snippets.len() != natural_language_descriptions.len() {
        return Err(DatasetError("Number of code snippets and descriptions do not match".to_string()));
    }

    Ok((rust_code_snippets, natural_language_descriptions))
}

fn load_rust_code_snippets(block_id: u64) -> Result<Vec<String>, DatasetError> {
    let file_path = get_dataset_path(block_id, "rust_code_snippets.txt");
    load_text_file(&file_path)
}

fn load_natural_language_descriptions(block_id: u64) -> Result<Vec<String>, DatasetError> {
    let file_path = get_dataset_path(block_id, "natural_language_descriptions.txt");
    load_text_file(&file_path)
}

fn load_text_file(file_path: &str) -> Result<Vec<String>, DatasetError> {
    let file = File::open(file_path).map_err(|e| DatasetError(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);

    let lines: Result<Vec<String>, DatasetError> = reader
        .lines()
        .map(|line| line.map_err(|e| DatasetError(format!("Failed to read line: {}", e))))
        .collect();

    lines
}

fn get_dataset_path(block_id: u64, file_name: &str) -> String {
    format!("datasets/block_{}/{}", block_id, file_name)
}

pub fn preprocess_dataset(
    rust_code_snippets: &[String],
    natural_language_descriptions: &[String],
    dimension: usize,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let processed_rust_code: Vec<Vec<f64>> = rust_code_snippets
        .iter()
        .map(|code| encode_rust_code(code, dimension))
        .collect();

    let processed_descriptions: Vec<Vec<f64>> = natural_language_descriptions
        .iter()
        .map(|desc| encode_natural_language(desc, dimension))
        .collect();

    (processed_rust_code, processed_descriptions)
}
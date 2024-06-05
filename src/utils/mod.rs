use std::fs;
use std::path::Path;

mod fft;
mod compression;
pub(crate) mod encryption;
pub(crate) mod node_id;

pub fn create_directory_if_not_exists(path: &str) -> Result<(), std::io::Error> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn write_to_file(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(file_path, content)
}

pub fn read_from_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn calculate_mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn calculate_standard_deviation(values: &[f64]) -> f64 {
    let mean = calculate_mean(values);
    let variance = values
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance.sqrt()
}

pub fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(&x, &y)| x * y).sum();
    let magnitude1: f64 = vec1.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.iter().map(|&y| y.powi(2)).sum::<f64>().sqrt();
    dot_product / (magnitude1 * magnitude2)
}

pub fn hamming_distance(vec1: &[f64], vec2: &[f64]) -> u64 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(&x, &y)| if x != y { 1 } else { 0 })
        .sum()
}

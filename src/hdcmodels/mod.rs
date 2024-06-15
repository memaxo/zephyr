use crate::hdcmodels::encoding::{encode_rust_code, encode_natural_language};
use crate::hdcmodels::similarity::{cosine_similarity, hamming_distance};

pub mod encoding;
pub mod similarity;
pub mod plugin;
pub mod services;

const HYPERVECTOR_DIMENSION: usize = 5000;

pub struct HDCModel {
    dimension: usize,
    similarity_metric: SimilarityMetric,
}

impl HDCModel {
    pub fn new(similarity_metric: SimilarityMetric) -> Self {
        HDCModel {
            dimension: HYPERVECTOR_DIMENSION,
            similarity_metric,
        }
    }

    pub fn train(&self, rust_code_snippets: &[String], natural_language_descriptions: &[String]) -> Vec<Vec<f64>> {
        let encoded_rust_code: Vec<Vec<f64>> = rust_code_snippets
            .iter()
            .map(|code| encode_rust_code(code, self.dimension))
            .collect();

        let encoded_descriptions: Vec<Vec<f64>> = natural_language_descriptions
            .iter()
            .map(|desc| encode_natural_language(desc, self.dimension))
            .collect();

        // Placeholder for the actual training logic
        // Replace this with the implementation of HDC training algorithm
        encoded_rust_code
    }

    pub fn generate_rust_code(&self, natural_language_query: &str, trained_model: &[Vec<f64>]) -> String {
        let encoded_query = encode_natural_language(natural_language_query, self.dimension);

        let mut max_similarity = f64::NEG_INFINITY;
        let mut best_match_index = 0;

        for (i, code_vector) in trained_model.iter().enumerate() {
            let similarity = match self.similarity_metric {
                SimilarityMetric::CosineSimilarity => cosine_similarity(&encoded_query, code_vector),
                SimilarityMetric::HammingDistance => hamming_distance(&encoded_query, code_vector),
            };

            if similarity > max_similarity {
                max_similarity = similarity;
                best_match_index = i;
            }
        }

        // Placeholder for the actual code generation logic
        // Replace this with the implementation of generating Rust code from the best match
        format!("Generated Rust code for query: {}", natural_language_query)
    }
}

pub enum SimilarityMetric {
    CosineSimilarity,
    HammingDistance,
}

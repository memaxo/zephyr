use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;

pub struct IncrementalLearningHDCModel {
    model: HDCModel,
    learning_rate: f64,
    similarity_threshold: f64,
}

impl IncrementalLearningHDCModel {
    pub fn new(model: HDCModel, learning_rate: f64, similarity_threshold: f64) -> Self {
        IncrementalLearningHDCModel {
            model,
            learning_rate,
            similarity_threshold,
        }
    }

    pub fn update(&mut self, data: &[f64], label: &str) {
        let encoded_data = encode_data(data, self.model.dimension);
        let similarity = self.calculate_similarity(&encoded_data, label);

        if similarity < self.similarity_threshold {
            self.increment(&encoded_data, label);
        }
    }

    fn calculate_similarity(&self, encoded_data: &[f64], label: &str) -> f64 {
        let label_vector = self.model.get_label_vector(label);
        cosine_similarity(encoded_data, &label_vector)
    }

    fn increment(&mut self, encoded_data: &[f64], label: &str) {
        let mut label_vector = self.model.get_label_vector(label);

        for (i, &value) in encoded_data.iter().enumerate() {
            let delta = self.learning_rate * value;
            label_vector[i] += delta;
        }

        self.model.update_label_vector(label, &label_vector);
    }

    pub fn predict(&self, data: &[f64]) -> String {
        let encoded_data = encode_data(data, self.model.dimension);
        let mut max_similarity = f64::NEG_INFINITY;
        let mut best_label = String::new();

        for (label, _) in &self.model.label_vectors {
            let similarity = self.calculate_similarity(&encoded_data, label);
            if similarity > max_similarity {
                max_similarity = similarity;
                best_label = label.clone();
            }
        }

        best_label
    }
}
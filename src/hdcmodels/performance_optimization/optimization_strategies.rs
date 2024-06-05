use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;
use ndarray::{Array, Array1, Array2, Axis};

pub struct OptimizedHDCModel {
    model: HDCModel,
}

impl OptimizedHDCModel {
    pub fn new(model: HDCModel) -> Self {
        OptimizedHDCModel { model }
    }

    pub fn train(&mut self, data: &[Vec<f64>], labels: &[String]) {
        let encoded_data = self.encode_data_batch(data);
        self.model.train(&encoded_data, labels);
    }

    pub fn predict(&self, data: &[f64]) -> String {
        let encoded_data = encode_data(data, self.model.dimension);
        self.model.predict(&encoded_data)
    }

    fn encode_data_batch(&self, data: &[Vec<f64>]) -> Array2<f64> {
        let num_samples = data.len();
        let dimension = self.model.dimension;
        let mut encoded_data = Array2::zeros((num_samples, dimension));

        for (i, sample) in data.iter().enumerate() {
            let encoded_sample = encode_data(sample, dimension);
            encoded_data
                .row_mut(i)
                .assign(&Array1::from_vec(encoded_sample));
        }

        encoded_data
    }
}

pub fn optimize_hyperparameters(model: &mut HDCModel, data: &[Vec<f64>], labels: &[String]) {
    let dimensions = vec![1000, 2000, 5000, 10000];
    let learning_rates = vec![0.01, 0.05, 0.1, 0.2];

    let mut best_accuracy = 0.0;
    let mut best_dimension = model.dimension;
    let mut best_learning_rate = model.learning_rate;

    for &dimension in &dimensions {
        for &learning_rate in &learning_rates {
            model.dimension = dimension;
            model.learning_rate = learning_rate;

            let encoded_data = encode_data_batch(data, dimension);
            model.train(&encoded_data, labels);

            let accuracy = evaluate_accuracy(model, data, labels);
            if accuracy > best_accuracy {
                best_accuracy = accuracy;
                best_dimension = dimension;
                best_learning_rate = learning_rate;
            }
        }
    }

    model.dimension = best_dimension;
    model.learning_rate = best_learning_rate;
}

fn evaluate_accuracy(model: &HDCModel, data: &[Vec<f64>], labels: &[String]) -> f64 {
    let mut correct_predictions = 0;

    for (sample, label) in data.iter().zip(labels) {
        let encoded_sample = encode_data(sample, model.dimension);
        let predicted_label = model.predict(&encoded_sample);
        if predicted_label == *label {
            correct_predictions += 1;
        }
    }

    correct_predictions as f64 / data.len() as f64
}

fn encode_data_batch(data: &[Vec<f64>], dimension: usize) -> Array2<f64> {
    let num_samples = data.len();
    let mut encoded_data = Array2::zeros((num_samples, dimension));

    for (i, sample) in data.iter().enumerate() {
        let encoded_sample = encode_data(sample, dimension);
        encoded_data
            .row_mut(i)
            .assign(&Array1::from_vec(encoded_sample));
    }

    encoded_data
}

use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;
use arrayfire::algorithm::sum;
use arrayfire::arith::mul;
use arrayfire::data::constant;
use arrayfire::{Array, Dim4, MatProp};

pub struct GPUHDCModel {
    model: HDCModel,
}

impl GPUHDCModel {
    pub fn new(model: HDCModel) -> Self {
        GPUHDCModel { model }
    }

    pub fn train(&mut self, data: &[Vec<f64>], labels: &[String]) {
        let encoded_data = self.encode_data_gpu(data);
        let labels_af = Array::new(
            &labels.iter().map(|s| s.as_bytes()).collect::<Vec<_>>(),
            Dim4::new(&[labels.len() as u64, 1, 1, 1]),
        );

        let train_size = data.len() as u64;
        let batch_size = 1000;
        let num_batches = (train_size + batch_size - 1) / batch_size;

        for i in 0..num_batches {
            let start_index = i * batch_size;
            let end_index = std::cmp::min((i + 1) * batch_size, train_size);
            let batch_data = encoded_data.slice(
                Dim4::new(&[start_index, 0, 1, 1]),
                Dim4::new(&[end_index - start_index, self.model.dimension as u64, 1, 1]),
            );
            let batch_labels = labels_af.slice(
                Dim4::new(&[start_index, 0, 1, 1]),
                Dim4::new(&[end_index - start_index, 1, 1, 1]),
            );

            self.model.update_weights_gpu(&batch_data, &batch_labels);
        }
    }

    pub fn predict(&self, data: &[f64]) -> String {
        let encoded_data = self.encode_data_gpu(&[data.to_vec()]);
        let mut max_similarity = f64::NEG_INFINITY;
        let mut best_label = String::new();

        for (label, label_vector) in &self.model.label_vectors {
            let label_vector_af = Array::new(
                label_vector,
                Dim4::new(&[label_vector.len() as u64, 1, 1, 1]),
            );
            let similarity = self.cosine_similarity_gpu(&encoded_data, &label_vector_af);
            if similarity > max_similarity {
                max_similarity = similarity;
                best_label = label.clone();
            }
        }

        best_label
    }

    fn encode_data_gpu(&self, data: &[Vec<f64>]) -> Array<f64> {
        let num_samples = data.len();
        let dimension = self.model.dimension;
        let mut encoded_data_vec = Vec::with_capacity(num_samples * dimension);

        for sample in data {
            let encoded_sample = encode_data(sample, dimension);
            encoded_data_vec.extend_from_slice(&encoded_sample);
        }

        Array::new(
            &encoded_data_vec,
            Dim4::new(&[num_samples as u64, dimension as u64, 1, 1]),
        )
    }

    fn cosine_similarity_gpu(&self, a: &Array<f64>, b: &Array<f64>) -> f64 {
        let dot_product = sum(&mul(a, b), 1).scalar::<f64>();
        let magnitude_a = sum(&mul(a, a), 1).sqrt().scalar::<f64>();
        let magnitude_b = sum(&mul(b, b), 1).sqrt().scalar::<f64>();

        dot_product / (magnitude_a * magnitude_b)
    }
}

impl HDCModel {
    fn update_weights_gpu(&mut self, encoded_data: &Array<f64>, labels: &Array<u8>) {
        let num_samples = encoded_data.dims()[0] as usize;
        let dimension = encoded_data.dims()[1] as usize;

        for (label_bytes, data_row) in labels
            .host::<u8>()
            .chunks_exact(1)
            .zip(encoded_data.host::<f64>().chunks_exact(dimension))
        {
            let label = String::from_utf8_lossy(&label_bytes).to_string();
            let data_vec = data_row.to_vec();

            let entry = self
                .label_vectors
                .entry(label)
                .or_insert_with(|| vec![0.0; dimension]);
            for (i, value) in data_vec.iter().enumerate() {
                entry[i] += self.learning_rate * value;
            }
        }
    }
}

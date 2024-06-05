use crate::hdcmodels::encoding::{encode_natural_language, encode_rust_code};
use crate::hdcmodels::HDCModel;
use ndarray::Array2;
use tch::{Device, Tensor};

pub struct DataConverter;

impl DataConverter {
    pub fn hdc_to_neural_network(hdc_vectors: &[Vec<f64>], output_size: usize) -> Tensor {
        let input_size = hdc_vectors[0].len();
        let num_samples = hdc_vectors.len();

        let mut data = Vec::with_capacity(num_samples * input_size);
        for vector in hdc_vectors {
            data.extend_from_slice(vector);
        }

        let array = Array2::from_shape_vec((num_samples, input_size), data).unwrap();
        Tensor::of_slice(array.as_slice().unwrap())
            .to_device(Device::Cpu)
            .view([-1, input_size as i64, 1])
    }

    pub fn neural_network_to_hdc(tensors: &[Tensor], hdc_dimension: usize) -> Vec<Vec<f64>> {
        let mut hdc_vectors = Vec::with_capacity(tensors.len());

        for tensor in tensors {
            let array = tensor.to_device(Device::Cpu).view([-1]).to_vec();
            let mut hdc_vector = vec![0.0; hdc_dimension];
            for (i, &value) in array.iter().enumerate() {
                if i >= hdc_dimension {
                    break;
                }
                hdc_vector[i] = value;
            }
            hdc_vectors.push(hdc_vector);
        }

        hdc_vectors
    }

    pub fn hdc_to_transformer(hdc_vectors: &[Vec<f64>], max_length: usize) -> Tensor {
        let num_samples = hdc_vectors.len();
        let input_size = hdc_vectors[0].len();

        let mut data = Vec::with_capacity(num_samples * max_length);
        for vector in hdc_vectors {
            let mut padded_vector = vector.clone();
            padded_vector.resize(max_length, 0.0);
            data.extend_from_slice(&padded_vector);
        }

        let array = Array2::from_shape_vec((num_samples, max_length), data).unwrap();
        Tensor::of_slice(array.as_slice().unwrap())
            .to_device(Device::Cpu)
            .view([-1, max_length as i64, 1])
    }

    pub fn transformer_to_hdc(tensors: &[Tensor], hdc_dimension: usize) -> Vec<Vec<f64>> {
        let mut hdc_vectors = Vec::with_capacity(tensors.len());

        for tensor in tensors {
            let array = tensor.to_device(Device::Cpu).view([-1]).to_vec();
            let mut hdc_vector = vec![0.0; hdc_dimension];
            for (i, &value) in array.iter().enumerate() {
                if i >= hdc_dimension {
                    break;
                }
                hdc_vector[i] = value;
            }
            hdc_vectors.push(hdc_vector);
        }

        hdc_vectors
    }

    pub fn rust_code_to_hdc(code: &str, hdc_model: &HDCModel) -> Vec<f64> {
        encode_rust_code(code, hdc_model.dimension)
    }

    pub fn natural_language_to_hdc(text: &str, hdc_model: &HDCModel) -> Vec<f64> {
        encode_natural_language(text, hdc_model.dimension)
    }
}

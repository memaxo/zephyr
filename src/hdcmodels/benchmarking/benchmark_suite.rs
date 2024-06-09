use crate::datasets::load_dataset;
use crate::hdcmodels::encoding::{decode_data, encode_data};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;
use std::time::Instant;

pub struct HDCBenchmarkSuite {
    models: Vec<HDCModel>,
    datasets: Vec<Dataset>,
}

impl HDCBenchmarkSuite {
    pub fn new(models: Vec<HDCModel>, datasets: Vec<Dataset>) -> Self {
        HDCBenchmarkSuite { models, datasets }
    }

    pub fn run(&self) -> Vec<HDCBenchmarkResult> {
        let mut results = Vec::new();

        for model in &self.models {
            for dataset in &self.datasets {
                let result = self.benchmark_model(model, dataset);
                results.push(result);
            }
        }

        results
    }

    fn benchmark_model(&self, model: &HDCModel, dataset: &Dataset) -> HDCBenchmarkResult {
        let start_time = Instant::now();

        let mut accuracy = 0.0;
        let mut encoded_data_size = 0;

        for (data, label) in dataset.iter() {
            let encoded_data = encode_data(data, model.dimension);
            encoded_data_size += encoded_data.len();

            let predicted_label = model.predict(&encoded_data);
            if predicted_label == *label {
                accuracy += 1.0;
            }
        }

        let end_time = Instant::now();
        let execution_time = end_time.duration_since(start_time);

        let accuracy = accuracy / dataset.len() as f64;
        let efficiency = encoded_data_size as f64 / execution_time.as_secs_f64();

        HDCBenchmarkResult {
            model_id: model.id.clone(),
            dataset_id: dataset.id.clone(),
            accuracy,
            execution_time,
            efficiency,
        }
    }
}

pub struct HDCBenchmarkResult {
    pub model_id: String,
    pub dataset_id: String,
    pub accuracy: f64,
    pub execution_time: std::time::Duration,
    pub efficiency: f64,
}

pub struct Dataset {
    pub id: String,
    pub data: Vec<(Vec<f64>, String)>,
}

impl Dataset {
    pub fn load(id: &str) -> Self {
        let data = load_dataset(id);
        Dataset {
            id: id.to_string(),
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<(Vec<f64>, String)> {
        self.data.iter()
    }
}

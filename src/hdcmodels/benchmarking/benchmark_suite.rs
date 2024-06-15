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
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub execution_time: std::time::Duration,
    pub efficiency: f64,
    pub confusion_matrix: HashMap<String, HashMap<String, usize>>,
}

pub struct Dataset {
    pub id: String,
    pub data: Vec<(Vec<f64>, String)>,
}

use plotters::prelude::*;

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

pub struct BenchmarkReport {
    accuracy: f64,
    precision: f64,
    recall: f64,
    f1_score: f64,
    throughput: f64,
    latency: f64,
    memory_usage: f64,
    energy_consumption: f64,
}

impl BenchmarkReport {
    pub fn new() -> Self {
        BenchmarkReport {
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            throughput: 0.0,
            latency: 0.0,
            memory_usage: 0.0,
            energy_consumption: 0.0,
        }
    }

    pub fn generate_report(&self) {
        println!("Accuracy: {}", self.accuracy);
        println!("Precision: {}", self.precision);
        println!("Recall: {}", self.recall);
        println!("F1-Score: {}", self.f1_score);
        println!("Throughput: {}", self.throughput);
        println!("Latency: {}", self.latency);
        println!("Memory Usage: {}", self.memory_usage);
        println!("Energy Consumption: {}", self.energy_consumption);
    }

    pub fn visualize(&self) {
        let root = BitMapBackend::new("benchmark_report.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Benchmark Report", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..8, 0.0..1.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                vec![
                    (0, self.accuracy),
                    (1, self.precision),
                    (2, self.recall),
                    (3, self.f1_score),
                    (4, self.throughput),
                    (5, self.latency),
                    (6, self.memory_usage),
                    (7, self.energy_consumption),
                ],
                &RED,
            ))
            .unwrap()
            .label("Metrics")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw().unwrap();
    }
}

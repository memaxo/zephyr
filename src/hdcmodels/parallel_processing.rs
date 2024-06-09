use crate::hdcmodels::{Dataset, HDCModel, SimilarityMetric};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct ParallelHDCTrainer {
    num_threads: usize,
}

impl ParallelHDCTrainer {
    pub fn new(num_threads: usize) -> Self {
        ParallelHDCTrainer { num_threads }
    }

    pub fn train(&self, model: &mut HDCModel, dataset: &Dataset) -> Vec<Vec<f64>> {
        let partitioned_data = self.partition_data(dataset);

        let trained_models: Vec<Vec<Vec<f64>>> = partitioned_data
            .par_iter()
            .map(|partition| {
                let mut local_model = HDCModel::new(model.similarity_metric);
                local_model.train(partition)
            })
            .collect();

        self.merge_trained_models(trained_models)
    }

    fn partition_data(&self, dataset: &Dataset) -> Vec<Dataset> {
        let mut partitions = Vec::with_capacity(self.num_threads);
        let partition_size = dataset.len() / self.num_threads;

        for i in 0..self.num_threads {
            let start = i * partition_size;
            let end = if i == self.num_threads - 1 {
                dataset.len()
            } else {
                (i + 1) * partition_size
            };

            let partition_data = dataset.slice(start..end).to_vec();
            let partition = Dataset::from_vec(partition_data);
            partitions.push(partition);
        }

        partitions
    }

    fn merge_trained_models(&self, trained_models: Vec<Vec<Vec<f64>>>) -> Vec<Vec<f64>> {
        let merged_model = trained_models
            .into_par_iter()
            .flatten()
            .collect::<Vec<Vec<f64>>>();
        merged_model
    }
}

pub struct ParallelHDCValidator {
    num_threads: usize,
}

impl ParallelHDCValidator {
    pub fn new(num_threads: usize) -> Self {
        ParallelHDCValidator { num_threads }
    }

    pub fn validate(&self, model: &mut HDCModel, dataset: &Dataset, trained_model: &[Vec<f64>]) {
        let partitioned_data = self.partition_data(dataset);

        let validation_results: Vec<(f64, f64)> = partitioned_data
            .par_iter()
            .map(|partition| {
                let mut local_model = HDCModel::new(model.similarity_metric);
                local_model.validate(partition, trained_model);
                (local_model.generalizability, local_model.robustness)
            })
            .collect();

        model.generalizability = validation_results.iter().map(|&(gen, _)| gen).sum::<f64>()
            / validation_results.len() as f64;
        model.robustness = validation_results.iter().map(|&(_, rob)| rob).sum::<f64>()
            / validation_results.len() as f64;
    }

    fn partition_data(&self, dataset: &Dataset) -> Vec<Dataset> {
        let mut partitions = Vec::with_capacity(self.num_threads);
        let partition_size = dataset.len() / self.num_threads;

        for i in 0..self.num_threads {
            let start = i * partition_size;
            let end = if i == self.num_threads - 1 {
                dataset.len()
            } else {
                (i + 1) * partition_size
            };

            let partition_data = dataset.slice(start..end).to_vec();
            let partition = Dataset::from_vec(partition_data);
            partitions.push(partition);
        }

        partitions
    }
}

pub struct ParallelHDCInference {
    num_threads: usize,
}

impl ParallelHDCInference {
    pub fn new(num_threads: usize) -> Self {
        ParallelHDCInference { num_threads }
    }

    pub fn infer(
        &self,
        model: &HDCModel,
        queries: &[String],
        trained_model: &[Vec<f64>],
    ) -> Vec<String> {
        let query_chunks = queries.chunks(queries.len() / self.num_threads);

        let results: Vec<Vec<String>> = query_chunks
            .into_par_iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|query| model.generate_rust_code(query, trained_model))
                    .collect()
            })
            .collect();

        results.into_iter().flatten().collect()
    }
}

trait DatasetExt {
    fn len(&self) -> usize;
    fn slice(&self, range: std::ops::Range<usize>) -> &[DataItem];
    fn to_vec(&self) -> Vec<DataItem>;
    fn from_vec(data: Vec<DataItem>) -> Self;
}

impl DatasetExt for Dataset {
    fn len(&self) -> usize {
        self.items.len()
    }

    fn slice(&self, range: std::ops::Range<usize>) -> &[DataItem] {
        &self.items[range]
    }

    fn to_vec(&self) -> Vec<DataItem> {
        self.items.clone()
    }

    fn from_vec(data: Vec<DataItem>) -> Self {
        Dataset { items: data }
    }
}

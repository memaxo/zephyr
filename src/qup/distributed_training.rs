use crate::hdcmodels::hdcmodels::{HDCModel, Dataset};
use crate::utils::node_id::NodeId;
use crate::qup::fault_tolerance::handle_node_failure;
use crate::monitoring::evaluate_model;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DatasetShard {
    pub data: Vec<f64>,
}

pub struct PartitionedDataset {
    pub shards: Vec<DatasetShard>,
    pub shard_distribution: HashMap<NodeId, Vec<usize>>,
}

impl PartitionedDataset {
    pub fn new(dataset: &Dataset, shard_count: usize, nodes: &[NodeId]) -> Self {
        let chunk_size = (dataset.items.len() + shard_count - 1) / shard_count;
        let mut shards = Vec::new();
        let mut shard_distribution = HashMap::new();

        for i in 0..shard_count {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, dataset.items.len());
            let shard = DatasetShard {
                data: dataset.items[start..end].to_vec(),
            };
            shards.push(shard);
        }

        for (i, node) in nodes.iter().enumerate() {
            shard_distribution.insert(node.clone(), vec![i]);
        }

        PartitionedDataset {
            shards,
            shard_distribution,
        }
    }

    pub fn get_shard(&self, node: &NodeId) -> Option<&[DatasetShard]> {
        self.shard_distribution.get(node).map(|indices| {
            indices.iter().map(|&i| &self.shards[i]).collect::<Vec<_>>()
        })
    }
}

pub struct TrainingResult {
    pub model: HDCModel,
    pub metrics: HashMap<String, f64>,
}

pub struct DistributedTrainer {
    pub nodes: Vec<NodeId>,
    pub partitioned_dataset: PartitionedDataset,
}

impl DistributedTrainer {
    pub fn new(nodes: Vec<NodeId>, dataset: Dataset, shard_count: usize) -> Self {
        let partitioned_dataset = PartitionedDataset::new(&dataset, shard_count, &nodes);
        DistributedTrainer {
            nodes,
            partitioned_dataset,
        }
    }

    pub fn train(&self) -> TrainingResult {
        let mut handles = vec![];

        for node in &self.nodes {
            let dataset_shard = self.partitioned_dataset.get_shard(node).unwrap().to_vec();
            let handle = std::thread::spawn(move || {
                let model = HDCModel::new();
                let trained_model = model.train(&dataset_shard);
                trained_model
            });
            handles.push(handle);
        }

        let mut models = vec![];
        for handle in handles {
            match handle.join() {
                Ok(model) => models.push(model),
                Err(_) => handle_node_failure(),
            }
        }

        let aggregated_model = self.aggregate_models(models);
        let metrics = evaluate_model(&aggregated_model);

        TrainingResult {
            model: aggregated_model,
            metrics,
        }
    }

    fn aggregate_models(&self, models: Vec<HDCModel>) -> HDCModel {
        // Placeholder for model aggregation logic
        models[0].clone()
    }
}
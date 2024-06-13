use crate::hdcmodels::hdcmodels::{HDCModel, Dataset, DatasetShard};
use crate::utils::node_id::NodeId;
use crate::qup::distributed_training::PartitionedDataset;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

impl HDCModel {
    pub fn to_huggingface(&self) -> String {
        // Placeholder for conversion logic to Hugging Face format
        "huggingface_model".to_string()
    }

    pub fn from_huggingface(hf_model: &str) -> Self {
        // Placeholder for conversion logic from Hugging Face format
        HDCModel::new()
    }

    pub fn train_on_shard(&mut self, shard: &DatasetShard) -> Vec<Vec<f64>> {
        let dataset = Dataset {
            items: shard.data.clone(),
        };
        self.train(&dataset)
    }

    pub fn train(&mut self, dataset: &Dataset, shard_index: Option<usize>, partitioned_dataset: Option<&PartitionedDataset>) -> Vec<Vec<f64>> {
        if let Some(index) = shard_index {
            if let Some(partitioned) = partitioned_dataset {
                if let Some(shard) = partitioned.get_shard(&NodeId::new()).and_then(|shards| shards.get(index)) {
                    return self.train_on_shard(shard);
                }
            }
        }
        self.train(dataset)
    }
}

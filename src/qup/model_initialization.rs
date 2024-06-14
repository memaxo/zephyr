use crate::qup::distributed_training::DistributedTrainer;
use crate::datasets::Dataset;

impl DistributedTrainer {
    pub fn initialize_with_validation_data(
        nodes: Vec<NodeId>,
        training_dataset: Dataset,
        validation_dataset: Dataset,
        shard_count: usize,
        data_parallelism: bool,
        model_parallelism: bool,
        pipeline_parallelism: bool,
    ) -> Self {
        let partitioned_training_dataset = PartitionedDataset::new(&training_dataset, shard_count, &nodes);
        let partitioned_validation_dataset = PartitionedDataset::new(&validation_dataset, shard_count, &nodes);

        DistributedTrainer {
            nodes,
            training_dataset: partitioned_training_dataset,
            validation_dataset: partitioned_validation_dataset,
            data_parallelism,
            model_parallelism,
            pipeline_parallelism,
        }
    }
}

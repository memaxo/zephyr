use crate::hdcmodels::distributed_training::distributed_framework::{
    Dataset, DistributedTrainingAggregator,
};
use crate::hdcmodels::HDCModel;
use std::sync::Arc;

pub trait LoadBalancer {
    fn partition_data(&self, dataset: Dataset, num_nodes: usize) -> Vec<Dataset>;
}

pub struct RoundRobinLoadBalancer;

impl LoadBalancer for RoundRobinLoadBalancer {
    fn partition_data(&self, dataset: Dataset, num_nodes: usize) -> Vec<Dataset> {
        let mut partitions = vec![Dataset::new(); num_nodes];
        let mut node_index = 0;

        for data_item in dataset.iter() {
            partitions[node_index].add_item(data_item.clone());
            node_index = (node_index + 1) % num_nodes;
        }

        partitions
    }
}

pub struct DataSizeLoadBalancer;

impl LoadBalancer for DataSizeLoadBalancer {
    fn partition_data(&self, dataset: Dataset, num_nodes: usize) -> Vec<Dataset> {
        let mut partitions = vec![Dataset::new(); num_nodes];
        let mut node_sizes = vec![0; num_nodes];

        for data_item in dataset.iter() {
            let item_size = data_item.size();
            let min_size_index = node_sizes
                .iter()
                .enumerate()
                .min_by_key(|&(_, &size)| size)
                .map(|(index, _)| index)
                .unwrap();

            partitions[min_size_index].add_item(data_item.clone());
            node_sizes[min_size_index] += item_size;
        }

        partitions
    }
}

pub struct ComputeCapacityLoadBalancer {
    node_capacities: Vec<f64>,
}

impl ComputeCapacityLoadBalancer {
    pub fn new(node_capacities: Vec<f64>) -> Self {
        ComputeCapacityLoadBalancer { node_capacities }
    }
}

impl LoadBalancer for ComputeCapacityLoadBalancer {
    fn partition_data(&self, dataset: Dataset, num_nodes: usize) -> Vec<Dataset> {
        let mut partitions = vec![Dataset::new(); num_nodes];
        let mut node_loads = vec![0.0; num_nodes];

        for data_item in dataset.iter() {
            let item_load = data_item.compute_load();
            let min_load_index = node_loads
                .iter()
                .enumerate()
                .min_by(|&(i, &a), &(j, &b)| {
                    let load_ratio_a = a / self.node_capacities[i];
                    let load_ratio_b = b / self.node_capacities[j];
                    load_ratio_a.partial_cmp(&load_ratio_b).unwrap()
                })
                .map(|(index, _)| index)
                .unwrap();

            partitions[min_load_index].add_item(data_item.clone());
            node_loads[min_load_index] += item_load;
        }

        partitions
    }
}

pub struct LoadBalancedDistributedTrainingAggregator<T: LoadBalancer> {
    node: DistributedTrainingAggregator,
    load_balancer: T,
}

impl<T: LoadBalancer> LoadBalancedDistributedTrainingAggregator<T> {
    pub fn new(node: DistributedTrainingAggregator, load_balancer: T) -> Self {
        LoadBalancedDistributedTrainingAggregator {
            node,
            load_balancer,
        }
    }

    pub async fn start_training(&mut self, num_nodes: usize, dataset: Dataset) {
        let partitioned_data = self.load_balancer.partition_data(dataset, num_nodes);
        self.node.start_training(num_nodes, partitioned_data).await;
    }
}

trait DataItem {
    fn size(&self) -> usize;
    fn compute_load(&self) -> f64;
}

// Implement the `DataItem` trait for `String`
impl DataItem for String {
    fn size(&self) -> usize {
        self.len()
    }

    fn compute_load(&self) -> f64 {
        // Calculate the compute load based on the string length and a constant factor
        const LOAD_FACTOR: f64 = 0.1;
        self.len() as f64 * LOAD_FACTOR
    }
}

// Implement the `DataItem` trait for `Vec<f64>`
impl DataItem for Vec<f64> {
    fn size(&self) -> usize {
        self.len() * std::mem::size_of::<f64>()
    }

    fn compute_load(&self) -> f64 {
        // Calculate the compute load based on the vector size and a constant factor
        const LOAD_FACTOR: f64 = 0.5;
        self.len() as f64 * LOAD_FACTOR
    }
}

// Implement the `DataItem` trait for `HDCModel`
impl DataItem for HDCModel {
    fn size(&self) -> usize {
        // Calculate the size of the HDCModel based on its internal representation
        self.calculate_size()
    }

    fn compute_load(&self) -> f64 {
        // Calculate the compute load based on the number of parameters and a constant factor
        const LOAD_FACTOR: f64 = 1.0;
        self.num_parameters() as f64 * LOAD_FACTOR
    }
}

// Implement the `DataItem` trait for a custom data type, e.g., `MyDataType`
struct MyDataType {
    field1: String,
    field2: Vec<f64>,
    field3: HDCModel,
}

impl DataItem for MyDataType {
    fn size(&self) -> usize {
        // Calculate the size of `MyDataType` based on its fields
        self.field1.size() + self.field2.size() + self.field3.size()
    }

    fn compute_load(&self) -> f64 {
        // Calculate the compute load based on the fields and their respective load factors
        const FIELD1_LOAD_FACTOR: f64 = 0.2;
        const FIELD2_LOAD_FACTOR: f64 = 0.3;
        const FIELD3_LOAD_FACTOR: f64 = 0.5;

        self.field1.compute_load() * FIELD1_LOAD_FACTOR
            + self.field2.compute_load() * FIELD2_LOAD_FACTOR
            + self.field3.compute_load() * FIELD3_LOAD_FACTOR
    }
}

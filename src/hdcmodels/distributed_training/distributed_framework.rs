use crate::hdcmodels::{Dataset, HDCModel, SimilarityMetric};
use crate::network::message::Message;
use crate::network::node::{Node, NodeConfig};
use crate::network::quantum_resistant::{
    QuantumResistantConnection, QuantumResistantConnectionManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Serialize, Deserialize)]
pub enum DistributedTrainingMessage {
    InitializeTraining(usize),
    TrainingData(Dataset),
    TrainedModel(Vec<Vec<f64>>),
    AggregatedModel(Vec<Vec<f64>>),
    TrainingComplete,
}

pub struct DistributedTrainingNode {
    node: Node,
    hdc_model: Arc<HDCModel>,
    training_data: Option<Dataset>,
    aggregator_channel: Option<Sender<DistributedTrainingMessage>>,
    trainer_channel: Option<Receiver<DistributedTrainingMessage>>,
}

impl DistributedTrainingNode {
    pub async fn new(
        node_config: NodeConfig,
        hdc_model: HDCModel,
        connection_manager: Arc<QuantumResistantConnectionManager>,
    ) -> Self {
        let node = Node::new(node_config, connection_manager).await;
        DistributedTrainingNode {
            node,
            hdc_model: Arc::new(hdc_model),
            training_data: None,
            aggregator_channel: None,
            trainer_channel: None,
        }
    }

    pub async fn start_training(&mut self, aggregator_address: &str) {
        let (sender, receiver) = mpsc::channel(10);
        self.aggregator_channel = Some(sender);
        self.trainer_channel = Some(receiver);

        self.node.connect(aggregator_address).await;
        self.node.send_message(Message::TrainingRequest).await;

        self.training_loop().await;
    }

    async fn training_loop(&mut self) {
        while let Some(message) = self.trainer_channel.as_mut().unwrap().recv().await {
            match message {
                DistributedTrainingMessage::InitializeTraining(_) => {
                    // Perform any necessary initialization steps
                }
                DistributedTrainingMessage::TrainingData(dataset) => {
                    self.training_data = Some(dataset);
                    self.train_model().await;
                }
                DistributedTrainingMessage::AggregatedModel(model) => {
                    // Update the local model with the aggregated model
                    *self.hdc_model = Arc::new(HDCModel::from_model(model));
                }
                DistributedTrainingMessage::TrainingComplete => {
                    break;
                }
                _ => {}
            }
        }
    }

    async fn train_model(&mut self) {
        if let Some(dataset) = &self.training_data {
            let trained_model = self.hdc_model.train(dataset);
            self.send_trained_model(trained_model).await;
        }
    }

    async fn send_trained_model(&mut self, trained_model: Vec<Vec<f64>>) {
        let message = DistributedTrainingMessage::TrainedModel(trained_model);
        self.node
            .send_message(Message::TrainingUpdate(message))
            .await;
    }
}

pub struct DistributedTrainingAggregator {
    node: Node,
    hdc_model: Arc<HDCModel>,
    num_nodes: usize,
    trained_models: Vec<Vec<Vec<f64>>>,
}

impl DistributedTrainingAggregator {
    pub async fn new(
        node_config: NodeConfig,
        hdc_model: HDCModel,
        connection_manager: Arc<QuantumResistantConnectionManager>,
    ) -> Self {
        let node = Node::new(node_config, connection_manager).await;
        DistributedTrainingAggregator {
            node,
            hdc_model: Arc::new(hdc_model),
            num_nodes: 0,
            trained_models: Vec::new(),
        }
    }

    pub async fn start_training(&mut self, num_nodes: usize, dataset: Dataset) {
        self.num_nodes = num_nodes;
        self.distribute_training_data(dataset).await;
        self.aggregation_loop().await;
    }

    async fn distribute_training_data(&mut self, dataset: Dataset) {
        let chunks = self.partition_data(dataset, self.num_nodes);
        for (node_id, chunk) in chunks.into_iter().enumerate() {
            let message = DistributedTrainingMessage::TrainingData(chunk);
            self.node
                .send_message_to(node_id, Message::TrainingUpdate(message))
                .await;
        }
    }

    async fn aggregation_loop(&mut self) {
        while self.trained_models.len() < self.num_nodes {
            if let Ok(Message::TrainingUpdate(DistributedTrainingMessage::TrainedModel(model))) =
                self.node.receive_message().await
            {
                self.trained_models.push(model);
            }
        }
        self.aggregate_models().await;
    }

    async fn aggregate_models(&mut self) {
        let aggregated_model = self.hdc_model.aggregate_models(&self.trained_models);
        self.broadcast_aggregated_model(aggregated_model).await;
        self.broadcast_training_complete().await;
    }

    async fn broadcast_aggregated_model(&mut self, aggregated_model: Vec<Vec<f64>>) {
        let message = DistributedTrainingMessage::AggregatedModel(aggregated_model);
        self.node
            .broadcast_message(Message::TrainingUpdate(message))
            .await;
    }

    async fn broadcast_training_complete(&mut self) {
        let message = DistributedTrainingMessage::TrainingComplete;
        self.node
            .broadcast_message(Message::TrainingUpdate(message))
            .await;
    }

    fn partition_data(&self, dataset: Dataset, num_partitions: usize) -> Vec<Dataset> {
        let mut partitions = Vec::with_capacity(num_partitions);
        let partition_size = dataset.len() / num_partitions;

        for i in 0..num_partitions {
            let start = i * partition_size;
            let end = if i == num_partitions - 1 {
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

trait HDCModelExt {
    fn from_model(model: Vec<Vec<f64>>) -> Self;
    fn aggregate_models(&self, models: &[Vec<Vec<f64>>]) -> Vec<Vec<f64>>;
}

impl HDCModelExt for HDCModel {
    fn from_model(model: Vec<Vec<f64>>) -> Self {
        HDCModel {
            dimension: model[0].len(),
            similarity_metric: SimilarityMetric::CosineSimilarity,
        }
    }

    fn aggregate_models(&self, models: &[Vec<Vec<f64>>]) -> Vec<Vec<f64>> {
        let num_models = models.len();
        let dimension = models[0][0].len();

        let mut aggregated_model = vec![vec![0.0; dimension]; models[0].len()];

        for model in models {
            for (i, row) in model.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    aggregated_model[i][j] += value;
                }
            }
        }

        for row in &mut aggregated_model {
            for value in row {
                *value /= num_models as f64;
            }
        }

        aggregated_model
    }
}

use crate::hdcmodels::benchmarking::benchmark_suite::{HDCBenchmarkResult, HDCBenchmarkSuite};
use crate::network::message::Message;
use crate::network::peer::Peer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkData {
    pub node_id: String,
    pub results: Vec<HDCBenchmarkResult>,
}

pub struct BenchmarkDataCollector {
    pub benchmark_suite: HDCBenchmarkSuite,
    pub peers: Vec<Peer>,
}

impl BenchmarkDataCollector {
    pub fn new(benchmark_suite: HDCBenchmarkSuite, peers: Vec<Peer>) -> Self {
        BenchmarkDataCollector {
            benchmark_suite,
            peers,
        }
    }

    pub async fn collect_data(&self) -> HashMap<String, Vec<HDCBenchmarkResult>> {
        let mut benchmark_data = HashMap::new();

        // Run benchmarks locally
        let local_results = self.benchmark_suite.run();
        benchmark_data.insert(self.get_node_id(), local_results);

        // Request benchmark data from peers
        for peer in &self.peers {
            let request_message = Message::BenchmarkDataRequest;
            if let Ok(Message::BenchmarkDataResponse(data)) =
                peer.send_request(request_message).await
            {
                benchmark_data.insert(data.node_id, data.results);
            }
        }

        benchmark_data
    }

    pub async fn aggregate_data(
        &self,
        benchmark_data: HashMap<String, Vec<HDCBenchmarkResult>>,
    ) -> HashMap<String, HDCBenchmarkResult> {
        let mut aggregated_data = HashMap::new();

        for (model_id, dataset_id, results) in benchmark_data.values().flatten().map(|result| {
            (
                result.model_id.clone(),
                result.dataset_id.clone(),
                result.clone(),
            )
        }) {
            let key = format!("{}_{}", model_id, dataset_id);
            let entry = aggregated_data
                .entry(key)
                .or_insert_with(|| HDCBenchmarkResult {
                    model_id,
                    dataset_id,
                    accuracy: 0.0,
                    execution_time: std::time::Duration::default(),
                    efficiency: 0.0,
                });

            entry.accuracy += results.accuracy;
            entry.execution_time += results.execution_time;
            entry.efficiency += results.efficiency;
        }

        for entry in aggregated_data.values_mut() {
            let count = benchmark_data.len() as f64;
            entry.accuracy /= count;
            entry.execution_time /= count as u32;
            entry.efficiency /= count;
        }

        aggregated_data
    }

    fn get_node_id(&self) -> String {
        // Replace this with the actual implementation to get the unique node identifier
        "local_node".to_string()
    }
}

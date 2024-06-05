use crate::hdcmodels::benchmarking::analysis::BenchmarkAnalyzer;
use crate::hdcmodels::benchmarking::benchmark_suite::HDCBenchmarkResult;
use crate::network::message::Message;
use crate::network::peer::Peer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkReport {
    pub model_id: String,
    pub dataset_id: String,
    pub accuracy: f64,
    pub execution_time: f64,
    pub efficiency: f64,
}

impl From<&HDCBenchmarkResult> for BenchmarkReport {
    fn from(result: &HDCBenchmarkResult) -> Self {
        BenchmarkReport {
            model_id: result.model_id.clone(),
            dataset_id: result.dataset_id.clone(),
            accuracy: result.accuracy,
            execution_time: result.execution_time.as_secs_f64(),
            efficiency: result.efficiency,
        }
    }
}

pub struct BenchmarkReporter {
    analyzer: BenchmarkAnalyzer,
    peers: Vec<Peer>,
}

impl BenchmarkReporter {
    pub fn new(analyzer: BenchmarkAnalyzer, peers: Vec<Peer>) -> Self {
        BenchmarkReporter { analyzer, peers }
    }

    pub async fn generate_report(&self) -> HashMap<String, BenchmarkReport> {
        let benchmark_data = self.analyzer.data_collector.collect_data().await;
        let aggregated_data = self
            .analyzer
            .data_collector
            .aggregate_data(benchmark_data)
            .await;

        let mut report = HashMap::new();
        for (key, result) in aggregated_data {
            let benchmark_report = BenchmarkReport::from(&result);
            report.insert(key, benchmark_report);
        }

        report
    }

    pub async fn share_report(&self, report: HashMap<String, BenchmarkReport>) {
        let message = Message::BenchmarkReport(report);
        for peer in &self.peers {
            peer.send_message(message.clone()).await;
        }
    }

    pub async fn receive_report(&self) -> Option<HashMap<String, BenchmarkReport>> {
        for peer in &self.peers {
            if let Ok(Message::BenchmarkReport(report)) = peer.receive_message().await {
                return Some(report);
            }
        }
        None
    }
}

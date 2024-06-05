use crate::hdcmodels::benchmarking::benchmark_suite::HDCBenchmarkResult;
use crate::hdcmodels::benchmarking::data_collection::BenchmarkDataCollector;
use crate::visualization::plot::{LinePlot, Plot};
use std::collections::HashMap;

pub struct BenchmarkAnalyzer {
    data_collector: BenchmarkDataCollector,
}

impl BenchmarkAnalyzer {
    pub fn new(data_collector: BenchmarkDataCollector) -> Self {
        BenchmarkAnalyzer { data_collector }
    }

    pub async fn analyze(&self) {
        let benchmark_data = self.data_collector.collect_data().await;
        let aggregated_data = self.data_collector.aggregate_data(benchmark_data).await;

        self.generate_accuracy_report(&aggregated_data);
        self.generate_performance_report(&aggregated_data);
        self.generate_efficiency_report(&aggregated_data);
    }

    fn generate_accuracy_report(&self, aggregated_data: &HashMap<String, HDCBenchmarkResult>) {
        let mut accuracy_data = Vec::new();

        for (key, result) in aggregated_data {
            accuracy_data.push((key.clone(), result.accuracy));
        }

        let accuracy_plot = LinePlot::new(
            "Accuracy Report",
            "Model-Dataset",
            "Accuracy",
            accuracy_data,
        );
        accuracy_plot.display();
    }

    fn generate_performance_report(&self, aggregated_data: &HashMap<String, HDCBenchmarkResult>) {
        let mut performance_data = Vec::new();

        for (key, result) in aggregated_data {
            let execution_time = result.execution_time.as_secs_f64();
            performance_data.push((key.clone(), execution_time));
        }

        let performance_plot = LinePlot::new(
            "Performance Report",
            "Model-Dataset",
            "Execution Time (s)",
            performance_data,
        );
        performance_plot.display();
    }

    fn generate_efficiency_report(&self, aggregated_data: &HashMap<String, HDCBenchmarkResult>) {
        let mut efficiency_data = Vec::new();

        for (key, result) in aggregated_data {
            efficiency_data.push((key.clone(), result.efficiency));
        }

        let efficiency_plot = LinePlot::new(
            "Efficiency Report",
            "Model-Dataset",
            "Efficiency",
            efficiency_data,
        );
        efficiency_plot.display();
    }
}

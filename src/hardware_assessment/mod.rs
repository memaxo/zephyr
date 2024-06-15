pub struct HardwareCapabilities {
    pub cpu_cores: usize,
    pub gpu_cores: Option<usize>,
    pub memory_gb: usize,
    pub storage_gb: usize,
    pub network_bandwidth_mbps: Option<f64>,
}

pub struct BenchmarkResult {
    pub cpu_score: f64,
    pub gpu_score: Option<f64>,
    pub memory_score: f64,
    pub storage_score: f64,
    pub network_latency_ms: Option<f64>,
}

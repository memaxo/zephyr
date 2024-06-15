use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinimumHardwareRequirements {
    pub min_cpu_cores: u8,
    pub min_gpu_memory_mb: u64,
    pub min_ram_mb: u64,
    pub min_storage_gb: u8,
    pub min_network_bandwidth_mbps: u32,
}

impl Default for MinimumHardwareRequirements {
    fn default() -> Self {
        Self {
            min_cpu_cores: 4,
            min_gpu_memory_mb: 2048,
            min_ram_mb: 8192,
            min_storage_gb: 128,
            min_network_bandwidth_mbps: 100,
        }
    }
}

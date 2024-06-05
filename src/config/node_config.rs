use crate::config::feature_flags::FeatureFlags;

pub struct NodeConfig {
    pub id: String,
    pub address: String,
    pub supports_quantum: bool, // Add a field to indicate quantum support
    pub feature_flags: FeatureFlags, // Add feature flags
}
impl NodeConfig {
    pub fn new() -> Self {
        NodeConfig {
            id: String::new(),
            address: String::new(),
            supports_quantum: false,
            feature_flags: FeatureFlags::new(),
        }
    }

    pub fn supports_quantum_features(&self) -> bool {
        self.supports_quantum
    }
}

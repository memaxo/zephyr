pub struct FeatureFlags {
    pub enable_quantum_cryptography: bool,
    pub enable_quantum_key_distribution: bool,
    pub enable_quantum_state_preparation: bool,
    pub enable_quantum_state_verification: bool,
}

impl FeatureFlags {
    pub fn new() -> Self {
        FeatureFlags {
            enable_quantum_cryptography: std::env::var("ENABLE_QUANTUM_CRYPTOGRAPHY")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            enable_quantum_key_distribution: std::env::var("ENABLE_QUANTUM_KEY_DISTRIBUTION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            enable_quantum_state_preparation: std::env::var("ENABLE_QUANTUM_STATE_PREPARATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            enable_quantum_state_verification: std::env::var("ENABLE_QUANTUM_STATE_VERIFICATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}

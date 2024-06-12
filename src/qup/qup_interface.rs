use crate::utils::error::Result;
use crate::qup::crypto::QuantumKeyPair;
use crate::qup::quantum_state::{QuantumState, QuantumStateMeasurement};

pub trait QUPInterface {
    fn generate_quantum_keys(&self) -> Result<QuantumKeyPair>;
    fn distribute_quantum_states(&self, states: Vec<QuantumState>) -> Result<()>;
    fn perform_quantum_computation(&self, computation: &str) -> Result<QuantumStateMeasurement>;
}

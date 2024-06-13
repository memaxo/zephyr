use qiskit::prelude::*;
use qiskit::quantum_info::{Statevector, state_fidelity};
use num_complex::Complex;

pub struct QuantumSimilarity;

impl QuantumSimilarity {
    pub fn quantum_fidelity(state1: &Statevector, state2: &Statevector) -> f64 {
        state_fidelity(state1, state2)
    }

    pub fn quantum_inner_product(state1: &Statevector, state2: &Statevector) -> Complex<f64> {
        state1.inner(state2)
    }
}

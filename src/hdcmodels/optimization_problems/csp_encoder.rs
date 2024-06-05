use crate::hdcmodels::encoding::{encode_natural_language, encode_transactional_data};
use crate::hdcmodels::optimization_problems::csp::{Constraint, Variable, CSP};
use std::collections::HashMap;

pub struct CSPEncoder {
    dimension: usize,
}

impl CSPEncoder {
    pub fn new(dimension: usize) -> Self {
        CSPEncoder { dimension }
    }

    pub fn encode<V: Variable, C: Constraint<V>>(&self, csp: &CSP<V, C>) -> Vec<f64> {
        let mut encoded_csp = Vec::new();

        // Encode the problem description
        let description = format!(
            "CSP with {} variables and {} constraints",
            csp.variables.len(),
            csp.constraints.len()
        );
        let encoded_description = encode_natural_language(&description, self.dimension);
        encoded_csp.extend(encoded_description);

        // Encode the variable domains
        for variable in &csp.variables {
            let domain_data: Vec<u64> = variable
                .get_domain()
                .iter()
                .map(|&value| value as u64)
                .collect();
            let encoded_domain = encode_transactional_data(&domain_data, self.dimension);
            encoded_csp.extend(encoded_domain);
        }

        // Encode the constraints
        for constraint in &csp.constraints {
            let mut constraint_data = Vec::new();
            for i in 0..csp.variables.len() {
                for j in 0..csp.variables.len() {
                    if i != j {
                        let mut assignment = HashMap::new();
                        assignment.insert(i, 0);
                        assignment.insert(j, 0);
                        if !constraint.is_satisfied(&assignment, &csp.variables) {
                            constraint_data.push((i as u64, j as u64));
                        }
                    }
                }
            }
            let encoded_constraint = encode_transactional_data(&constraint_data, self.dimension);
            encoded_csp.extend(encoded_constraint);
        }

        encoded_csp
    }
}

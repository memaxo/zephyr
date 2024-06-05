use crate::hdcmodels::optimization_problems::csp::{Constraint, Variable, CSP};
use crate::hdcmodels::optimization_problems::csp_encoder::CSPEncoder;
use crate::hdcmodels::HDCModel;
use std::collections::HashMap;

pub struct CSPSolver {
    hdc_model: HDCModel,
    encoder: CSPEncoder,
}

impl CSPSolver {
    pub fn new(hdc_model: HDCModel, dimension: usize) -> Self {
        let encoder = CSPEncoder::new(dimension);
        CSPSolver { hdc_model, encoder }
    }

    pub fn solve<V: Variable, C: Constraint<V>>(
        &self,
        csp: &CSP<V, C>,
    ) -> Option<HashMap<usize, usize>> {
        let encoded_csp = self.encoder.encode(csp);
        let output = self.hdc_model.predict(&encoded_csp);

        self.decode_solution(&output, csp)
    }

    fn decode_solution<V: Variable, C: Constraint<V>>(
        &self,
        output: &[f64],
        csp: &CSP<V, C>,
    ) -> Option<HashMap<usize, usize>> {
        let mut assignment = HashMap::new();
        let mut index = 0;

        for variable in &csp.variables {
            let domain_size = variable.get_domain().len();
            let domain_output = &output[index..index + domain_size];

            let mut max_value = 0.0;
            let mut max_index = 0;

            for (i, &value) in domain_output.iter().enumerate() {
                if value > max_value {
                    max_value = value;
                    max_index = i;
                }
            }

            assignment.insert(index / domain_size, variable.get_domain()[max_index]);
            index += domain_size;
        }

        if csp
            .constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(&assignment, &csp.variables))
        {
            Some(assignment)
        } else {
            None
        }
    }
}

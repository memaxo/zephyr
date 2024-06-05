use crate::hdcmodels::encoding::{encode_natural_language, encode_smart_contract};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;

pub struct SmartContractSpecEncoder {
    hdc_model: HDCModel,
}

impl SmartContractSpecEncoder {
    pub fn new(hdc_model: HDCModel) -> Self {
        SmartContractSpecEncoder { hdc_model }
    }

    pub fn encode_specification(&self, specification: &str) -> Vec<f64> {
        encode_natural_language(specification, self.hdc_model.dimension)
    }

    pub fn encode_smart_contract(&self, smart_contract: &str) -> Vec<f64> {
        encode_smart_contract(smart_contract, self.hdc_model.dimension, 3)
    }

    pub fn find_similar_contracts(
        &self,
        specification: &str,
        smart_contracts: &[String],
        top_n: usize,
    ) -> Vec<(String, f64)> {
        let spec_vector = self.encode_specification(specification);
        let mut similarities: Vec<(String, f64)> = smart_contracts
            .iter()
            .map(|contract| {
                let contract_vector = self.encode_smart_contract(contract);
                let similarity = cosine_similarity(&spec_vector, &contract_vector);
                (contract.clone(), similarity)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_n);
        similarities
    }

    pub fn generate_smart_contract(
        &self,
        specification: &str,
        smart_contracts: &[String],
    ) -> String {
        let spec_vector = self.encode_specification(specification);
        let mut max_similarity = f64::NEG_INFINITY;
        let mut best_match = String::new();

        for contract in smart_contracts {
            let contract_vector = self.encode_smart_contract(contract);
            let similarity = cosine_similarity(&spec_vector, &contract_vector);
            if similarity > max_similarity {
                max_similarity = similarity;
                best_match = contract.clone();
            }
        }

        best_match
    }
}

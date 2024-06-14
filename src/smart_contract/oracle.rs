use crate::smart_contract::types::Value;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleRequest {
    pub url: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleResponse {
    pub request_id: u64,
    pub result: Value,
}

pub trait OracleProvider: std::fmt::Debug {
    fn request_data(&self, request: OracleRequest) -> Result<u64, String>;
    fn get_response(&self, request_id: u64) -> Result<OracleResponse, String>;
}

pub struct DecentralizedOracle {
    providers: Vec<Box<dyn OracleProvider>>,
    reputation: HashMap<String, f64>,
}

impl DecentralizedOracle {
    pub fn new(providers: Vec<Box<dyn OracleProvider>>) -> Self {
        DecentralizedOracle {
            providers,
            reputation: HashMap::new(),
        }
    }

    fn update_reputation(&mut self, provider_name: &str, success: bool) {
        let entry = self.reputation.entry(provider_name.to_string()).or_insert(1.0);
        if success {
            *entry += 1.0;
        } else {
            *entry *= 0.9;
        }
    }

    fn aggregate_responses(&self, responses: Vec<OracleResponse>) -> Value {
        // Simple majority voting mechanism for aggregation
        let mut counts = HashMap::new();
        for response in responses {
            *counts.entry(response.result.clone()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|&(_, count)| count).map(|(value, _)| value).unwrap_or(Value::Null)
    }
}

impl OracleProvider for DecentralizedOracle {
    fn request_data(&self, request: OracleRequest) -> Result<u64, String> {
        for provider in &self.providers {
            provider.request_data(request.clone())?;
        }
        Ok(1) // Return a dummy request ID for now
    }

    fn get_response(&self, request_id: u64) -> Result<OracleResponse, String> {
        let mut responses = Vec::new();
        for provider in &self.providers {
            match provider.get_response(request_id) {
                Ok(response) => {
                    self.update_reputation(&format!("{:?}", provider), true);
                    responses.push(response);
                }
                Err(_) => {
                    self.update_reputation(&format!("{:?}", provider), false);
                }
            }
        }
        let aggregated_result = self.aggregate_responses(responses);
        Ok(OracleResponse {
            request_id,
            result: aggregated_result,
        })
    }
}

use crate::smart_contract::types::Value;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleRequest {
    pub url: String,
    pub attributes: Vec<String>,
    pub fn add_provider(&mut self, provider: Box<dyn OracleProvider>) {
        self.providers.push(provider);
    }

    pub fn remove_provider(&mut self, provider_name: &str) -> Result<(), String> {
        let index = self.providers.iter().position(|p| format!("{:?}", p) == provider_name);
        if let Some(index) = index {
            self.providers.remove(index);
            Ok(())
        } else {
            Err(format!("Provider '{}' not found", provider_name))
        }
    }

    pub fn vote_add_provider(&mut self, provider: Box<dyn OracleProvider>, votes: HashMap<String, bool>) -> Result<(), String> {
        let positive_votes = votes.values().filter(|&&v| v).count();
        let negative_votes = votes.values().filter(|&&v| !v).count();

        if positive_votes > negative_votes {
            self.add_provider(provider);
            Ok(())
        } else {
            Err("Provider addition rejected by votes".to_string())
        }
    }

    pub fn vote_remove_provider(&mut self, provider_name: &str, votes: HashMap<String, bool>) -> Result<(), String> {
        let positive_votes = votes.values().filter(|&&v| v).count();
        let negative_votes = votes.values().filter(|&&v| !v).count();

        if positive_votes > negative_votes {
            self.remove_provider(provider_name)
        } else {
            Err("Provider removal rejected by votes".to_string())
        }
    }
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

use reqwest::Client;
use serde::Deserialize;

const QRN_URL: &str = "https://api.quantumnumbers.anu.edu.au/";
const QRN_KEY: &str = "txxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx7"; // replace with your secret API-KEY

#[derive(Debug, Deserialize)]
struct QRNResponse {
    success: bool,
    data: Option<Vec<u8>>,
    message: Option<String>,
}

pub struct QuantumRandom {
    client: Client,
}

impl QuantumRandom {
    pub fn new() -> Self {
        QuantumRandom {
            client: Client::new(),
        }
    }

    pub async fn generate_random_bytes(&self, length: usize) -> Result<Vec<u8>, String> {
        let params = [("length", length.to_string()), ("type", "uint8".to_string())];
        let response = self
            .client
            .get(QRN_URL)
            .header("x-api-key", QRN_KEY)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let qrn_response: QRNResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if qrn_response.success {
            Ok(qrn_response.data.unwrap())
        } else {
            Err(qrn_response.message.unwrap())
        }
    }

    // Add more random number generation methods as needed
}

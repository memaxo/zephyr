use reqwest::blocking::Client;
use std::error::Error;

pub struct QuantumRandom {
    client: Client,
}

impl QuantumRandom {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn generate_random_bytes(&self, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint8", length);
        let response = self.client.get(&url).send()?.json::<QRNGResponse>()?;
        Ok(response.data)
    }
}

#[derive(serde::Deserialize)]
struct QRNGResponse {
    data: Vec<u8>,
    success: bool,
}

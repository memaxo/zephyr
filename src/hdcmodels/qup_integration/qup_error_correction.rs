use crate::qup::error_correction::{QuantumErrorCorrection, QuantumErrorCorrectionScheme};

pub struct QUPErrorCorrection {
    error_correction: QuantumErrorCorrection,
}

impl QUPErrorCorrection {
    pub fn new(scheme: QuantumErrorCorrectionScheme) -> Self {
        let error_correction = QuantumErrorCorrection::new(scheme);
        QUPErrorCorrection { error_correction }
    }

    pub fn encode(&self, data: &[f64]) -> Vec<f64> {
        self.error_correction.encode(data)
    }

    pub fn decode(&self, encoded_data: &[f64]) -> Result<Vec<f64>, QuantumErrorCorrectionError> {
        self.error_correction.decode(encoded_data)
    }

    pub fn correct_errors(
        &self,
        encoded_data: &[f64],
    ) -> Result<Vec<f64>, QuantumErrorCorrectionError> {
        self.error_correction.correct_errors(encoded_data)
    }

    pub fn detect_errors(&self, encoded_data: &[f64]) -> bool {
        self.error_correction.detect_errors(encoded_data)
    }

    pub fn get_error_rate(&self, encoded_data: &[f64]) -> f64 {
        self.error_correction.get_error_rate(encoded_data)
    }

    pub fn set_error_correction_scheme(&mut self, scheme: QuantumErrorCorrectionScheme) {
        self.error_correction.set_scheme(scheme);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QuantumErrorCorrectionError {
    #[error("Error during quantum error correction: {0}")]
    CorrectionError(String),
    #[error("Invalid encoded data")]
    InvalidEncodedData,
}

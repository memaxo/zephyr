use crate::hdcmodels::encoding::{
    encode_natural_language, encode_rust_code, encode_smart_contract, encode_transactional_data,
};
use crate::qup::crypto::{QUPCrypto, QUPSignature};
use crate::Transaction;

pub struct QUPEncoding {
    qup_crypto: QUPCrypto,
}

impl QUPEncoding {
    pub fn new(qup_crypto: QUPCrypto) -> Self {
        QUPEncoding { qup_crypto }
    }

    pub fn encode_rust_code(&self, code: &str, dimension: usize) -> Vec<f64> {
        let encoded_code = encode_rust_code(code, dimension);
        self.post_quantum_encrypt(&encoded_code)
    }

    pub fn encode_natural_language(&self, text: &str, dimension: usize) -> Vec<f64> {
        let encoded_text = encode_natural_language(text, dimension);
        self.post_quantum_encrypt(&encoded_text)
    }

    pub fn encode_transactional_data(&self, data: &[Transaction], dimension: usize) -> Vec<f64> {
        let encoded_data = encode_transactional_data(data, dimension);
        self.post_quantum_encrypt(&encoded_data)
    }

    pub fn encode_smart_contract(&self, contract: &str, dimension: usize, n: usize) -> Vec<f64> {
        let encoded_contract = encode_smart_contract(contract, dimension, n);
        self.post_quantum_encrypt(&encoded_contract)
    }

    fn post_quantum_encrypt(&self, data: &[f64]) -> Vec<f64> {
        let serialized_data = bincode::serialize(data).expect("Failed to serialize data");
        let encrypted_data = self
            .qup_crypto
            .encrypt(&serialized_data)
            .expect("Failed to encrypt data");
        let deserialized_data: Vec<f64> =
            bincode::deserialize(&encrypted_data).expect("Failed to deserialize data");
        deserialized_data
    }

    pub fn sign_encoded_data(&self, encoded_data: &[f64]) -> QUPSignature {
        let serialized_data =
            bincode::serialize(encoded_data).expect("Failed to serialize encoded data");
        self.qup_crypto
            .sign(&serialized_data)
            .expect("Failed to sign encoded data")
    }

    pub fn verify_encoded_data_signature(
        &self,
        encoded_data: &[f64],
        signature: &QUPSignature,
    ) -> bool {
        let serialized_data =
            bincode::serialize(encoded_data).expect("Failed to serialize encoded data");
        self.qup_crypto.verify(&serialized_data, signature)
    }
}

use crate::qup::crypto::QUPCrypto;

pub struct QUPDecoding {
    qup_crypto: QUPCrypto,
}

impl QUPDecoding {
    pub fn new(qup_crypto: QUPCrypto) -> Self {
        QUPDecoding { qup_crypto }
    }

    pub fn decode_rust_code(&self, encoded_code: &[f64]) -> String {
        let decrypted_data = self.post_quantum_decrypt(encoded_code);
        let deserialized_code: String =
            bincode::deserialize(&decrypted_data).expect("Failed to deserialize Rust code");
        deserialized_code
    }

    pub fn decode_natural_language(&self, encoded_text: &[f64]) -> String {
        let decrypted_data = self.post_quantum_decrypt(encoded_text);
        let deserialized_text: String = bincode::deserialize(&decrypted_data)
            .expect("Failed to deserialize natural language text");
        deserialized_text
    }

    pub fn decode_transactional_data(&self, encoded_data: &[f64]) -> Vec<Transaction> {
        let decrypted_data = self.post_quantum_decrypt(encoded_data);
        let deserialized_data: Vec<Transaction> = bincode::deserialize(&decrypted_data)
            .expect("Failed to deserialize transactional data");
        deserialized_data
    }

    pub fn decode_smart_contract(&self, encoded_contract: &[f64]) -> String {
        let decrypted_data = self.post_quantum_decrypt(encoded_contract);
        let deserialized_contract: String =
            bincode::deserialize(&decrypted_data).expect("Failed to deserialize smart contract");
        deserialized_contract
    }

    fn post_quantum_decrypt(&self, encoded_data: &[f64]) -> Vec<u8> {
        let serialized_data =
            bincode::serialize(encoded_data).expect("Failed to serialize encoded data");
        let decrypted_data = self
            .qup_crypto
            .decrypt(&serialized_data)
            .expect("Failed to decrypt encoded data");
        decrypted_data
    }
}

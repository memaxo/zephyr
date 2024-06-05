use hkdf::Hkdf;
use sha3::Sha3_256;

pub struct QuantumKeyDerivation;

impl QuantumKeyDerivation {
    pub fn derive_key(
        input_key_material: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Vec<u8> {
        let hkdf = Hkdf::<Sha3_256>::new(Some(salt), input_key_material);
        let mut output_key = vec![0u8; output_length];
        hkdf.expand(info, &mut output_key)
            .expect("Failed to derive key");
        output_key
    }

    pub fn derive_encryption_key(input_key_material: &[u8], salt: &[u8], info: &[u8]) -> Vec<u8> {
        Self::derive_key(input_key_material, salt, info, 32)
    }

    pub fn derive_mac_key(input_key_material: &[u8], salt: &[u8], info: &[u8]) -> Vec<u8> {
        Self::derive_key(input_key_material, salt, info, 32)
    }

    // Add more key derivation methods for specific purposes as needed
}

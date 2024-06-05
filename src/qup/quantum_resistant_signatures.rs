use crate::qup::crypto::{QUPKeyPair, QUPPublicKey, QUPSecretKey, QUPSignature};
use xmss::{XmssKeyPair, XmssParams, XmssSignature};

pub struct QuantumResistantSignature {
    xmss_params: XmssParams,
}

impl QuantumResistantSignature {
    pub fn new(height: u32) -> Self {
        let xmss_params = XmssParams::new(height).expect("Failed to create XMSS parameters");
        QuantumResistantSignature { xmss_params }
    }

    pub fn generate_key_pair(&self) -> QUPKeyPair {
        let xmss_key_pair =
            XmssKeyPair::generate(&self.xmss_params).expect("Failed to generate XMSS key pair");
        let public_key = QUPPublicKey::from_bytes(&xmss_key_pair.public_key().to_bytes());
        let secret_key = QUPSecretKey::from_bytes(&xmss_key_pair.secret_key().to_bytes());
        QUPKeyPair {
            public_key,
            secret_key,
        }
    }

    pub fn sign(&self, message: &[u8], secret_key: &QUPSecretKey) -> QUPSignature {
        let xmss_secret_key = xmss::SecretKey::from_bytes(&secret_key.to_bytes())
            .expect("Failed to parse XMSS secret key");
        let xmss_signature = xmss_secret_key
            .sign(message)
            .expect("Failed to sign message");
        QUPSignature::from_bytes(&xmss_signature.to_bytes())
    }

    pub fn verify(
        &self,
        message: &[u8],
        signature: &QUPSignature,
        public_key: &QUPPublicKey,
    ) -> bool {
        let xmss_public_key = xmss::PublicKey::from_bytes(&public_key.to_bytes())
            .expect("Failed to parse XMSS public key");
        let xmss_signature = XmssSignature::from_bytes(&signature.to_bytes())
            .expect("Failed to parse XMSS signature");
        xmss_public_key.verify(message, &xmss_signature).is_ok()
    }
}

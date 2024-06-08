use crate::crypto::post_quantum::dilithium::DilithiumKeyPair;
use crate::crypto::post_quantum::kyber::KyberKeyPair;
use crate::crypto::post_quantum::mceliece::McElieceKeyPair;
use crate::crypto::post_quantum::ntru::NTRUKeyPair;
use crate::qup::crypto_common::{Decrypt, Encrypt, Sign, Verify};

pub struct QUPCrypto {
    pub dilithium_keypair: DilithiumKeyPair,
    pub kyber_keypair: KyberKeyPair,
    pub mceliece_keypair: McElieceKeyPair,
    pub ntru_keypair: NTRUKeyPair,
}

impl QUPCrypto {
    pub fn new() -> Self {
        QUPCrypto {
            dilithium_keypair: DilithiumKeyPair::generate(),
            kyber_keypair: KyberKeyPair::generate(),
            mceliece_keypair: McElieceKeyPair::generate(),
            ntru_keypair: NTRUKeyPair::generate(),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Use Kyber for encryption
        self.kyber_keypair.public_key.encrypt(data.as_ref())
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        // Use Kyber for decryption
        self.kyber_keypair.secret_key.decrypt(ciphertext.as_ref())
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        // Use Dilithium for signing
        self.dilithium_keypair.sign(data.as_ref())
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        // Use Dilithium for signature verification
        self.dilithium_keypair.public_key.verify(data.as_ref(), signature.as_ref())
    }
}

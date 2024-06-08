use crate::crypto::post_quantum::dilithium::{DilithiumPublicKey, DilithiumSecretKey};
use crate::crypto::post_quantum::kyber::{KyberPublicKey, KyberSecretKey};
use crate::crypto::post_quantum::mceliece::{McEliecePublicKey, McElieceSecretKey};
use crate::crypto::post_quantum::ntru::{NTRUPublicKey, NTRUSecretKey};
use crate::qup::crypto_common::{Decrypt, Encrypt, KeyPair, Sign, Verify};

pub struct QUPCrypto {
    pub dilithium_keypair: KeyPair<DilithiumPublicKey, DilithiumSecretKey>,
    pub kyber_keypair: KeyPair<KyberPublicKey, KyberSecretKey>,
    pub mceliece_keypair: KeyPair<McEliecePublicKey, McElieceSecretKey>,
    pub ntru_keypair: KeyPair<NTRUPublicKey, NTRUSecretKey>,
}

impl QUPCrypto {
    pub fn new() -> Self {
        QUPCrypto {
            dilithium_keypair: KeyPair {
                public_key: DilithiumPublicKey::generate(),
                secret_key: DilithiumSecretKey::generate(),
            },
            kyber_keypair: KeyPair {
                public_key: KyberPublicKey::generate(),
                secret_key: KyberSecretKey::generate(),
            },
            mceliece_keypair: KeyPair {
                public_key: McEliecePublicKey::generate(),
                secret_key: McElieceSecretKey::generate(),
            },
            ntru_keypair: KeyPair {
                public_key: NTRUPublicKey::generate(),
                secret_key: NTRUSecretKey::generate(),
            },
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
        self.dilithium_keypair.secret_key.sign(data.as_ref())
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        // Use Dilithium for signature verification
        self.dilithium_keypair.public_key.verify(data.as_ref(), signature.as_ref())
    }
}

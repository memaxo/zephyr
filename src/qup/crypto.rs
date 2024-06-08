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
            dilithium_keypair: KeyPair::new(DilithiumPublicKey::generate(), DilithiumSecretKey::generate()),
            kyber_keypair: KeyPair::new(KyberPublicKey::generate(), KyberSecretKey::generate()),
            mceliece_keypair: KeyPair::new(McEliecePublicKey::generate(), McElieceSecretKey::generate()),
            ntru_keypair: KeyPair::new(NTRUPublicKey::generate(), NTRUSecretKey::generate()),
        }
    }

    pub fn encrypt<P: Encrypt>(&self, data: &[u8], public_key: &P) -> Vec<u8> {
        public_key.encrypt(data)
    }

    pub fn decrypt<S: Decrypt>(&self, ciphertext: &[u8], secret_key: &S) -> Vec<u8> {
        secret_key.decrypt(ciphertext)
    }

    pub fn sign<S: Sign>(&self, data: &[u8], secret_key: &S) -> Vec<u8> {
        secret_key.sign(data)
    }

    pub fn verify<P: Verify>(&self, data: &[u8], signature: &[u8], public_key: &P) -> bool {
        public_key.verify(data, signature)
    }
}

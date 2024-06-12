use pqcrypto_kyber::kyber512::*;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey};
use newhope::*;
use rand::rngs::OsRng;
use rand::RngCore;

pub struct QuantumSafeEncryption;

impl QuantumSafeEncryption {
    /// Encrypts the given plaintext using Kyber KEM.
    pub fn encrypt_kyber(plaintext: &[u8], public_key: &PublicKey) -> (Vec<u8>, Ciphertext) {
        let (ciphertext, shared_secret) = encapsulate(public_key);
        let mut encrypted_data = shared_secret.to_vec();
        encrypted_data.extend_from_slice(plaintext);
        (encrypted_data, ciphertext)
    }

    /// Decrypts the given ciphertext using Kyber KEM.
    pub fn decrypt_kyber(ciphertext: &Ciphertext, secret_key: &SecretKey) -> Vec<u8> {
        let shared_secret = decapsulate(ciphertext, secret_key);
        shared_secret.to_vec()
    }

    /// Generates a NewHope keypair.
    pub fn generate_newhope_keypair() -> (Vec<u8>, Vec<u8>) {
        let mut rng = OsRng;
        let (public_key, secret_key) = keygen(&mut rng);
        (public_key.to_vec(), secret_key.to_vec())
    }

    /// Encrypts the given plaintext using NewHope with the provided public key.
    pub fn newhope_encrypt(plaintext: &[u8], public_key: &[u8]) -> Vec<u8> {
        let mut rng = OsRng;
        let public_key = PublicKey::from_slice(public_key).expect("Invalid public key");
        let (ciphertext, shared_secret) = encrypt(&public_key, &mut rng);
        let mut encrypted_data = shared_secret.to_vec();
        encrypted_data.extend_from_slice(plaintext);
        encrypted_data
    }

    /// Decrypts the given ciphertext using NewHope with the provided secret key.
    pub fn newhope_decrypt(ciphertext: &[u8], secret_key: &[u8]) -> Vec<u8> {
        let secret_key = SecretKey::from_slice(secret_key).expect("Invalid secret key");
        let ciphertext = Ciphertext::from_slice(ciphertext).expect("Invalid ciphertext");
        let (shared_secret, decrypted_data) = decrypt(&secret_key, &ciphertext).expect("Decryption failed");
        let mut plaintext = shared_secret.to_vec();
        plaintext.extend_from_slice(&decrypted_data);
        plaintext
    }

    /// Generates a secure random nonce of 12 bytes.
    pub fn generate_nonce() -> Vec<u8> {
        let mut nonce = vec![0u8; 12];
        let mut rng = OsRng;
        rng.fill_bytes(&mut nonce);
        nonce
    }
}

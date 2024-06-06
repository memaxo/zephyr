use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, Key, Nonce,
};
use newhope::*;
use rand::rngs::OsRng;

pub struct QuantumSafeEncryption;

impl QuantumSafeEncryption {
    pub fn encrypt_aes_gcm(plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher.encrypt(nonce, plaintext).expect("Encryption failed")
    }

    pub fn decrypt_aes_gcm(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher
            .decrypt(nonce, ciphertext)
            .expect("Decryption failed")
    }

    pub fn generate_newhope_keypair() -> (Vec<u8>, Vec<u8>) {
        let mut rng = OsRng;
        let (public_key, secret_key) = keygen(&mut rng);
        (public_key.to_vec(), secret_key.to_vec())
    }

    pub fn newhope_encrypt(plaintext: &[u8], public_key: &[u8]) -> Vec<u8> {
        let mut rng = OsRng;
        let public_key = PublicKey::from_slice(public_key).expect("Invalid public key");
        let (ciphertext, shared_secret) = encrypt(&public_key, &mut rng);
        let mut encrypted_data = shared_secret.to_vec();
        encrypted_data.extend_from_slice(plaintext);
        encrypted_data
        ciphertext.to_vec()
    }

    pub fn newhope_decrypt(ciphertext: &[u8], secret_key: &[u8]) -> Vec<u8> {
        let secret_key = SecretKey::from_slice(secret_key).expect("Invalid secret key");
        let ciphertext = Ciphertext::from_slice(ciphertext).expect("Invalid ciphertext");
        let (shared_secret, decrypted_data) = decrypt(&secret_key, &ciphertext).expect("Decryption failed");
        let mut plaintext = shared_secret.to_vec();
        plaintext.extend_from_slice(&decrypted_data);
        plaintext
    }
}

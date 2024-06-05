use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, Key, Nonce,
};
use chacha20poly1305::ChaCha20Poly1305;
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
}

pub type EncryptionKey = [u8; 32];

pub fn generate_key() -> Result<EncryptionKey, EncryptionError> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    Ok(key)
}

pub fn encrypt_aes_256_gcm(
    plaintext: &[u8],
    key: &EncryptionKey,
    nonce: &[u8; 12],
) -> Result<Vec<u8>, EncryptionError> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))
}

pub fn decrypt_aes_256_gcm(
    ciphertext: &[u8],
    key: &EncryptionKey,
    nonce: &[u8; 12],
) -> Result<Vec<u8>, EncryptionError> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
}

pub fn encrypt_chacha20_poly1305(
    plaintext: &[u8],
    key: &EncryptionKey,
    nonce: &[u8; 12],
) -> Result<Vec<u8>, EncryptionError> {
    let key = chacha20poly1305::Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(nonce);
    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))
}

pub fn decrypt_chacha20_poly1305(
    ciphertext: &[u8],
    key: &EncryptionKey,
    nonce: &[u8; 12],
) -> Result<Vec<u8>, EncryptionError> {
    let key = chacha20poly1305::Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(nonce);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
}
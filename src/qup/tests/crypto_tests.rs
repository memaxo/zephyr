use crate::qup::config::QUPConfig;
use crate::qup::crypto::{QUPCrypto, QUPKeyPair};
use rand::RngCore;
use std::sync::Arc;

#[test]
fn test_key_generation() {
    let config = Arc::new(QUPConfig::default());
    let crypto = QUPCrypto::new(config);

    let key_pair = crypto.generate_key_pair();

    // Check if the key pair is valid
    assert!(key_pair.signing_key_pair.is_valid());
    assert!(key_pair.encryption_key_pair.is_valid());
}

#[test]
fn test_signing_and_verification() {
    let config = Arc::new(QUPConfig::default());
    let crypto = QUPCrypto::new(config);

    let key_pair = crypto.generate_key_pair();
    let message = b"Hello, World!";

    let signature = crypto.sign(message, &key_pair.signing_key_pair.secret_key);

    assert!(crypto.verify(message, &signature, &key_pair.signing_key_pair.public_key));
}

#[test]
fn test_encryption_and_decryption() {
    let config = Arc::new(QUPConfig::default());
    let crypto = QUPCrypto::new(config);

    let key_pair = crypto.generate_key_pair();
    let mut rng = rand::thread_rng();
    let mut message = vec![0u8; 32];
    rng.fill_bytes(&mut message);

    let encrypted_message = crypto.encrypt(&message, &key_pair.encryption_key_pair.public_key);
    let decrypted_message =
        crypto.decrypt(&encrypted_message, &key_pair.encryption_key_pair.secret_key);

    assert_eq!(message, decrypted_message);
}

#[test]
fn test_hashing() {
    let config = Arc::new(QUPConfig::default());
    let crypto = QUPCrypto::new(config);

    let message = b"Hello, World!";
    let hash = crypto.hash(message);

    // Check if the hash is valid
    assert!(hash.is_valid());
}

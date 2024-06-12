use crate::crypto::post_quantum::{Dilithium, Keypair, PublicKey, SecretKey, Signature};
use crate::network::quantum_resistant::QuantumResistantConnection;
use crate::qup::quantum_random::QuantumRandom;
use crate::utils::error::QuantumEntropyError;
use log::{debug, error, info, trace};
use secrecy::{ExposeSecret, Secret};
use std::sync::Arc;
use thiserror::Error;
use zeroize::Zeroize;

pub struct QuantumEntropySource {
    dilithium: Arc<Dilithium>,
}

impl QuantumEntropySource {
    pub fn new() -> Self {
        let quantum_random = QuantumRandom::new();
        let dilithium = Arc::new(Dilithium::new(quantum_random));
        Self { dilithium }
    }

    pub fn generate_post_quantum_keypair(&self) -> Result<Keypair, QuantumEntropyError> {
        self.dilithium
            .keypair()
            .map_err(|_| QuantumEntropyError::KeyGenerationError)
    }

    pub fn get_public_key(&self, keypair: &Keypair) -> PublicKey {
        keypair.public.clone()
    }

    pub fn get_secret_key(&self, keypair: &Keypair) -> Secret<SecretKey> {
        Secret::new(keypair.secret.clone())
    }

    pub fn sign(&self, message: &[u8], secret_key: &SecretKey) -> Signature {
        self.dilithium.sign(message, secret_key)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
        self.dilithium.verify(message, signature, public_key)
    }

    pub fn zeroize_secret_key(&self, secret_key: &mut SecretKey) {
        secret_key.zeroize();
    }
}

pub struct QuantumResistantConnectionManager {
    connections: Arc<QuantumResistantConnection>,
}

impl QuantumResistantConnectionManager {
    pub fn new() -> Self {
        let connections = Arc::new(QuantumResistantConnection::new());
        Self { connections }
    }

    pub async fn establish_connection(&self, node_id: &str) -> Result<Keypair, QuantumEntropyError> {
        let connection = self.connections.clone();
        connection.establish(node_id).await
    }

    pub async fn close_connection(&self, node_id: &str) -> Result<(), QuantumEntropyError> {
        let connection = self.connections.clone();
        connection.close(node_id).await
    }
}

pub async fn establish_quantum_resistant_connection(
    manager: &QuantumResistantConnectionManager,
    node_id: &str,
) -> Result<Keypair, QuantumEntropyError> {
    info!("Establishing quantum-resistant connection for node: {}", node_id);
    let keypair = manager.establish_connection(node_id).await?;
    debug!("Quantum-resistant connection established successfully");
    Ok(keypair)
}

pub async fn close_quantum_resistant_connection(
    manager: &QuantumResistantConnectionManager,
    node_id: &str,
) -> Result<(), QuantumEntropyError> {
    info!("Closing quantum-resistant connection for node: {}", node_id);
    manager.close_connection(node_id).await?;
    debug!("Quantum-resistant connection closed successfully");
    Ok(())
}

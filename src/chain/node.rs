use crate::chain::certificate_authority::{CertificateAuthority, CertificateAuthorityClient};
use crate::chain::quantum_entropy::{QuantumEntropyError, QuantumEntropySource};
use crate::crypto::post_quantum::{Keypair, PublicKey, SecretKey};
use crate::network::node_message::{NodeMessage, NodeMessageHandler};
use crate::network::quantum_resistant::{
    QuantumResistantConnection, QuantumResistantConnectionManager,
};
use crate::utils::error::NodeError;
use crate::utils::node_id::NodeId;
use log::{debug, error, info, trace, warn};
use secrecy::{ExposeSecret, Secret};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use x509_certificate::rfc5280::Validity;
use x509_certificate::X509Certificate;
use zeroize::Zeroize;

pub struct NodeConfig {
    pub node_id: NodeId,
    pub has_local_entropy_source: bool,
    pub certificate_validity_period: Validity,
}

pub struct Node {
    id: NodeId,
    quantum_entropy_source: Arc<QuantumEntropySource>,
    post_quantum_keypair: Arc<RwLock<Option<Keypair>>>,
    post_quantum_certificate: Arc<RwLock<Option<X509Certificate>>>,
    message_handler: Arc<NodeMessageHandler>,
    certificate_authority: Arc<CertificateAuthority>,
    connection_manager: Arc<QuantumResistantConnectionManager>,
}

impl Node {
    pub async fn new(
        config: NodeConfig,
        message_handler: Arc<NodeMessageHandler>,
        certificate_authority: Arc<CertificateAuthority>,
        connection_manager: Arc<QuantumResistantConnectionManager>,
    ) -> Result<Self, NodeError> {
        let quantum_entropy_source = Arc::new(QuantumEntropySource::new());
        let post_quantum_keypair = if config.has_local_entropy_source {
            let keypair = quantum_entropy_source
                .generate_post_quantum_keypair()
                .await?;
            Arc::new(RwLock::new(Some(keypair)))
        } else {
            warn!("No local entropy source available. Post-quantum keypair not generated.");
            Arc::new(RwLock::new(None))
        };

        Ok(Self {
            id: config.node_id,
            quantum_entropy_source,
            post_quantum_keypair,
            post_quantum_certificate: Arc::new(RwLock::new(None)),
            message_handler,
            certificate_authority,
            connection_manager,
        })
    }

    pub async fn generate_post_quantum_keypair(&self) -> Result<(), NodeError> {
        let mut keypair = self.post_quantum_keypair.write().await;
        if keypair.is_none() {
            let new_keypair = self
                .quantum_entropy_source
                .generate_post_quantum_keypair()
                .await?;
            *keypair = Some(new_keypair);
            info!("Post-quantum keypair generated for node: {}", self.id);
        }
        Ok(())
    }

    pub async fn get_public_key(&self) -> Result<PublicKey, NodeError> {
        let keypair = self.post_quantum_keypair.read().await;
        keypair
            .as_ref()
            .map(|k| k.public.clone())
            .ok_or(NodeError::MissingPublicKey)
    }

    pub async fn get_secret_key(&self) -> Result<Secret<SecretKey>, NodeError> {
        let keypair = self.post_quantum_keypair.read().await;
        keypair
            .as_ref()
            .map(|k| Secret::new(k.secret.clone()))
            .ok_or(NodeError::MissingSecretKey)
    }

    pub async fn generate_post_quantum_certificate_signing_request(
        &self,
        validity_period: Validity,
    ) -> Result<PostQuantumCertificateSigningRequest, NodeError> {
        let public_key = self.get_public_key().await?;
        let subject = format!("CN={}", self.id);

        Ok(PostQuantumCertificateSigningRequest {
            subject,
            public_key,
            validity_period,
        })
    }

    pub async fn set_post_quantum_certificate(&self, certificate: X509Certificate) {
        let mut cert = self.post_quantum_certificate.write().await;
        *cert = Some(certificate);
    }

    pub async fn request_post_quantum_certificate(
        &self,
        validity_period: Validity,
    ) -> Result<(), NodeError> {
        let csr = self
            .generate_post_quantum_certificate_signing_request(validity_period)
            .await?;
        let certificate = self
            .certificate_authority
            .request_post_quantum_certificate(&csr)
            .await?;
        self.set_post_quantum_certificate(certificate).await;
        debug!(
            "Post-quantum certificate received and set for node: {}",
            self.id
        );
        Ok(())
    }

    pub async fn start_message_handler(&self) {
        let message_handler = self.message_handler.clone();
        let node_id = self.id.clone();
        let connection_manager = self.connection_manager.clone();

        tokio::spawn(async move {
            if let Err(e) = message_handler.start(node_id, connection_manager).await {
                error!(
                    "Failed to start message handler for node {}: {}",
                    node_id, e
                );
            }
        });

        info!("Node message handler started for node: {}", self.id);
    }

    pub async fn handle_message(&self, message: NodeMessage) {
        match message {
            NodeMessage::PostQuantumCertificateRequest(csr) => {
                // Handle post-quantum certificate request
                if let Err(e) = self.handle_post_quantum_certificate_request(csr).await {
                    error!(
                        "Failed to handle post-quantum certificate request: {}",
                        e
                    );
                }
            }
            NodeMessage::PostQuantumCertificateResponse(certificate) => {
                // Handle post-quantum certificate response
                if let Err(e) = self
                    .handle_post_quantum_certificate_response(certificate)
                    .await
                {
                    error!(
                        "Failed to handle post-quantum certificate response: {}",
                        e
                    );
                }
            } // Add more message types as needed
        }
    }

    async fn handle_post_quantum_certificate_request(
        &self,
        csr: PostQuantumCertificateSigningRequest,
    ) -> Result<(), NodeError> {
        // Step 1: Generate or retrieve the post-quantum certificate
        let certificate = self.generate_or_retrieve_post_quantum_certificate()?;

        // Step 2: Send the certificate to the requester
        self.send_certificate(&certificate).await?;

        Ok(())
    }

    async fn handle_post_quantum_certificate_response(
        &self,
        certificate: X509Certificate,
    ) -> Result<(), NodeError> {
        // Step 1: Verify the received certificate
        self.verify_certificate(&certificate)?;

        // Step 2: Store the verified certificate
        self.store_certificate(certificate).await?;

        Ok(())
    }
}

pub struct PostQuantumCertificateSigningRequest {
    pub subject: String,
    pub public_key: PublicKey,
    pub validity_period: Validity,
}
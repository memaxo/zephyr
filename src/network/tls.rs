use crate::chain::node::Node;
use rustls::{Certificate, ClientConfig, ClientConnection, Error as TLSError, PrivateKey, ServerConfig, ServerConnection, Stream};
use std::sync::Arc;
use std::io::{Read, Write};
use tokio::net::TcpStream;
use webpki::DNSNameRef;
use log::{info, warn, error};

pub struct PostQuantumTLSConfig {
    pub certificate: Certificate,
    pub private_key: PrivateKey,
    pub ciphersuites: Vec<&'static rustls::SupportedCipherSuite>,
    pub kx_groups: Vec<&'static rustls::SupportedKxGroup>,
    pub certificate_verifier: Arc<dyn rustls::client::ServerCertVerifier>,
}

impl PostQuantumTLSConfig {
    pub fn new() -> Self {
        Self {
            certificate: Vec::new(),
            private_key: Vec::new(),
            ciphersuites: vec![
                &rustls::ciphersuite::TLS13_AES_256_GCM_SHA384,
                &rustls::ciphersuite::TLS13_CHACHA20_POLY1305_SHA256,
            ],
            kx_groups: vec![
                &rustls::kx_group::X25519,
                &rustls::kx_group::SECP384R1,
            ],
            certificate_verifier: Arc::new(rustls::client::WebPkiVerifier::new(
                rustls::client::RootCertStore::empty(),
                None,
            )),
        }
    }

    pub fn set_certificate(&mut self, certificate: Certificate) {
        self.certificate = certificate;
    }

    pub fn set_private_key(&mut self, private_key: PrivateKey) {
        self.private_key = private_key;
    }

    pub fn set_ciphersuites(&mut self, ciphersuites: Vec<&'static rustls::SupportedCipherSuite>) {
        self.ciphersuites = ciphersuites;
    }

    pub fn set_kx_groups(&mut self, kx_groups: Vec<&'static rustls::SupportedKxGroup>) {
        self.kx_groups = kx_groups;
    }

    pub fn set_certificate_verifier(&mut self, verifier: Arc<dyn rustls::client::ServerCertVerifier>) {
        self.certificate_verifier = verifier;
    }
}

impl PostQuantumTLSConfig {
    pub fn new() -> Self {
        Self {
            certificate: Vec::new(),
            private_key: Vec::new(),
        }
    }

    pub fn set_certificate(&mut self, certificate: Certificate) {
        self.certificate = certificate;
    }

    pub fn set_private_key(&mut self, private_key: PrivateKey) {
        self.private_key = private_key;
    }
}

pub struct PostQuantumTLSConnection {
    connection: ClientConnection,
    stream: StreamOwned<ClientConnection, TcpStream>,
}
impl PostQuantumTLSConnection {
    pub async fn new(stream: TcpStream, config: PostQuantumTLSConfig) -> Result<Self, TLSError> {
        let dns_name = DNSNameRef::try_from_ascii_str("localhost").map_err(|_| TLSError::General("Invalid DNS name".into()))?;
        let mut client_config = ClientConfig::new();
        
        client_config.ciphersuites = config.ciphersuites;
        client_config.kx_groups = config.kx_groups;
        client_config.dangerous().set_certificate_verifier(config.certificate_verifier);
        client_config.set_single_cert(vec![config.certificate], config.private_key)?;

        let connection = ClientConnection::new(Arc::new(client_config), dns_name)?;
        let stream = StreamOwned::new(connection, stream);

        Ok(Self { connection, stream })
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.stream.write_all(data).map_err(|e| {
            error!("Failed to send data: {}", e);
            e.into()
        })
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer).map_err(|e| {
            error!("Failed to receive data: {}", e);
            e.into()
        })?;
        Ok(buffer)
    }
}

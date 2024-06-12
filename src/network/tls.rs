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
    stream: Stream<TcpStream<'a>>,
}
impl PostQuantumTLSConnection {
    pub async fn new(stream: TcpStream) -> Result<Self, TLSError> {
        let node = Node::new(); // Assuming the existence of a `Node` struct with post-quantum keys
        let post_quantum_certificate = node.get_post_quantum_certificate().ok_or(TLSError::General("Post-quantum certificate not found".into()))?;
        let post_quantum_private_key = node.get_post_quantum_private_key().ok_or(TLSError::General("Post-quantum private key not found".into()))?;

        let mut config = ClientConfig::new();
        let mut dangerous_config = config.dangerous();
        dangerous_config.set_certificate_verifier(Arc::new(webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap()));
        dangerous_config.set_single_cert(vec![post_quantum_certificate], post_quantum_private_key.clone())?;

        // Enforce strong cipher suites
        config.ciphersuites = vec![
            &rustls::ciphersuite::TLS13_AES_256_GCM_SHA384,
            &rustls::ciphersuite::TLS13_CHACHA20_POLY1305_SHA256,
        ];

        // Enable Perfect Forward Secrecy (PFS)
        config.kx_groups = vec![&rustls::kx_group::X25519, &rustls::kx_group::SECP384R1];

        let dns_name = DNSNameRef::try_from_ascii_str("localhost").map_err(|_| TLSError::General("Invalid DNS name".into()))?;
        let mut session = ClientConnection::new(Arc::new(config), dns_name)?;
        let mut stream = Stream::new(&mut session, stream);

        match stream.complete_io(rustls::Connection::Client) {
            Ok(_) => {
                info!("TLS connection established with peer");
                Ok(Self { connection: session, stream })
            },
            Err(e) => {
                error!("TLS connection failed: {}", e);
                Err(e)
            },
        }
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

use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::sync::Arc;

use rustls::cipher_suite::{TLS13_AES_128_GCM_SHA256, TLS13_AES_256_GCM_SHA384};
use rustls::pemfile::{certs, pkcs8_private_keys};
use rustls::{ClientConfig, NoClientAuth, ServerConfig};

use crate::chain::error_handling::{handle_error, ErrorSeverity};
use crate::chain::crypto::PostQuantumCertificate;

fn set_common_alpn_protocols(config: &mut ClientConfig) {
    let alpn_protocols = env::var("ALPN_PROTOCOLS").unwrap_or_else(|_| "http/1.1,h2".to_string());
    let alpn_protocols: Vec<Vec<u8>> = alpn_protocols
        .split(',')
        .map(|s| s.as_bytes().to_vec())
        .collect();
    config.alpn_protocols = alpn_protocols;
}

fn set_common_alpn_protocols_server(config: &mut ServerConfig) {
    let alpn_protocols = env::var("ALPN_PROTOCOLS").unwrap_or_else(|_| "http/1.1,h2".to_string());
    let alpn_protocols: Vec<Vec<u8>> = alpn_protocols
        .split(',')
        .map(|s| s.as_bytes().to_vec())
        .collect();
    config.alpn_protocols = alpn_protocols;
}

pub fn load_server_config(cert_path: &str, key_path: &str, post_quantum_cert_path: &str) -> Result<Arc<ServerConfig>, io::Error> {
    let cert_file = &mut BufReader::new(File::open(cert_path)?);
    let key_file = &mut BufReader::new(File::open(key_path)?);
    let post_quantum_cert_file = &mut BufReader::new(File::open(post_quantum_cert_path)?);

    let cert_chain = certs(cert_file).map_err(|e| {
        handle_error(&format!("Invalid cert file: {}", e), ErrorSeverity::Fatal);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid cert file")
    })?;

    let mut keys = pkcs8_private_keys(key_file).map_err(|e| {
        handle_error(&format!("Invalid key file: {}", e), ErrorSeverity::Fatal);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid key file")
    })?;

    let post_quantum_cert = PostQuantumCertificate::from_pem(post_quantum_cert_file).map_err(|e| {
        handle_error(&format!("Invalid post-quantum cert file: {}", e), ErrorSeverity::Fatal);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid post-quantum cert file")
    })?;

    let mut config = ServerConfig::new(NoClientAuth::new());

    if let Some(key) = keys.pop() {
        config.set_single_cert(cert_chain, key).map_err(|e| {
            handle_error(&format!("Invalid private key: {}", e), ErrorSeverity::Fatal);
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid private key")
        })?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No private key found",
        ));
    }

    config.set_post_quantum_cert(post_quantum_cert);

    set_cipher_suites_server(&mut config);
    set_common_alpn_protocols_server(&mut config);

    Ok(Arc::new(config))
}

pub fn load_client_config(cert_path: &str, post_quantum_cert_path: &str) -> Result<Arc<ClientConfig>, io::Error> {
    let cert_file = &mut BufReader::new(File::open(cert_path)?);
    let post_quantum_cert_file = &mut BufReader::new(File::open(post_quantum_cert_path)?);

    let cert_chain = certs(cert_file).map_err(|e| {
        handle_error(&format!("Invalid cert file: {}", e), ErrorSeverity::Fatal);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid cert file")
    })?;

    let post_quantum_cert = PostQuantumCertificate::from_pem(post_quantum_cert_file).map_err(|e| {
        handle_error(&format!("Invalid post-quantum cert file: {}", e), ErrorSeverity::Fatal);
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid post-quantum cert file")
    })?;

    let mut config = ClientConfig::new();
    config.set_single_client_cert(cert_chain)?;
    config.set_post_quantum_cert(post_quantum_cert);

    set_cipher_suites_client(&mut config);
    set_common_alpn_protocols(&mut config);

    Ok(Arc::new(config))
}

fn set_cipher_suites_client(config: &mut ClientConfig) {
    let cipher_suites = env::var("CIPHER_SUITES").unwrap_or_else(|_| {
        "TLS13_AES_128_GCM_SHA256,TLS13_AES_256_GCM_SHA384".to_string()
    });

    let suites: Vec<&'static rustls::SupportedCipherSuite> = cipher_suites
        .split(',')
        .map(|suite| match suite {
            "TLS13_AES_128_GCM_SHA256" => &TLS13_AES_128_GCM_SHA256,
            "TLS13_AES_256_GCM_SHA384" => &TLS13_AES_256_GCM_SHA384,
            _ => panic!("Unsupported cipher suite: {}", suite),
        })
        .collect();

    config.ciphersuites = suites;
}

fn set_cipher_suites_server(config: &mut ServerConfig) {
    let cipher_suites = env::var("CIPHER_SUITES").unwrap_or_else(|_| {
        "TLS13_AES_128_GCM_SHA256,TLS13_AES_256_GCM_SHA384".to_string()
    });

    let suites: Vec<&'static rustls::SupportedCipherSuite> = cipher_suites
        .split(',')
        .map(|suite| match suite {
            "TLS13_AES_128_GCM_SHA256" => &TLS13_AES_128_GCM_SHA256,
            "TLS13_AES_256_GCM_SHA384" => &TLS13_AES_256_GCM_SHA384,
            _ => panic!("Unsupported cipher suite: {}", suite),
        })
        .collect();

    config.ciphersuites = suites;
}
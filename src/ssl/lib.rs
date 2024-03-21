// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use mintaka_error::Result;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

pub struct CertificateManager {}

impl CertificateManager {
    pub fn load(cert_path: &str, key_path: &str) -> Result<ServerConfig> {
        let key_file =
            File::open(&key_path).expect(&format!("Failed to open private key file: {}", key_path));
        let key_file = &mut BufReader::new(key_file);

        let cert_file = File::open(&cert_path)
            .expect(&format!("Failed to open private cert file: {}", cert_path));
        let cert_file = &mut BufReader::new(cert_file);

        let cert_chain = certs(cert_file)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();

        let mut keys = pkcs8_private_keys(key_file).unwrap();

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, PrivateKey(keys.remove(0)))
            .unwrap();

        Ok(config)
    }
}

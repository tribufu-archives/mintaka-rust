// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub listen_ipv4: String,
    pub enable_ipv6: bool,
    pub listen_ipv6: String,
    pub port: u16,
    pub enable_tls: bool,
    pub private_key_file: Option<String>,
    pub certificate_chain_file: Option<String>,
    pub tls_port: u16,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            listen_ipv4: "0.0.0.0".to_string(),
            enable_ipv6: false,
            listen_ipv6: "[::1]".to_string(),
            port: 80,
            enable_tls: false,
            private_key_file: Some("config/key.pem".to_string()),
            certificate_chain_file: Some("config/cert.pem".to_string()),
            tls_port: 443,
        }
    }
}

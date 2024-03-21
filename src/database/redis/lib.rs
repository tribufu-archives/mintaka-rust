// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use redis::Client;

#[derive(Default)]
pub struct RedisClientFactory {
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl RedisClientFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }

    pub async fn create(&self) -> Client {
        let client = Client::open(self.url()).unwrap();
        client
    }
}

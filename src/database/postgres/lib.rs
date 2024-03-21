// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use sqlx::PgPool;

pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    #[cfg(feature = "sea-query")]
    pub fn query_builder(&self) -> sea_query::PostgresQueryBuilder {
        sea_query::PostgresQueryBuilder {}
    }
}

#[derive(Default)]
pub struct PostgresDatabaseFactory {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub schema: String,
}

impl PostgresDatabaseFactory {
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

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = schema.into();
        self
    }

    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.schema
        )
    }

    pub async fn create(&self) -> PostgresDatabase {
        let pool = PgPool::connect(&self.url()).await.unwrap();
        PostgresDatabase { pool }
    }
}

// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use sqlx::MySqlPool;

#[derive(Clone)]
pub struct MySqlDatabase {
    pool: MySqlPool,
}

impl MySqlDatabase {
    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    #[cfg(feature = "sea-query")]
    pub fn query_builder(&self) -> sea_query::MysqlQueryBuilder {
        sea_query::MysqlQueryBuilder {}
    }
}

#[derive(Default)]
pub struct MySqlDatabaseFactory {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub schema: String,
}

impl MySqlDatabaseFactory {
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
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.schema
        )
    }

    pub async fn create(&self) -> MySqlDatabase {
        let pool = MySqlPool::connect(&self.url()).await.unwrap();
        MySqlDatabase { pool }
    }
}

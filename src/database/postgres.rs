// Copyright (c) Tribufu. All Rights Reserved.

use crate::DatabaseFactory;
use async_trait::async_trait;
use sea_query::PostgresQueryBuilder;
use sqlx::PgPool;
use std::env;

pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn query_builder(&self) -> PostgresQueryBuilder {
        PostgresQueryBuilder {}
    }
}

pub struct PostgresDatabaseFactory {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl PostgresDatabaseFactory {
    pub const POSTGRES_HOST: &'static str = "localhost";
    pub const POSTGRES_PORT: u16 = 3306;
    pub const POSTGRES_USER: &'static str = "root";
    pub const POSTGRES_PASSWORD: &'static str = "";
    pub const POSTGRES_DATABASE: &'static str = "";

    pub async fn create(&self) -> PostgresDatabase {
        let pool = PgPool::connect(&self.url()).await.unwrap();
        PostgresDatabase { pool }
    }
}

#[async_trait]
impl DatabaseFactory for PostgresDatabaseFactory {
    fn from_env() -> Self {
        let port = if let Ok(p) = env::var("POSTGRES_PORT") {
            p.parse().unwrap_or(Self::POSTGRES_PORT)
        } else {
            Self::POSTGRES_PORT
        };

        Self {
            host: env::var("POSTGRES_HOST").unwrap_or(Self::POSTGRES_HOST.into()),
            port,
            user: env::var("POSTGRES_USER").unwrap_or(Self::POSTGRES_USER.into()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or(Self::POSTGRES_PASSWORD.into()),
            database: env::var("POSTGRES_DATABASE").unwrap_or(Self::POSTGRES_DATABASE.into()),
        }
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }

    fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    fn database(mut self, database: impl Into<String>) -> Self {
        self.database = database.into();
        self
    }

    fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

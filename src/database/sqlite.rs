// Copyright (c) Tribufu. All Rights Reserved.

use crate::DatabaseFactory;
use async_trait::async_trait;
use sea_query::SqliteQueryBuilder;
use sqlx::SqlitePool;
use std::env;

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub fn query_builder(&self) -> SqliteQueryBuilder {
        SqliteQueryBuilder {}
    }
}

pub struct SqliteDatabaseFactory {
    pub file: String,
}

impl SqliteDatabaseFactory {
    pub const SQLITE_DATABASE: &'static str = "data.db";

    pub async fn create(&self) -> SqliteDatabase {
        let pool = SqlitePool::connect(&self.url()).await.unwrap();
        SqliteDatabase { pool }
    }
}

#[async_trait]
impl DatabaseFactory for SqliteDatabaseFactory {
    fn from_env() -> Self {
        Self {
            file: env::var("SQLITE_DATABASE").unwrap_or(Self::SQLITE_DATABASE.into()),
        }
    }

    fn host(self, _: impl Into<String>) -> Self {
        self
    }

    fn port(self, _: u16) -> Self {
        self
    }

    fn user(self, _: impl Into<String>) -> Self {
        self
    }

    fn password(self, _: impl Into<String>) -> Self {
        self
    }

    fn database(mut self, database: impl Into<String>) -> Self {
        self.file = database.into();
        self
    }

    fn url(&self) -> String {
        format!("sqlite://{}", self.file)
    }
}

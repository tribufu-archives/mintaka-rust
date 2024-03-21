// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use sqlx::SqlitePool;

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    #[cfg(feature = "sea-query")]
    pub fn query_builder(&self) -> sea_query::SqliteQueryBuilder {
        sea_query::SqliteQueryBuilder {}
    }
}

#[derive(Default)]
pub struct SqliteDatabaseFactory {
    pub file: String,
}

impl SqliteDatabaseFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = file.into();
        self
    }

    pub fn url(&self) -> String {
        format!("sqlite://{}", self.file)
    }

    pub async fn create(&self) -> SqliteDatabase {
        let pool = SqlitePool::connect(&self.url()).await.unwrap();
        SqliteDatabase { pool }
    }
}

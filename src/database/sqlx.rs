// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::DatabaseDriver;
use sqlx::{AnyPool, Pool};

pub struct Database {
    driver: DatabaseDriver,
    pool: AnyPool,
}

impl Database {
    pub fn pool(&self) -> &Pool<sqlx::Any> {
        &self.pool
    }

    #[cfg(feature = "sea-query")]
    pub fn query_builder(&self) -> Box<dyn sea_query::QueryBuilder> {
        match self.driver {
            DatabaseDriver::Mysql => Box::new(sea_query::MysqlQueryBuilder {}),
            DatabaseDriver::Postgres => Box::new(sea_query::PostgresQueryBuilder {}),
            DatabaseDriver::Sqlite => Box::new(sea_query::SqliteQueryBuilder {}),
        }
    }

    #[cfg(feature = "sea-query")]
    pub fn schema_builder(&self) -> Box<dyn sea_query::SchemaBuilder> {
        match self.driver {
            DatabaseDriver::Mysql => Box::new(sea_query::MysqlQueryBuilder {}),
            DatabaseDriver::Postgres => Box::new(sea_query::PostgresQueryBuilder {}),
            DatabaseDriver::Sqlite => Box::new(sea_query::SqliteQueryBuilder {}),
        }
    }
}

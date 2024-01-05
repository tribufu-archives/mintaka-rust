// Copyright (c) Tribufu. All Rights Reserved.

use async_trait::async_trait;
use sea_query::{
    MysqlQueryBuilder, PostgresQueryBuilder, QueryBuilder, SchemaBuilder, SqliteQueryBuilder,
};
use sqlx::{AnyPool, Pool};

pub enum DatabaseDriver {
    MySql,
    Postgres,
    Sqlite,
}

pub struct DatabasePool {
    driver: DatabaseDriver,
    pool: AnyPool,
}

impl DatabasePool {
    pub fn pool(&self) -> &Pool<sqlx::Any> {
        &self.pool
    }

    pub fn query_builder(&self) -> Box<dyn QueryBuilder> {
        match self.driver {
            DatabaseDriver::MySql => Box::new(MysqlQueryBuilder {}),
            DatabaseDriver::Postgres => Box::new(PostgresQueryBuilder {}),
            DatabaseDriver::Sqlite => Box::new(SqliteQueryBuilder {}),
        }
    }

    pub fn schema_builder(&self) -> Box<dyn SchemaBuilder> {
        match self.driver {
            DatabaseDriver::MySql => Box::new(MysqlQueryBuilder {}),
            DatabaseDriver::Postgres => Box::new(PostgresQueryBuilder {}),
            DatabaseDriver::Sqlite => Box::new(SqliteQueryBuilder {}),
        }
    }
}

#[async_trait]
pub trait DatabaseFactory {
    fn from_env() -> Self;
    fn host(self, host: impl Into<String>) -> Self;
    fn port(self, port: u16) -> Self;
    fn user(self, user: impl Into<String>) -> Self;
    fn password(self, password: impl Into<String>) -> Self;
    fn database(self, database: impl Into<String>) -> Self;
    fn url(&self) -> String;
}

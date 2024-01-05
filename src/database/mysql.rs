// Copyright (c) Tribufu. All Rights Reserved.

use crate::{DatabaseFactory, StaticIden};
use async_trait::async_trait;
use sea_query::{IntoIden, MysqlQueryBuilder, TableRef};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::env;

#[derive(Clone)]
pub struct MySqlDatabase {
    pool: MySqlPool,
}

impl MySqlDatabase {
    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    pub fn query_builder(&self) -> MysqlQueryBuilder {
        MysqlQueryBuilder {}
    }
}

pub struct MySqlDatabaseFactory {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl MySqlDatabaseFactory {
    pub const MYSQL_HOST: &'static str = "localhost";
    pub const MYSQL_PORT: u16 = 3306;
    pub const MYSQL_USER: &'static str = "root";
    pub const MYSQL_PASSWORD: &'static str = "";
    pub const MYSQL_DATABASE: &'static str = "";

    pub async fn create(&self) -> MySqlDatabase {
        let pool = MySqlPool::connect(&self.url()).await.unwrap();
        MySqlDatabase { pool }
    }
}

#[async_trait]
impl DatabaseFactory for MySqlDatabaseFactory {
    fn from_env() -> Self {
        let port = if let Ok(p) = env::var("MYSQL_PORT") {
            p.parse().unwrap_or(Self::MYSQL_PORT)
        } else {
            Self::MYSQL_PORT
        };

        Self {
            host: env::var("MYSQL_HOST").unwrap_or(Self::MYSQL_HOST.into()),
            port,
            user: env::var("MYSQL_USER").unwrap_or(Self::MYSQL_USER.into()),
            password: env::var("MYSQL_PASSWORD").unwrap_or(Self::MYSQL_PASSWORD.into()),
            database: env::var("MYSQL_DATABASE").unwrap_or(Self::MYSQL_DATABASE.into()),
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
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub trait MysqlTable:
    for<'r> FromRow<'r, MySqlRow> + Send + Sync + Unpin + Serialize + DeserializeOwned
{
    const TABLE_NAME: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(StaticIden(Self::TABLE_NAME).into_iden())
    }
}

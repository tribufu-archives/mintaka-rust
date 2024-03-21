// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseDriver {
    #[default]
    Mysql,
    Postgres,
    Sqlite,
}

impl DatabaseDriver {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mysql" => Some(Self::Mysql),
            "postgres" => Some(Self::Postgres),
            "sqlite" => Some(Self::Sqlite),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub schema: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            driver: DatabaseDriver::Mysql,
            host: "localhost".to_string(),
            port: 3306,
            user: Some("root".to_string()),
            password: Some("".to_string()),
            schema: Some("".to_string()),
        }
    }
}

pub trait FromDatabaseConfig {
    fn from_config(config: &DatabaseConfig) -> Self
    where
        Self: Sized;
}

// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

mod cache_config;
mod database_config;
pub mod sqlx;

pub use cache_config::*;
pub use database_config::*;

#[cfg(feature = "mysql")]
pub mod mysql {
    use crate::{DatabaseConfig, FromDatabaseConfig};
    pub use mintaka_mysql::*;

    impl FromDatabaseConfig for MySqlDatabaseFactory {
        fn from_config(config: &DatabaseConfig) -> Self {
            MySqlDatabaseFactory::new()
                .host(config.host.clone())
                .port(config.port)
                .user(config.user.clone().unwrap())
                .password(config.password.clone().unwrap())
                .schema(config.schema.clone().unwrap())
        }
    }
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use crate::{DatabaseConfig, FromDatabaseConfig};
    pub use mintaka_postgres::*;

    impl FromDatabaseConfig for PostgresDatabaseFactory {
        fn from_config(config: &DatabaseConfig) -> Self {
            PostgresDatabaseFactory::new()
                .host(config.host.clone())
                .port(config.port)
                .user(config.user.clone().unwrap())
                .password(config.password.clone().unwrap())
                .schema(config.schema.clone().unwrap())
        }
    }
}

#[cfg(feature = "redis")]
pub mod redis {
    pub use mintaka_redis::*;
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use crate::{DatabaseConfig, FromDatabaseConfig};
    pub use mintaka_sqlite::*;

    impl FromDatabaseConfig for SqliteDatabaseFactory {
        fn from_config(config: &DatabaseConfig) -> Self {
            SqliteDatabaseFactory::new().file(config.host.clone())
        }
    }
}

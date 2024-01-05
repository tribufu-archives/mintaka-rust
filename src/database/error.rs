// Copyright (c) Tribufu. All Rights Reserved.

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type DatabaseResult<T> = core::result::Result<T, DatabaseError>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum DatabaseError {
    EntityNotFound {
        entity: &'static str,
        id: u64,
    },

    ListLimitOverMax {
        max: u64,
        actual: u64,
    },

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
}

impl core::fmt::Display for DatabaseError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for DatabaseError {}

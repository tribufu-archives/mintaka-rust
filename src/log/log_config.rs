// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::LogLevel;
use mintaka_error::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,
    pub file: Option<String>,
}

impl LogConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            level: LogLevel::from_str(&env::var("LOG_LEVEL")?).unwrap_or_default(),
            file: env::var("LOG_FILE").ok(),
        })
    }
}

// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use mintaka_error::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

pub use mintaka_derive::Config;

pub trait Config: Default + Debug + Clone + Serialize + DeserializeOwned {}

pub struct ConfigManager {}

impl ConfigManager {
    pub fn init<T: Config>(path: &str) -> Result<T> {
        if !fs::metadata(path).is_ok() {
            let config = T::default();
            Self::save(&config, path)?;
            Ok(config)
        } else {
            Self::load(path)
        }
    }

    pub fn save<T: Config>(config: &T, path: &str) -> Result<()> {
        let path = PathBuf::from(path);
        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, mintaka_toml::to_string_pretty(config)?)?;

        Ok(())
    }

    pub fn load<T: Config>(path: &str) -> Result<T> {
        let content = fs::read_to_string(path)?;
        let config = mintaka_toml::from_str(&content)?;
        Ok(config)
    }
}

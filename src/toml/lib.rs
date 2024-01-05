// Copyright (c) Tribufu. All Rights Reserved.

pub use toml::from_str;
pub use toml::to_string;
pub use toml::to_string_pretty;

#[macro_export]
macro_rules! include_toml {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load TOML")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TomlTest {
        version: String,
    }

    #[test]
    fn test_include_toml() {
        let data: TomlTest = include_toml!("test.toml");
        assert!(data.version == "0.0.0".to_owned())
    }
}

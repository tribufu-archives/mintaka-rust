// Copyright (c) Tribufu. All Rights Reserved.

pub use serde_yaml::from_slice;
pub use serde_yaml::from_str;
pub use serde_yaml::to_string;

#[macro_export]
macro_rules! include_yaml {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load YAML")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct YamlTest {
        version: String,
    }

    #[test]
    fn test_include_yaml() {
        let data: YamlTest = include_yaml!("test.yaml");
        assert!(data.version == "0.0.0".to_owned())
    }
}

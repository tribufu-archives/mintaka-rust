// Copyright (c) Tribufu. All Rights Reserved.

pub use ron::de::from_str;
pub use ron::ser::to_string;
pub use ron::ser::to_string_pretty;

#[macro_export]
macro_rules! include_ron {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load RON")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct RonTest {
        version: String,
    }

    #[test]
    fn test_include_ron() {
        let data: RonTest = include_ron!("test.ron");
        assert!(data.version == "0.0.0".to_owned())
    }
}

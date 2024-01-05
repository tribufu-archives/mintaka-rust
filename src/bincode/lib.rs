// Copyright (c) Tribufu. All Rights Reserved.

pub use bincode::deserialize as from_bytes;
pub use bincode::serialize as to_bytes;

#[macro_export]
macro_rules! include_bincode {
    ($path:expr) => {
        $crate::from_bytes(include_bytes!($path)).expect("Failed to load BINCODE")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct BincodeTest {
        version: String,
    }

    #[test]
    fn test_include_bincode() {
        let data: BincodeTest = include_bincode!("test.bin");
        assert!(data.version == "0.0.0".to_owned())
    }
}

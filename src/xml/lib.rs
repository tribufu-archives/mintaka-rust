// Copyright (c) Tribufu. All Rights Reserved.

pub use quick_xml::de::from_str;
pub use quick_xml::se::to_string;

#[macro_export]
macro_rules! include_xml {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load XML")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct XmlTest {
        version: String,
    }

    #[test]
    fn test_include_xml() {
        let data: XmlTest = include_xml!("test.xml");
        assert!(data.version == "0.0.0".to_owned())
    }
}

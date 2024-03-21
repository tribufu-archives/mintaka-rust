// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#[cfg(feature = "bincode")]
pub use mintaka_bincode as bincode;

#[cfg(feature = "json")]
pub use mintaka_json as json;

#[cfg(feature = "ron")]
pub use mintaka_ron as ron;

#[cfg(feature = "toml")]
pub use mintaka_toml as toml;

#[cfg(feature = "xml")]
pub use mintaka_xml as xml;

#[cfg(feature = "yaml")]
pub use mintaka_yaml as yaml;

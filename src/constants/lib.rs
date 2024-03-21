// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub const MINTAKA_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const TARGET_TRIPLE: &'static str = env!("VERGEN_CARGO_TARGET_TRIPLE");
pub const LLVM_VERSION: &'static str = env!("VERGEN_RUSTC_LLVM_VERSION");
pub const RUSTC_VERSION: &'static str = env!("VERGEN_RUSTC_SEMVER");
pub const RUSTC_CHANNEL: &'static str = env!("VERGEN_RUSTC_CHANNEL");
pub const RUSTC_COMMIT: &'static str = env!("VERGEN_RUSTC_COMMIT_HASH");
pub const CARGO_PROFILE: &'static str = env!("VERGEN_CARGO_PROFILE");
pub const BUILD_TIMESTAMP: &'static str = env!("VERGEN_BUILD_TIMESTAMP");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        assert_eq!(MINTAKA_VERSION, env!("CARGO_PKG_VERSION"));
    }
}

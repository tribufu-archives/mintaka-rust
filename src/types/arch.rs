// Copyright (c) Tribufu. All Rights Reserved.

use mintaka_consts::TARGET_TRIPLE;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[repr(u8)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceArch {
    X86_64,
    I686,
    AArch64,
    ArmV7,
    Wasm32,
    ArmV6,
    I386,
    I586,
    Mips64el,
    Mipsel,
    Mips64,
    Mips,
    PowerPC64LE,
    PowerPC64,
    PowerPC,
    Riscv64gc,
    S390x,
    Sparc64,
}

impl DeviceArch {
    /// Returns the current built arch.
    pub fn current() -> Option<Self> {
        Self::parse(TARGET_TRIPLE)
    }

    /// Parse arch from rust target target or browser user agent.
    pub fn parse(target: &str) -> Option<Self> {
        let target = target.to_lowercase();

        if target.contains("x86_64")
            || target.contains("x86-64")
            || target.contains("win64")
            || target.contains("x64")
            || target.contains("amd64")
            || target.contains("wow64")
        {
            Some(Self::X86_64)
        } else if target.contains("i686") || target.contains("x86") || target.contains("x32") {
            Some(Self::I686)
        } else if target.contains("aarch64") {
            Some(Self::AArch64)
        } else if target.contains("armv7") {
            Some(Self::ArmV7)
        } else if target.contains("wasm32") {
            Some(Self::Wasm32)
        } else if target.contains("arm") || target.contains("armv6") {
            Some(Self::ArmV6)
        } else if target.contains("i386") {
            Some(Self::I386)
        } else if target.contains("i586") {
            Some(Self::I586)
        } else if target.contains("mips64el") {
            Some(Self::Mips64el)
        } else if target.contains("mipsel") {
            Some(Self::Mipsel)
        } else if target.contains("mips64-") {
            Some(Self::Mips64)
        } else if target.contains("mips-") {
            Some(Self::Mips)
        } else if target.contains("powerpc64le") {
            Some(Self::PowerPC64LE)
        } else if target.contains("powerpc64") {
            Some(Self::PowerPC64)
        } else if target.contains("powerpc") {
            Some(Self::PowerPC)
        } else if target.contains("riscv64gc") {
            Some(Self::I686)
        } else if target.contains("s390x") {
            Some(Self::S390x)
        } else if target.contains("sparc64") {
            Some(Self::Sparc64)
        } else {
            None
        }
    }

    pub fn is_32bits(&self) -> bool {
        match &self {
            Self::I686
            | Self::ArmV7
            | Self::Wasm32
            | Self::ArmV6
            | Self::I386
            | Self::I586
            | Self::Mipsel
            | Self::Mips
            | Self::PowerPC
            | Self::S390x => true,
            _ => false,
        }
    }

    pub fn is_64bits(&self) -> bool {
        match &self {
            Self::X86_64
            | Self::AArch64
            | Self::Mips64el
            | Self::Mips64
            | Self::PowerPC64
            | Self::Riscv64gc
            | Self::Sparc64 => true,
            _ => false,
        }
    }
}

impl Display for DeviceArch {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::AArch64 => write!(f, "aarch64"),
            Self::ArmV6 => write!(f, "arm"),
            Self::ArmV7 => write!(f, "armv7"),
            Self::I386 => write!(f, "i386"),
            Self::I586 => write!(f, "i586"),
            Self::I686 => write!(f, "i686"),
            Self::Mips => write!(f, "mips"),
            Self::Mips64 => write!(f, "mips64"),
            Self::Mips64el => write!(f, "mips64el"),
            Self::Mipsel => write!(f, "mipsel"),
            Self::PowerPC => write!(f, "powerpc"),
            Self::PowerPC64 => write!(f, "powerpc64"),
            Self::PowerPC64LE => write!(f, "powerpc64le"),
            Self::Riscv64gc => write!(f, "riscv64gc"),
            Self::S390x => write!(f, "s390x"),
            Self::Sparc64 => write!(f, "sparc64"),
            Self::Wasm32 => write!(f, "wasm32"),
            Self::X86_64 => write!(f, "x86_64"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_64bits() {
        assert!(DeviceArch::X86_64.is_64bits());
    }

    #[test]
    fn is_32bits() {
        assert!(DeviceArch::I386.is_32bits());
    }
}

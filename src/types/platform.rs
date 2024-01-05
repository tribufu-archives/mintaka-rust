// Copyright (c) Tribufu. All Rights Reserved.

use mintaka_consts::TARGET_TRIPLE;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result};

#[repr(u8)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DevicePlatform {
    Windows,
    Linux,
    Mac,
    Android,
    IOS,
    UWP,
    Web,
    XboxOne,
    XboxSeries,
    PS4,
    PS5,
    Switch,
    TVOS,
    FreeBSD,
    NetBSD,
    OpenBSD,
    Fuchsia,
    Redox,
}

impl DevicePlatform {
    /// Returns the current built platform.
    pub fn current() -> Option<Self> {
        Self::parse(TARGET_TRIPLE)
    }

    /// Parse platform from rust target triple or browser user agent.
    pub fn parse(target: &str) -> Option<Self> {
        let target = target.to_lowercase();

        if target.contains("uwp") {
            Some(Self::UWP)
        } else if target.contains("android") {
            Some(Self::Android)
        } else if target.contains("ios") || target.contains("like mac") {
            Some(Self::IOS)
        } else if target.contains("linux") {
            Some(Self::Linux)
        } else if target.contains("darwin") || target.contains("mac") {
            Some(Self::Mac)
        } else if target.contains("windows") || target.contains("win") {
            Some(Self::Windows)
        } else if target.contains("wasm32") {
            Some(Self::Web)
        } else if target.contains("xboxone") {
            Some(Self::XboxOne)
        } else if target.contains("xboxseries") {
            Some(Self::XboxSeries)
        } else if target.contains("ps4") {
            Some(Self::PS4)
        } else if target.contains("ps5") {
            Some(Self::PS5)
        } else if target.contains("switch") {
            Some(Self::Switch)
        } else if target.contains("tvos") {
            Some(Self::TVOS)
        } else if target.contains("freebsd") {
            Some(Self::FreeBSD)
        } else if target.contains("openbsd") {
            Some(Self::OpenBSD)
        } else if target.contains("netbsd") {
            Some(Self::NetBSD)
        } else if target.contains("fuchsia") {
            Some(Self::Fuchsia)
        } else if target.contains("redox") {
            Some(Self::Redox)
        } else {
            None
        }
    }

    /// Return true if is a desktop platform, otherwise return false.
    pub fn is_desktop(&self) -> bool {
        match &self {
            Self::Windows
            | Self::Linux
            | Self::Mac
            | Self::FreeBSD
            | Self::NetBSD
            | Self::OpenBSD
            | Self::Redox => true,
            _ => false,
        }
    }

    /// Return true if is a mobile platform, otherwise return false.
    pub fn is_mobile(&self) -> bool {
        match &self {
            Self::Android | Self::IOS => true,
            _ => false,
        }
    }

    /// Return true if is a console platform, otherwise return false.
    pub fn is_console(&self) -> bool {
        match &self {
            Self::XboxOne | Self::XboxSeries | Self::PS4 | Self::PS5 | Self::Switch => true,
            _ => false,
        }
    }

    /// Return true if is a apple platform, otherwise return false.
    pub fn is_apple(&self) -> bool {
        match &self {
            Self::Mac | Self::IOS => true,
            _ => false,
        }
    }

    /// Return true if is a microsoft platform, otherwise return false.
    pub fn is_microsoft(&self) -> bool {
        match &self {
            Self::Windows | Self::UWP | Self::XboxOne | Self::XboxSeries => true,
            _ => false,
        }
    }

    /// Return true if is a sony platform, otherwise return false.
    pub fn is_sony(&self) -> bool {
        match &self {
            Self::PS4 | Self::PS5 => true,
            _ => false,
        }
    }

    /// Return true if is a unix-like platform, otherwise return false.
    pub fn is_unix(&self) -> bool {
        match &self {
            Self::Linux
            | Self::Android
            | Self::FreeBSD
            | Self::NetBSD
            | Self::OpenBSD
            | Self::Fuchsia
            | Self::Redox => true,
            _ => false,
        }
    }

    /// Return true if is a bsd platform, otherwise return false.
    pub fn is_bsd(&self) -> bool {
        match &self {
            Self::FreeBSD | Self::NetBSD | Self::OpenBSD => true,
            _ => false,
        }
    }

    /// Return true if suports dynamic loading libraries, otherwise return false.
    pub fn supports_dynamic_loading(&self) -> bool {
        match &self {
            Self::Windows | Self::Linux | Self::Mac | Self::Android | Self::UWP | Self::FreeBSD => {
                true
            }
            _ => false,
        }
    }
}

impl Display for DevicePlatform {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_mobile() {
        assert!(DevicePlatform::Android.is_mobile());
    }

    #[test]
    fn is_console() {
        assert!(DevicePlatform::XboxOne.is_console());
    }

    #[test]
    fn is_apple() {
        assert!(DevicePlatform::Mac.is_apple());
    }

    #[test]
    fn is_microsoft() {
        assert!(DevicePlatform::Windows.is_microsoft());
    }

    #[test]
    fn is_sony() {
        assert!(DevicePlatform::PS4.is_sony());
    }

    #[test]
    fn is_unix() {
        assert!(DevicePlatform::Linux.is_unix());
    }

    #[test]
    fn is_bsd() {
        assert!(DevicePlatform::FreeBSD.is_bsd());
    }

    #[test]
    fn supports_dynamic_loading() {
        assert!(DevicePlatform::Windows.supports_dynamic_loading());
    }
}

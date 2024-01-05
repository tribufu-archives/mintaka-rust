// Copyright (c) Tribufu. All Rights Reserved.

#![allow(non_snake_case)]

use chrono::prelude::*;
pub use env_logger::fmt::Color;
use env_logger::{Builder, Target};
pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;
use log::Level;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod colors;

const MINTAKA_ENV_FILTER: &'static str = "MINTAKA_LOG";

pub fn init() {
    let logger = Logger::from_env(None);
    logger.init();
}

pub fn init_level(level: LogLevel) {
    let logger = Logger::new(level.into());
    logger.init();
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

pub struct Logger {
    builder: Builder,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        let mut builder = Builder::new();
        builder.filter_level(level);
        Self { builder }
    }

    pub fn from_env(var: Option<String>) -> Self {
        let builder = Builder::from_env(var.unwrap_or(MINTAKA_ENV_FILTER.to_string()));
        Self { builder }
    }

    pub fn init(mut self) {
        self.builder
            .target(Target::Stdout)
            .format(|fmt, record| {
                let mut style = fmt.style();

                match record.level() {
                    Level::Error => style.set_color(Color::Ansi256(colors::RED)),
                    Level::Warn => style.set_color(Color::Ansi256(colors::YELLOW)),
                    Level::Info => style.set_color(Color::Ansi256(colors::GREEN)),
                    Level::Debug => style.set_color(Color::Ansi256(colors::WHITE)),
                    Level::Trace => style.set_color(Color::Ansi256(colors::BRIGHT_BLACK)),
                };

                let line = format!(
                    "[{}] [{}]: {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.level(),
                    record.args()
                );

                writeln!(fmt, "{}", style.value(line))
            })
            .init();
    }
}

//! Module for logging with the `log` crate.

use crate::{Log, LogLevel};
use log::{LevelFilter, Metadata, Record, SetLoggerError};

static LOGGER: Logger = Logger;

/// Set the global logger for the `log` crate.
/// # Errors
/// This function will return an error if the logger can't be set.
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            Log::from(record).output();
        }
    }

    fn flush(&self) {
    }
}

#[cfg(feature = "log")]
impl From<&log::Record<'_>> for Log {
    fn from(record: &log::Record) -> Self {
        Self {
            level: record.level().into(),
            message: record.args().to_string(),
            location: None,
            hint: None,
        }
    }
}

#[cfg(feature = "log")]
impl From<log::Level> for LogLevel {
    fn from(val: log::Level) -> Self {
        match val {
            log::Level::Trace => LogLevel::Trace,
            log::Level::Debug => LogLevel::Debug,
            log::Level::Info => LogLevel::Info,
            log::Level::Warn => LogLevel::Warning,
            log::Level::Error => LogLevel::Error,
        }
    }
}

use crate::{Location, Log, LogLevel};
use std::{
    panic::{self, PanicHookInfo},
    process,
};

/// Sets a panic hook that will print a fatal log on panic.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        Log::from(panic_info).output();
        process::exit(1);
    }));
}

impl From<&PanicHookInfo<'_>> for Log {
    fn from(info: &PanicHookInfo) -> Self {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            info.to_string()
        };

        let location = info.location().map(Location::from);

        Log {
            message,
            level: LogLevel::Fatal,
            location,
            hint: None,
        }
    }
}

impl From<&panic::Location<'_>> for Location {
    #[allow(clippy::range_minus_one, reason = "`Location` uses RangeInclusive")]
    fn from(location: &panic::Location) -> Self {
        let path = location.file();

        Location::from_path(path, (location.line() as usize - 1)..=(location.line() as usize - 1)).unwrap_or(Location {
            path: Some(path.into()),
            text: String::new(),
            lines: (location.line() as usize - 1)..=(location.line() as usize - 1),
            section: None,
        })
    }
}

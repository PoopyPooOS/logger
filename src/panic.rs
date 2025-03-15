use crate::{Location, Log, LogLevel, location::Section};
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

        let location = if cfg!(debug_assertions) {
            info.location().map(Location::from)
        } else {
            None
        };

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

        let section =
            Section((location.line() as usize - 1, 0)..=(location.line() as usize - 1, usize::MAX));

        let location = Location::from_path(path);
        match location {
            Ok(location) => {
                #[cfg(debug_assertions)]
                return location.section(section);

                #[cfg(not(debug_assertions))]
                return location;
            }
            Err(_) => Location {
                path: Some(path.into()),
                text: String::new(),
                #[cfg(debug_assertions)]
                section: Some(section),
                #[cfg(not(debug_assertions))]
                section: None,
            },
        }
    }
}

#![feature(let_chains, macro_metavar_expr, concat_idents)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

pub mod helpers;
pub mod level;
pub mod location;
pub mod panic;
pub mod utils; // Sharing is caring

#[cfg(feature = "log")]
mod log_impl;
#[cfg(feature = "log")]
pub use log_impl::{init, Logger};

pub use colored::{Color, Colorize};
pub use level::LogLevel;
pub use location::Location;
use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
};

pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub location: Option<Location>,
    pub hint: Option<String>,
}

impl Log {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            location: None,
            hint: None,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn output(self) {
        print!("{self}");
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let last_line_string = if let Some(ref location) = self.location {
            (location.lines.end() + 1).to_string()
        } else {
            String::new()
        };

        let padding_size = last_line_string.len() + 1;
        let padding = " ".repeat(padding_size);

        let app_name = env::var("LOGGER_APP_NAME").unwrap_or_default();

        // Log level and message
        writeln!(
            f,
            "{}{}",
            if app_name.is_empty() {
                self.level.to_string().color(self.level).bold()
            } else {
                format!("{}[{}]", self.level.to_string().color(self.level), app_name).bold()
            },
            format!(": {}", self.message).bold()
        )?;

        // Location
        if let Some(location) = &self.location {
            writeln!(f, "{}{} {}", &padding[1..], "-->".blue().bold(), location)?;

            // Source
            writeln!(f, "{}{}", padding, "|".blue().bold())?;

            let source = utils::remove_excess_tabs(&location.text);
            highlight_source(f, source, location, &padding, self.level)?;
        }

        // Hint
        if let Some(hint) = &self.hint {
            if self.location.is_some() {
                writeln!(f, "{}{}", padding, "|".blue().bold())?;
            }

            writeln!(f, "{}{} {} {}", padding, "|".blue().bold(), "help:".bold(), hint)?;
        } else if self.location.is_some() {
            writeln!(f, "{}{}", padding, "|".blue().bold())?;
        }

        Ok(())
    }
}

impl Debug for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

/// # Errors
/// This function will propagate errors from `write! and writeln!`
pub fn highlight_source<S: Into<String>>(
    f: &mut Formatter<'_>,
    source: S,
    location: &Location,
    mut padding: &str,
    level: LogLevel,
) -> fmt::Result {
    let source: String = source.into();

    for (idx, line) in source.lines().enumerate() {
        if !utils::range_contains(&location.lines, idx) {
            continue;
        }

        let line_number = (idx + 1).to_string().blue().bold();
        let line_number_len = line_number.len();

        if padding.len() - line_number_len == 1 {
            padding = &padding[line_number_len - 1..];
        }

        write!(f, "{line_number}{}{} ", &padding[1..], "|".blue().bold())?;

        if let Some(section) = &location.section {
            writeln!(
                f,
                "{}",
                match level {
                    LogLevel::Info | LogLevel::Debug | LogLevel::Trace => utils::bold_highlight(line, section),
                    _ => utils::highlight(line, section, level.into()),
                }
            )?;
        } else {
            writeln!(f, "{}", line.color(level).bold())?;
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! set_app_name {
    () => {
        std::env::set_var("LOGGER_APP_NAME", env!("CARGO_PKG_NAME"))
    };
    ($name:expr) => {
        std::env::set_var("LOGGER_APP_NAME", $name)
    };
}

#[macro_export]
macro_rules! unset_app_name {
    () => {
        std::env::remove_var("LOGGER_APP_NAME")
    };
}

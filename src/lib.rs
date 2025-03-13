#![feature(let_chains, macro_metavar_expr, concat_idents)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

pub mod helpers;
pub mod level;
pub mod location;
pub mod panic;
pub mod utils; // Sharing is caring

#[cfg(feature = "log")]
mod log_impl;
use location::Section;
#[cfg(feature = "log")]
pub use log_impl::{Logger, init};

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
        let last_line_string = if let Some(ref location) = self.location
            && let Some(ref section) = location.section
        {
            (section.lines().end() + 1).to_string()
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

            if location.section.is_some() {
                // Source
                writeln!(f, "{}{}", padding, "|".blue().bold())?;

                let source = utils::remove_excess_tabs(&location.text);
                highlight_source(f, source, location, &padding, self.level)?;
            }
        }

        // Hint
        if let Some(hint) = &self.hint {
            if self
                .location
                .as_ref()
                .is_some_and(|location| location.section.is_some())
            {
                writeln!(f, "{}{}", padding, "|".blue().bold())?;
            }

            writeln!(
                f,
                "{}{} {} {}",
                padding,
                "|".blue().bold(),
                "help:".bold(),
                hint
            )?;
        } else if self
            .location
            .as_ref()
            .is_some_and(|location| location.section.is_some())
        {
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
    let section = &location.section.clone().unwrap_or(Section::full());
    let highlighted = match level {
        LogLevel::Trace | LogLevel::Debug | LogLevel::Info => {
            utils::bold_highlight(source, section)
        }
        _ => utils::highlight(source, section, level.into()),
    };

    for (idx, line) in highlighted.lines().enumerate() {
        if !utils::range_contains(&section.lines(), idx) {
            continue;
        }

        let line_number = (idx + 1).to_string().blue().bold();
        let line_number_len = line_number.len();

        if padding.len() - line_number_len == 1 {
            padding = &padding[line_number_len - 1..];
        }

        write!(f, "{line_number}{}{} ", &padding[1..], "|".blue().bold())?;
        writeln!(f, "{line}")?;
    }

    Ok(())
}

/// SAFETY: See <https://doc.rust-lang.org/std/env/fn.set_var.html#safety> to know when this is safe to use.
#[macro_export]
macro_rules! set_app_name {
    () => {
        unsafe { std::env::set_var("LOGGER_APP_NAME", env!("CARGO_PKG_NAME")) }
    };
    ($name:expr) => {
        unsafe { std::env::set_var("LOGGER_APP_NAME", $name) }
    };
}

#[macro_export]
macro_rules! unset_app_name {
    () => {
        std::env::remove_var("LOGGER_APP_NAME")
    };
}

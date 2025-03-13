macro_rules! define_log_helper {
    ($name:ident, $make_name:ident, $level:ident) => {
        #[macro_export]
        macro_rules! $name {
            (location: $location:expr, hint: $hint:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: Some($hint.to_string()),
                }
                .output();
            }};
            (hint: $hint:expr, location: $location:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: Some($hint.to_string()),
                }
                .output();
            }};
            (location: $location:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: None,
                }
                .output();
            }};
            (hint: $hint:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: None,
                    hint: Some($hint.to_string()),
                }
                .output();
            }};
            ($$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: None,
                    hint: None,
                }
                .output();
            }};
        }

        #[macro_export]
        macro_rules! $make_name {
            (location: $location:expr, hint: $hint:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: Some($hint.to_string()),
                }
            }};
            (hint: $hint:expr, location: $location:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: Some($hint.to_string()),
                }
            }};
            (location: $location:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: Some($location),
                    hint: None,
                }
            }};
            (hint: $hint:expr, $$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: None,
                    hint: Some($hint.to_string()),
                }
            }};
            ($$($$arg:tt)+) => {{
                $crate::Log {
                    level: $crate::level::LogLevel::$level,
                    message: format!($$($$arg)*),
                    location: None,
                    hint: None,
                }
            }};
        }
    };
}

define_log_helper!(trace, make_trace, Trace);
define_log_helper!(debug, make_debug, Debug);
define_log_helper!(info, make_info, Info);
define_log_helper!(warn, make_warn, Warning);
define_log_helper!(error, make_error, Error);
define_log_helper!(fatal, make_fatal, Fatal);

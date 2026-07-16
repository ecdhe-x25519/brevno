#[macro_export]
macro_rules! init_global_logger {
    ($nb:expr, $nl:expr, $level:expr) => {
        // NB - ring buffer size; NB - max log message length (bytes)
        pub static GLOBAL_LOGGER: $crate::log::Logger<$nb, $nl> = $crate::log::Logger::init($level);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        GLOBAL_LOGGER.write_log($crate::log::LogLevel::Error, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        GLOBAL_LOGGER.write_log($crate::log::LogLevel::Warn, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        GLOBAL_LOGGER.write_log($crate::log::LogLevel::Info, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        GLOBAL_LOGGER.write_log($crate::log::LogLevel::Debug, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        GLOBAL_LOGGER.write_log($crate::log::LogLevel::Trace, format_args!($($arg)*))
    };
}
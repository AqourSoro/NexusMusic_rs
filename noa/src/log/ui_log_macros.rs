#[macro_export]
macro_rules! ui_error {
    ($($arg:tt)*) => ({
        // Use the custom logger associated with UI-related logs
        let logger = log::logger();
        logger.log(
            &log::Record::builder()
                .args(format_args!($($arg)*))
                .level(log::Level::Error)
                .target("ui_logger")
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    });
}

#[macro_export]
macro_rules! ui_warn {
    ($($arg:tt)*) => ({
        let logger = log::logger();
        logger.log(
            &log::Record::builder()
                .args(format_args!($($arg)*))
                .level(log::Level::Warn)
                .target("ui_logger")
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    });
}

#[macro_export]
macro_rules! ui_info {
    ($($arg:tt)*) => ({
        let logger = log::logger();
        logger.log(
            &log::Record::builder()
                .args(format_args!($($arg)*))
                .level(log::Level::Info)
                .target("ui_logger")
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    });
}

#[macro_export]
macro_rules! ui_debug {
    ($($arg:tt)*) => ({
        let logger = log::logger();
        logger.log(
            &log::Record::builder()
                .args(format_args!($($arg)*))
                .level(log::Level::Debug)
                .target("ui_logger")
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    });
}

#[macro_export]
macro_rules! ui_trace {
    ($($arg:tt)*) => ({
        let logger = log::logger();
        logger.log(
            &log::Record::builder()
                .args(format_args!($($arg)*))
                .level(log::Level::Trace)
                .target("ui_logger")
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    });
}

pub use {ui_error, ui_warn, ui_info, ui_debug, ui_trace};
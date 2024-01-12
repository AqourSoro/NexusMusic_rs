/// The `noa_log` macro simplifies logging operations through the NexusLogger by providing a
/// convenient way to log messages. It takes a reference to a `NexusLogger`, log level, and log
/// message as arguments. The macro logs the message at the specified log level using the
/// `NexusLogger` implementation.
///
/// # Example
///
/// ```
/// use noa_macro::noa_log;
/// use noa::log::NexusLogger;
///
/// // Your NexusLogger instance
/// let nexus_logger = NexusLogger;
///
/// fn main() {
///     noa_log!(&nexus_logger, noa::log::LogLevel::INFO("Nexus Logger is initialized!"), "main()");
///
///     // Rest of your application logic...
/// }
/// ```
#[macro_export]
macro_rules! noa_log {
    ($logger:expr, $level:expr, $log:expr) => {{
        let logger: &dyn ConsoleLogger = &*$logger;
        logger.log($level, $log, line!());
    }};
}


/// The `noa_ui_log` macro simplifies UI logging operations through the NexusLogger by providing a
/// convenient way to log messages. It takes a reference to a `NexusLogger`, log level, and log
/// message as arguments. The macro logs the message at the specified log level using the
/// `NexusLogger` implementation.
///
/// # Example
///
/// ```
/// use noa_macro::noa_ui_log;
/// use noa::log::NexusLogger;
///
/// // Your NexusLogger instance
/// let nexus_logger = NexusLogger;
///
/// fn main() {
///     noa_ui_log!(&nexus_logger, noa::log::LogLevel::INFO("UI is initialized!"), "main()");
///
///     // Rest of your UI logic...
/// }
/// ```
#[macro_export]
macro_rules! noa_ui_log {
    ($logger:expr, $level:expr, $log:expr) => {{
        let logger: &dyn UILogger = &*$logger;
        logger.log($level, $log, line!());
    }};
}

pub use {noa_log, noa_ui_log};
#[path = "ui_log_macros.rs"]
mod ui_log_macros;
pub use ui_log_macros::*;

use std::path::{Path, PathBuf};
use log::{debug, error, info, trace, warn};
use log4rs;

/// Enum to specify the configuration for the Nexus Music logger.
pub enum NoaLoggerConfig<'a> {
    /// Use the default configuration file path.
    Default,
    /// Use a custom configuration file path.
    Custom(&'a str),
}

pub enum LogLevel<'a>
{
    TRACE(&'a str),
    DEBUG(&'a str),
    INFO(&'a str),
    WARN(&'a str),
    ERROR(&'a str)
}


pub trait NoaLog
{
    fn log(&self);
}


/// Initializes the Nexus Music logger based on the specified configuration.
///
/// # Arguments
///
/// * `config_path` - A `NoaLoggerConfig` enum specifying the logger configuration:
///     - `Default` to use the default configuration file path.
///     - `Custom` to provide a custom configuration file path.
///
/// # Examples
///
/// ```
/// use your_module_name::NoaLoggerConfig;
/// use your_module_name::init_nexus_music_logger;
///
/// // Initialize the logger with the default configuration file path
/// init_nexus_music_logger(NoaLoggerConfig::Default);
///
/// // Initialize the logger with a custom configuration file path
/// init_nexus_music_logger(NoaLoggerConfig::Custom("/path/to/custom/log4rs.yaml"));
/// ```
pub fn init_nexus_music_logger(config_path: NoaLoggerConfig) {
    let path = match config_path 
    {
        NoaLoggerConfig::Custom(p) if Path::new(p).exists() => p,
        _ => "./config/log4rs.yaml",
    };

    log4rs::init_file(path, Default::default()).unwrap();
    trace!("detailed tracing info for debug in Nexus Music!");
    info!("The main program of Nexus Music in Rust!");
}



/// Tests and logs information about a function.
///
/// This function takes a closure `func` as an argument, executes it with two `usize` parameters,
/// and logs information about the function's name and the result it returns.
///
/// # Arguments
///
/// * `func` - A closure or function that takes two `usize` parameters and returns a `usize`.
///
/// # Notes
///
/// The function `func` is executed with values `2` and `2` as parameters, and its name and result
/// are logged using the `debug!` macro.
///
/// # Examples
///
/// ```
/// // Define a test function that adds two numbers
/// let add_function = |a, b| a + b;
///
/// // Test the function logger with the 'add_function'
/// test_func_logger(add_function);
/// ```
pub fn test_func_logger<F>(name: &str, func: F)
where
    F: Fn(usize, usize) -> usize,
{

    let function_name_str = stringify!(func);

    debug!("Test the function {}, result: {}",name, func(2,2));
}


pub fn ui_logger_init(window_name: &str, info: &str)
{
    ui_info!("UI initialied in {}: {}",window_name, info);
}

pub fn ui_log()
{
    ui_info!("this is UI log!");
}

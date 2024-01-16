#[path = "ui_log_macros.rs"]
mod ui_log_macros;
pub use ui_log_macros::*;

use std::path::Path;
use log::{debug, error, info, trace, warn};
use log4rs;

use crate::noa_ui_log;

/// Enum to specify the configuration for the Nexus Music logger.
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum NoaLoggerConfig<'a> {
    /// Use the default configuration file path.
    Default,
    /// Use a custom configuration file path.
    Custom(&'a str),
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum LogLevel<'a>
{
    TRACE(&'a str),
    DEBUG(&'a str),
    INFO(&'a str),
    WARN(&'a str),
    ERROR(&'a str)
}

impl<'a> LogLevel<'a>
{
    pub fn get_message(&'a self) -> &'a str
    {
        match self 
        {
            LogLevel::TRACE(msg) | LogLevel::DEBUG(msg) | LogLevel::INFO(msg)
            | LogLevel::WARN(msg) | LogLevel::ERROR(msg) => msg 
        }
    }    
}

pub enum LogType
{
    Console,
    UI,
}

pub trait ConsoleLogger
{
    fn log(&self, level: LogLevel, location:&str, lines: u32) 
    {
        match level 
        {
            LogLevel::TRACE(log) => trace!("{} => on line {}, in {}",log,lines,location),
            LogLevel::DEBUG(log) => debug!("{} => on line {}, in {}",log,lines,location),
            LogLevel::INFO(log) => info!("{} => on line {}, in {}",log,lines,location),
            LogLevel::WARN(log) => warn!("{} => on line {}, in {}",log,lines,location),
            LogLevel::ERROR(log) => error!("{} => on line {}, in {}",log,lines,location)    
        }
    }

    fn print_logo(&self);
}

pub trait UILogger
{
    fn log(&self, level: LogLevel, location:&str, lines: u32)
    {
        match level 
        {
            LogLevel::TRACE(log) => ui_trace!("{} => on line {}, in {}",log,lines,location),
            LogLevel::DEBUG(log) => ui_debug!("{} => on line {}, in {}",log,lines,location),
            LogLevel::INFO(log) => ui_info!("{} => on line {}, in {}",log,lines,location),
            LogLevel::WARN(log) => ui_warn!("{} => on line {}, in {}",log,lines,location),
            LogLevel::ERROR(log) => ui_error!("{} => on line {}, in {}",log,lines,location)    
        }
    }

    fn print_logo(&self);
}

//Should impl this trait to auto-log
pub trait NoaLog: ConsoleLogger + UILogger
{
    
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct NexusLogger
{



}

impl NexusLogger
{
    pub fn new(config_path: NoaLoggerConfig) -> Self
    {

        let instance = NexusLogger
        {

        };
        init_nexus_music_logger(config_path);
        ConsoleLogger::print_logo(&instance);
        noa_ui_log!(&instance, 
            LogLevel::INFO("Nexus Logger in console is initialized!"), 
            "NexusLogger::new(config_path) -> Self");
        UILogger::print_logo(&instance);
        noa_ui_log!(&instance, 
            LogLevel::INFO("Nexus Logger in UI is initialized!"), 
            "NexusLogger::new(config_path) -> Self");
        instance
    }
}

impl ConsoleLogger for NexusLogger
{
    

    fn print_logo(&self) 
    {
        print_console_logo();
    }
}

impl UILogger for NexusLogger
{
    
    fn print_logo(&self) 
    {
        print_ui_logo();
    }
}

impl NoaLog for NexusLogger 
{
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
fn init_nexus_music_logger(config_path: NoaLoggerConfig) {
    let path = match config_path 
    {
        NoaLoggerConfig::Custom(p) if Path::new(p).exists() => p,
        _ => "./config/log4rs.yaml",
    };

    log4rs::init_file(path, Default::default()).unwrap_or_else(|e| 
    {
        error!("{} oocured, Unable to initialize with the given path: {}", e, path);
        eprint!("There is an error named {} occurs when calling log4rs::init_file with the folloing path: {}, 
                please check and ensure there exists an YAML file for the configuration!", e, path);
    });

    trace!("log4rs is configured with file: {}", path);
    info!("Nexus Music Logger initialized!");
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

    let _function_name_str = stringify!(func);

    debug!("Test the function {}, result: {}",name, func(2,2));
}


fn print_console_logo()
{
    info!("***************************");
    info!("***************************");
    info!(" _   _                     ");
    info!("| \\ | |                    ");
    info!("|  \\| | _____  ___   _ ___ ");
    info!("| . ` |/ _ \\ \\/ / | | / __|");
    info!("| |\\  |  __/>  <| |_| \\__ \\");
    info!("\\_| \\_/\\___/_/\\_\\\\__,_|___/");
    info!("                           ");
    info!("***************************");
    info!("***************************");


}

fn print_ui_logo()
{
    ui_info!("======================================================================");
    ui_info!("======================================================================");
    ui_info!("                                                                      ");
    ui_info!("      ___           ___           ___           ___           ___     ");
    ui_info!("     /\\__\\         /\\  \\         |\\__\\         /\\__\\         /\\  \\    ");
    ui_info!("    /::|  |       /::\\  \\        |:|  |       /:/  /        /::\\  \\   ");
    ui_info!("   /:|:|  |      /:/\\:\\  \\       |:|  |      /:/  /        /:/\\ \\  \\  ");
    ui_info!("  /:/|:|  |__   /::\\~\\:\\  \\      |:|__|__   /:/  /  ___   _\\:\\~\\ \\  \\ ");
    ui_info!(" /:/ |:| /\\__\\ /:/\\:\\ \\:\\__\\ ____/::::\\__\\ /:/__/  /\\__\\ /\\ \\:\\ \\ \\__\\");
    ui_info!(" \\/__|:|/:/  / \\:\\~\\:\\ \\/__/ \\::::/~~/~    \\:\\  \\ /:/  / \\:\\ \\:\\ \\/__/");
    ui_info!("     |:/:/  /   \\:\\ \\:\\__\\    ~~|:|~~|      \\:\\  /:/  /   \\:\\ \\:\\__\\  ");
    ui_info!("     |::/  /     \\:\\ \\/__/      |:|  |       \\:\\/:/  /     \\:\\/:/  /  ");
    ui_info!("     /:/  /       \\:\\__\\        |:|  |        \\::/  /       \\::/  /   ");
    ui_info!("     \\/__/         \\/__/         \\|__|         \\/__/         \\/__/    ");
    ui_info!("                                                                      ");
    ui_info!("======================================================================");
    ui_info!("======================================================================");
}

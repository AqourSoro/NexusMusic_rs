use std::path::{Path, PathBuf};
use log::{debug, error, info, trace, warn};
use log4rs;


enum NoaLogger
{
    
}


/// Initializes the Nexus Music logger.
///
/// This function sets up the logger configuration based on the provided path
/// or uses a default path if no path is provided. It initializes the logger
/// and logs initial messages for tracing and information.
///
/// # Arguments
///
/// * `config_path` - An optional string slice representing the path to the logger configuration file.
///
/// # Notes
///
/// If `config_path` is provided and the path exists, the logger will be initialized with that path.
/// If `config_path` is provided but the path does not exist, the logger will fall back to a default path.
/// If `config_path` is not provided (`None`), the logger will use a default path.
///
/// # Examples
///
/// ```
/// // Initialize the logger with a custom configuration file path
/// init_nexus_music_logger(Some("custom_config.yaml"));
///
/// // Initialize the logger using the default configuration file path
/// init_nexus_music_logger(None);
/// ```
pub fn init_nexus_music_logger(config_path: Option<&str>)
{
    // let path = match config_path 
    // {
    //     Some(p) if Path::new(p).exists() => p,
    //     Some(_) => "./config/log4rs.yaml",
    //     None => "./config/log4rs.yaml",
    // };
    // Below is nicer with options implementation!

    let path = config_path.map_or_else(
        || "./config/log4rs.yaml".to_string(),
        |p| if Path::new(p).exists() 
        { 
            p.to_string() 
        } 
        else 
        { 
            "./config/log4rs.yaml".to_string() 
        });

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


pub fn ui_logger(info: String)
{
    info!("UI initialied: {}", info);
}

use nexus_music::*;
use noa::log::log::*;

fn main() 
{

    let nexus_logger = NexusLogger::new(NoaLoggerConfig::Default);

    test_logger_usage_with_crates();
    display_window(&nexus_logger);
}





use nexus_music::*;
use noa::log::log::*;
use lazy_static::lazy_static;


fn main() 
{

    lazy_static!
    {
        static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
    }

    let nexus_logger = NexusLogger::new(NoaLoggerConfig::Default);

    display_window(&NEXUS_LOGGER);
}





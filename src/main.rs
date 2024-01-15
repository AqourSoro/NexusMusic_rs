use nexus_music::*;
use noa::log::log::*;
use lazy_static::lazy_static;


fn main() 
{

    // use this lazy_static! since the logger can not be initialized at start-up of the program.
    // Also, it ensures its safety among all possible threads.
    lazy_static!
    {
        static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
    }

    let ui_logger: &'static dyn UILogger = &*NEXUS_LOGGER;

    let title = String::from("Nexus Music");

    start_window(ui_logger, title);
    
    
}





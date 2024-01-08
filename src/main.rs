use nexus_music::*;
use noa::log::log::*;

fn main() 
{
    init_nexus_music_logger(NoaLoggerConfig::Default);
    test_logger_usage_with_crates();
    display_window();
}





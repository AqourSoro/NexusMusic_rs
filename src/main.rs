use nexus_music_rs::*;
use graphics_nexus;
use sound_nexus;
use log::{debug, error, info, trace, warn};
use log4rs;

fn main() 
{
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    trace!("detailed tracing info for debug in Nexus Music!");
    info!("The main program of Nexus Music in Rust!");
    debug!("use the functions from graphics and sounds: {}", crate_usage(graphics_nexus::add_in_graphics, sound_nexus::add_in_sound));
}





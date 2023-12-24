use nexus_music_rs::*;
use graphics_nexus;
use sound_nexus;


fn main() {
    println!("The main program of Nexus Music in Rust!");
 
    println!("use the functions from graphics and sounds: {}", crate_usage(graphics_nexus::add_in_graphics, sound_nexus::add_in_sound));
    
}





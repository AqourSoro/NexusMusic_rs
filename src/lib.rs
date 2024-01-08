pub mod logs
{
    pub mod log;
}


use graphics_nexus::{self, show_main_window};
use sound_nexus;
use logs::log::*;
use log::{debug, trace};

pub fn display_window()
{
    show_main_window(logs::log::ui_logger);
}


pub fn crate_usage<FG,FS>(func_graphics:FG, func_sounds:FS) ->usize
where
    FG: Fn(usize,usize) -> usize,
    FS: Fn(usize,usize) -> usize,
{
    func_graphics(2,2) + func_sounds(2,2)
}



pub fn test_logger_usage_with_crates()
{
    test_func_logger("graphics_nexus::add_in_graphics",graphics_nexus::add_in_graphics);
    test_func_logger("sound_nexus::add_in_sound", sound_nexus::add_in_sound);
    trace!("use functions at the same time: {}", crate_usage(graphics_nexus::add_in_graphics, sound_nexus::add_in_sound))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = crate_usage(graphics_nexus::add_in_graphics, sound_nexus::add_in_sound);
        assert_eq!(result, 8);
    }
}
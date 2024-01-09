use noa::log::log::*;
use next_graphics::*;
use next_audio;
use log::{debug, trace};

pub fn display_window()
{
    next_graphics::show_main_window();
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
    test_func_logger("graphics_nexus::add_in_graphics",next_graphics::add_in_graphics);
    test_func_logger("sound_nexus::add_in_sound", next_audio::add_in_sound);
    trace!("use functions at the same time: {}", crate_usage(next_graphics::add_in_graphics, next_audio::add_in_sound))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = crate_usage(next_graphics::add_in_graphics, next_audio::add_in_sound);
        assert_eq!(result, 8);
    }
}
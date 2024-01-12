use noa::log::log::*;

pub fn start_window(logger:&'static dyn UILogger)
{
    let _ = next_graphics::show_main_window(logger);
}


pub fn crate_usage<FG,FS>(func_graphics:FG, func_sounds:FS) ->usize
where
    FG: Fn(usize,usize) -> usize,
    FS: Fn(usize,usize) -> usize,
{
    func_graphics(2,2) + func_sounds(2,2)
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
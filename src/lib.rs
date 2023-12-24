use graphics_nexus;
use sound_nexus;

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
        let result = crate_usage(graphics_nexus::add_in_graphics, sound_nexus::add_in_sound);
        assert_eq!(result, 8);
    }
}
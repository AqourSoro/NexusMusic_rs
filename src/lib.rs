

use noa::{event::thread_event::{EventHandlerClient, GlobalEventSender, NexusEventSender}, log::log::*, noa_ui_log};
use std::sync::{Arc, Mutex};


pub fn start_window(logger:&'static dyn UILogger, event_sender: &'static dyn EventHandlerClient,main_title:String)
{
    let _ = next_graphics::show_main_window(logger,event_sender,main_title);
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
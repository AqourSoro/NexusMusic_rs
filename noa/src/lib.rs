pub mod log
{
    pub mod log;
}

pub mod event
{
    pub mod event;
    pub mod event_handler;
}

pub use log::log::NoaLoggerConfig as NoaLoggerConfig;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // TODO: test when call init_nexus_music_logger returns an error
    fn failed_to_init_logger()
    {
        
    }
}

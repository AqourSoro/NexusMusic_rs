use std::thread;

pub struct EventPool
{
    event_threads: Vec<thread::JoinHandle<()>>,
}

impl EventPool
{
    pub fn new(size:usize) -> EventPool
    {
        assert!(size > 0);

        let mut event_threads = Vec::with_capacity(size);

        for _ in 0..size
        {

        }


        EventPool
        {
            event_threads
        }
    }

    pub fn send_event<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {

    }

}
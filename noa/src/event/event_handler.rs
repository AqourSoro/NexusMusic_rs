use super::event::Event;

pub trait Listener
{
    fn notify(&mut self, event:Event);
}

pub trait Dispatchable<F>
{
    fn register(&mut self, callbacks: F) 
        where F: Fn(&Event) + 'static;
}
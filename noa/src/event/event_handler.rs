#[path = "event.rs"]
mod event;
pub use event::*;
pub trait EventListener
{
    fn on_event(&self, event: &Event);
}

pub struct EventHandler<'a>
{
    listeners: Vec<Box<dyn EventListener + 'a>>,
}

impl<'a> EventHandler<'a>
{
    fn new() -> Self
    {
        EventHandler
        {
            listeners: Vec::new()
        }
    }

    fn add_listener(&mut self, listener: impl EventListener + 'a)
    {
        self.listeners.push(Box::new(listener));
    }

    fn dispatch_event(&self, event: Event)
    {
        for listener in &self.listeners
        {
            listener.on_event(&event);
        }
    }

}
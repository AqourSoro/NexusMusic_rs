use super::event::Event;

pub trait Dispatcher
{
    fn send(&self, event:Event);
}

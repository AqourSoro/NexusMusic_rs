use std::collections::HashMap;


#[derive(Debug)]
pub enum Event<T>
{
    None,
    Default(T),

}

pub type EventListenerList<T> = Vec<Box<dyn Fn(&Event<T>) + 'static>>;
const EVENTMANAGER_QUEUES_NUM: u8 = 2;

pub struct EventManager<T>
{
    
    listeners_map: HashMap<Event<T>, EventListenerList<T>>,
    event_queue: Vec<Event<T>>,

}

trait Eventable<T>
{
    fn as_event(event_data:T) -> Event<T>
    {
        Event::Default(event_data)
    }
}

trait EventManagable<T>
{
    fn add_listener(event:Event<T>, callback: dyn Fn(&Event<T>) + 'static);
    fn remove_listener(event:Event<T>);
    fn queue_event(&mut self, event:Event<T>);
    fn abort_event(&mut self, event:Event<T>);
    fn update(&mut self);
}

pub trait Invoker<T>
{
    fn send(&self, event:Event<T>);
}

pub trait Register<T> 
{
    fn register(&self, callback: Box<dyn Fn(&Event<T>) + 'static>);
    fn unregister(&self);
}


impl<T> Register<T> for Event<T>
{
    fn register(&self, callback: Box<dyn Fn(&Event<T>) + 'static>) 
    {
        
    }

    fn unregister(&self) 
    {
        todo!()
    }
}

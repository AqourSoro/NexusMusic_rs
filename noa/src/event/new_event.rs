use std::collections::HashMap;


#[derive(Debug)]
pub enum Event
{
    None,
    Default(String),
    Key(KeyEvent),
    Mouse(MouseEvent)

}
#[derive(Debug)]
enum KeyEvent
{
    KeyPressed(char),
    KeyReleased(char)

} 
#[derive(Debug)]
enum MouseEvent 
{
    SingleClick(char),
    DoubleClick(char),
    LongPress(char),    
}


pub type EventListenerList = Vec<Box<dyn Fn(&Event) + 'static>>;
const EVENTMANAGER_QUEUES_NUM: u8 = 2;

pub struct EventManager
{
    
    listeners_map: HashMap<Event, EventListenerList>,
    event_queue: Vec<Event>,

}



trait EventManagable
{
    fn add_listener(event:Event, callback: dyn Fn(&Event) + 'static);
    fn remove_listener(event:Event);
    fn queue_event(&mut self, event:Event);
    fn abort_event(&mut self, event:Event);
    fn update(&mut self);
}

pub trait Invoker
{
    fn send(&self, event:Event);
}

pub trait Register 
{
    fn register(&self, callback: Box<dyn Fn(&Event) + 'static>);
    fn unregister(&self);
}

impl EventManager
{
    
}

impl Register for Event
{
    fn register(&self, callback: Box<dyn Fn(&Event) + 'static>) 
    {
        
    }

    fn unregister(&self) 
    {
        todo!()
    }
}

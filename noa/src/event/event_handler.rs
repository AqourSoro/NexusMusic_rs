use std::{cell::RefCell, rc::Weak, sync::Arc};

use log::debug;

use super::event::Event;


pub trait Invokable
{
    fn Invoke(&mut self, event:Event);
}

pub trait Dispatchable
{
    fn add_listener(&mut self, listener:Arc<RefCell<(Event, impl Fn(&Event) + 'static)>>); 
}

pub struct DefaultListener
{
    listeners: Vec<Weak<RefCell<(Event, dyn Fn(&Event) + 'static)>>> 
}

impl Invokable for DefaultListener
{
    fn Invoke(&mut self, event:Event) 
    {
        let ref_listener = self.listeners.clone();
        let callbacks:Vec<Weak<RefCell<(Event, dyn Fn(&Event) + 'static)>>> = ref_listener.into_iter().filter(|listener|
        {
            if let Some(weak_listener) = listener.upgrade()
            {
                if let Ok(listener) = weak_listener.try_borrow()
                {
                    return event == listener.0;
                }
                else
                {
                    debug!("Failed to borrow the RefCell");
                    return false;
                }
            }
            else 
            {
                debug!("Weak reference is no longer valid");
                return false;
            }
        }).collect();
    }
}
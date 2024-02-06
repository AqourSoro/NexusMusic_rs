use std::{cell::RefCell,sync::Arc};

use log::debug;

use super::event::Event;


pub trait Bindable<T>
{
    fn bind(callbackcell:RefCell<dyn Fn(&T) + 'static>);
}


pub struct Bind<T>
{
    pub event: T,
    pub callback: Box<dyn Fn(&T) + 'static>
}


pub struct EventBind
{
    pub event: Event,
    pub callback: Box<dyn Fn(&Event) + 'static>
}

pub struct EventS
{
    pub event:Event,
    callbacks:Vec<Option<Box<dyn Fn(&Event) + 'static>>>
}
//event.register(|| {});
//event.unregister();
//event.send();



pub trait Invokable
{
    fn invoke(&self, event:Event);
}

pub trait Dispatchable
{
    fn add_listener(&mut self, listener:Arc<RefCell<EventBind>>); 
}

pub struct DefaultListener
{
    listeners: Vec<Arc<RefCell<EventBind>>> 
}

impl DefaultListener
{
    pub fn new() -> DefaultListener
    {
        debug!("Create one defaultListener");
        DefaultListener
        {
            listeners: Vec::new()
        }
    }
}

impl Invokable for DefaultListener
{
    // TODO: Fix error that upgrade get nothing!
    fn invoke(&self, event_to_invoke:Event) 
    {
        debug!("listener num: {}", self.listeners.len());


        for listener in &self.listeners
        {
            
            let lsn_cell =  listener.borrow();
            if lsn_cell.event == event_to_invoke 
            {
                (lsn_cell.callback)(&lsn_cell.event);
            }

        }


    }
}

impl Dispatchable for DefaultListener
{
    fn add_listener(&mut self, listener:Arc<RefCell<EventBind>>) 
    {
        debug!("Add a listener!");
        //println!("Add a listener!");
        //self.listeners.push(Arc::downgrade(&listener));
        self.listeners.push(listener);
    }
}
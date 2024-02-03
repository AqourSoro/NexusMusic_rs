use std::{cell::RefCell,sync::Arc};

use log::debug;

use super::event::Event;


pub trait Bindable<T>
{
    fn bind(callbackcell:RefCell< dyn Fn(&T) + 'static>);
}


pub struct Bind<T>
{
    pub event: T,
    pub callback: dyn Fn(&T) + 'static
}


pub struct EventBind
{
    pub event: Event,
    pub callback: Box<dyn Fn(&Event) + 'static>
}




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
        let ref_listener = self.listeners.clone();
        debug!("listener num: {}", ref_listener.len());

        for listener in &self.listeners
        {
            
            let lsn_cell =  listener.borrow();
            if lsn_cell.event == event_to_invoke 
            {
                (lsn_cell.callback)(&lsn_cell.event);
            }

        }

        // let callbacks:Vec<Weak<RefCell<EventBind>>> = ref_listener.iter().filter(|&listener|
        // {
        //     debug!("This is: {:?}", listener);

            

        //     if let Some(weak_listener) = listener.upgrade()
        //     {
        //         if let Ok(listener) = weak_listener.try_borrow()
        //         {
        //             return event_to_invoke == listener.event;
        //         }
        //         else
        //         {
        //             debug!("Failed to borrow the RefCell");
        //             return false;
        //         }
        //     }
        //     else 
        //     {
        //         debug!("What?");
        //         debug!("Weak reference is no longer valid: {:?}", listener);
        //         debug!("listener num: {}", ref_listener.len());
        //         return false;
        //     }
        // }).cloned().collect();

        // for callback_weak in callbacks
        // {
        //     if let Some(callback_cell) = callback_weak.upgrade()
        //     {
        //         let eventbind = &*callback_cell.borrow();

        //         (eventbind.callback)(&eventbind.event);
        //     }
        //     else 
        //     {
        //         debug!("No callback found.");
        //     }
        // }

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
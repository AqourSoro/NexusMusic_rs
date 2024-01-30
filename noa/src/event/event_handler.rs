use std::sync::Arc;

use futures::lock::Mutex;
use log::debug;
use tokio::sync::mpsc;

use crate::{log::log::UILogger, ui_debug};

use super::event::{self, Event};

// TODO：make it work!

pub trait Listener:{
    fn update(&self, event: Event);
}

pub trait Dispatcher {
    fn invoke_event(&self, event: Event);
}

pub struct UIEventHandler {
    event: Event,
}

impl Listener for UIEventHandler {
    fn update(&self, event: Event) {
        ui_debug!("{:?} is received!", event);
    }
}

pub struct EventManager {
    listeners: Vec<Box<dyn Listener>>,
    event: Arc<Mutex<Event>>,
    dispatcher: mpsc::Sender<Event>,
}

// impl EventManager {
//     fn new() -> Self {
//         let (sender, mut receiver) = mpsc::channel::<Event>(5);
//         let data = Arc::new(Mutex::new(Event::None));

//         tokio::spawn(async move {
//             while let Some(event) = receiver.recv().await {
//                 debug!("event received in background tasks: {:?}", event);
//             }
//         });

//         EventManager 
//         {
//             listeners: Vec::new(),
//             event: Arc::clone(&data),
//             dispatcher: sender,
//         }
//     }

//     fn register_listener(&mut self, listener: Box<dyn Listener>) 
//     {
        
//     }


// }

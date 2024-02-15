use std::{cell::RefCell, fmt::Debug, sync::{mpsc, Arc}};

use nexus_music::*;
use noa::{event::event_handler::{self, DefaultListener, Dispatchable, EventBind, Invokable}, log::log::*, noa_log};
use lazy_static::lazy_static;
//use tokio::sync::mpsc;

use noa::event::thread_event::EventHandler as EventHandler;
use std::thread;

fn main() 
{

    // use this lazy_static! since the logger can not be initialized at start-up of the program.
    // Also, it ensures its safety among all possible threads.
    lazy_static!
    {
        static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
        
    }

    let (event_sender, event_receiver) = mpsc::channel();
    let event_handler = EventHandler::new(event_receiver);

    // Spawn a thread for event handling
    let handler_thread = thread::spawn(move || {
        event_handler.start_handling_events();
    });


    //noa_log!(NEXUS_LOGGER,LogLevel::DEBUG("Callback added?"), "main()");

    let ui_logger: &'static dyn UILogger = &*NEXUS_LOGGER;


    let title = String::from("Nexus Music");



    start_window(ui_logger, title);
    


    
}





use std::{cell::RefCell, fmt::Debug, sync::Arc};

use nexus_music::*;
use noa::{event::event_handler::{self, DefaultListener, Dispatchable, EventBind, Invokable}, log::log::*, noa_log};
use noa::event::{utils,new_event::Event};
use lazy_static::lazy_static;


#[tokio::main]
async fn main() 
{

    // use this lazy_static! since the logger can not be initialized at start-up of the program.
    // Also, it ensures its safety among all possible threads.
    lazy_static!
    {
        static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
        
    }

    let a:String = String::from("value");

    let mut listeners = DefaultListener::new();
    let n_listener = Arc::new(RefCell::new(EventBind
    {
        event: noa::event::event::Event::Test,
        callback:Box::new(|e|
        {
            noa_log!(NEXUS_LOGGER,LogLevel::DEBUG(format!("Callback executed with {:?} event!", e).as_str()), "main()");
        })
    }));

    let t_listener = Arc::new(RefCell::new(EventBind
    {
        event: noa::event::event::Event::Test,
        callback:Box::new(|e|
        {
            noa_log!(NEXUS_LOGGER,LogLevel::DEBUG(format!("Callback executed with {:?} event!", e).as_str()), "main()");
        })
    }));

    listeners.add_listener(n_listener);
    listeners.add_listener(t_listener);


    //noa_log!(NEXUS_LOGGER,LogLevel::DEBUG("Callback added?"), "main()");

    let ui_logger: &'static dyn UILogger = &*NEXUS_LOGGER;

    listeners.invoke(noa::event::event::Event::Test);


    let title = String::from("Nexus Music");



    start_window(ui_logger, title);
    


    
}





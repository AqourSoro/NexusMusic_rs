use std::{cell::RefCell, fmt::Debug};

use nexus_music::*;
use noa::{
    event::{
        thread_event::{EventHandlerClient, EventType, Message, NexusEventSender},
        utils::get_event_sender,
    },
    log::log::*,
    noa_log, noa_ui_log,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
}


#[tokio::main]
async fn main() {
    // Access the sender wherever needed.
    let ui_logger: &'static dyn UILogger = &*NEXUS_LOGGER;
    let console_logger: &'static dyn ConsoleLogger = &*NEXUS_LOGGER;
    let nexus_sender: &'static dyn EventHandlerClient = get_event_sender();
    // ... (remaining existing code)

    // register an event for test:
    nexus_sender.register_event(EventType::String, Box::new(|event_data|
    {
        if let Some(data) = event_data.get_data().downcast_ref::<String>()
        {
            noa_log!(console_logger, LogLevel::INFO(format!("Event received: {}", data).as_str()), stringify!(fn main()));
        }
        
    }));

    let title = String::from("Nexus Music");

    // Pass nexus_sender directly to the start_window function.
    start_window(ui_logger, nexus_sender, title);



    
    
}




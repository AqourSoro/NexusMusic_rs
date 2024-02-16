use std::{cell::RefCell, fmt::Debug};

use nexus_music::*;
use noa::{
    event::{
        event_handler::{self, DefaultListener, Dispatchable, EventBind, Invokable},
        thread_event::{EventHandlerClient, EventType, Message, NexusEventSender},
    },
    log::log::*,
    noa_log, noa_ui_log,
};
use lazy_static::lazy_static;

use noa::event::thread_event::EventHandler as EventHandler;
use std::thread;
use std::sync::{mpsc, Arc, Mutex, Once};

lazy_static! {
    static ref NEXUS_LOGGER: NexusLogger = NexusLogger::new(NoaLoggerConfig::Default);
}

static mut SENDER: Option<&'static dyn EventHandlerClient> = None;
static INIT_ONCE: Once = Once::new();

fn initialize_sender(nexus_event_sender: NexusEventSender) {
    unsafe {
        SENDER = Some(Box::leak(Box::new(nexus_event_sender)));
    }
}

fn get_sender() -> &'static dyn EventHandlerClient {
    INIT_ONCE.call_once(|| {
        let (event_sender, event_receiver) = mpsc::channel();
        let event_handler = EventHandler::new(event_receiver);
        let nexus_event_sender = NexusEventSender::new(event_sender);

        initialize_sender(nexus_event_sender);

        // Spawn a thread for event handling
        thread::spawn(move || {
            event_handler.start_handling_events();
        });
    });

    unsafe {
        SENDER.expect("Sender not initialized")
    }
}

fn main() {
    // Access the sender wherever needed.
    let ui_logger: &'static dyn UILogger = &*NEXUS_LOGGER;
    let console_logger: &'static dyn ConsoleLogger = &*NEXUS_LOGGER;
    let nexus_sender: &'static dyn EventHandlerClient = get_sender();

    // ... (remaining existing code)

    // register an event for test:
    nexus_sender.register_event(EventType::String, Box::new(|event_data|
        {
            if let Some(data) = event_data.get_data().downcast_ref::<String>()
            {
                noa_log!(console_logger, LogLevel::DEBUG(format!("Event received: {}", data).as_str()), stringify!(fn main()));
            }
            
        }));

    let title = String::from("Nexus Music");

    // Pass nexus_sender directly to the start_window function.
    start_window(ui_logger, nexus_sender, title);



    
    
}




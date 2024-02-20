use super::thread_event::{EventHandler, EventHandlerClient, NexusEventSender};
use std::sync::{mpsc, Arc, Mutex, Once};
use std::thread;


static mut SENDER: Option<&'static dyn EventHandlerClient> = None;
static INIT_ONCE: Once = Once::new();

pub fn initialize_sender(nexus_event_sender: NexusEventSender) {
    unsafe {
        SENDER = Some(Box::leak(Box::new(nexus_event_sender)));
    }
}

pub fn get_event_sender() -> &'static dyn EventHandlerClient {
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
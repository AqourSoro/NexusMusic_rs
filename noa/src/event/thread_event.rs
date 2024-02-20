use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::SendError;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EventType
{
    Default,
    String,
    Int,
    PressedKey(char)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EventData
{
    String(String),
    U64(u64),
}

// Define a trait for events
pub trait EventBehaviour: Any + Send + Sync{
    fn event_type(&self) -> EventType;
    fn as_any(&self) -> &dyn Any;
}

pub trait EventDataBehaviour: Any + Debug + Send + Sync 
{
    fn as_any(&self) -> &dyn Any;
    fn get_data(&self) -> &dyn Any;
}

impl EventDataBehaviour for EventData
{
    fn as_any(&self) -> &dyn Any 
    {
        self
    }
    fn get_data(&self) -> &dyn Any {
        match self {
            EventData::String(data) => data,
            EventData::U64(data) => data,
        }
    }
}

// Implement the trait for a generic event struct
pub struct Event {
    event_type: EventType,
    data: Box<dyn EventDataBehaviour>,
}


impl EventBehaviour for Event {
    fn event_type(&self) -> EventType {
        self.event_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

impl Event
{

    pub fn new(event_type: EventType, data: Box<dyn EventDataBehaviour>) -> Self
    {
        Event
        {
            event_type,
            data
        }
    }

    fn get_data(&self) -> &Box<dyn EventDataBehaviour>
    {
        &self.data
    }
}


type TypedCallback = Box<dyn Fn(&Box<dyn EventDataBehaviour>) + Send>;

// Enum to represent messages for the centralized event handler
pub enum Message {
    RegisterEvent(EventType, TypedCallback),
    UnregisterEvent(EventType),
    SendEvent(Box<dyn EventBehaviour>),
    Terminate // Not useful right now
}

// Trait for interacting with the centralized event handler
pub trait EventHandlerClient {
    fn register_event(&self, event: EventType, callback: TypedCallback);
    fn unregister_event(&self, event: EventType);
    fn send_event(&self, event: Box<dyn EventBehaviour>);
    fn terminate_event_threads(&self) -> Result<(), SendError<Message>>;
}

pub struct EventHandler {
    event_map: Arc<Mutex<HashMap<EventType, Vec<TypedCallback>>>>,
    event_receiver: mpsc::Receiver<Message>,
}

impl EventHandler {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        EventHandler {
            event_map: Arc::new(Mutex::new(HashMap::new())),
            event_receiver: receiver,
        }
    }

    pub fn start_handling_events(&self) {
        loop {
            // Receive events and handle them
            match self.event_receiver.recv().unwrap() {
                Message::RegisterEvent(event, callback) => {
                    self.register_event(event, callback);
                }
                Message::UnregisterEvent(event) => {
                    self.unregister_event(event);
                }
                Message::SendEvent(event) => {
                    self.handle_event(event);
                }
                Message::Terminate => break
            }
        }
    }

    fn handle_event(&self, event: Box<dyn EventBehaviour>) {
        // Do something before handling the event, if needed
        let map = self.event_map.lock().unwrap();
    
        // Attempt dynamic downcasting without specifying T
        if let Some(event) = event.as_any().downcast_ref::<Event>() {
            if let Some(callbacks) = map.get(&event.event_type()) {
                for callback in callbacks.iter() {
                    // Extract the data from the event and execute the callback
                    callback(event.get_data());
                }
            }
            else {
                println!("nothing is registered!");
            }
        } else {
            // Handle the case where downcast fails
            println!("Downcast failed for event of type {:?}", event.event_type());
        }
    
        // Do something after handling the event, if needed
    }

    fn register_event(&self, event: EventType, callback: TypedCallback) {
        let mut map = self.event_map.lock().unwrap();
        map.entry(event).or_insert_with(Vec::new).push(callback);
    }
    

    fn unregister_event(&self, event: EventType) {
        let mut map = self.event_map.lock().unwrap();
        map.remove(&event);
    }
}

pub struct NexusEventSender
{
    event_sender: mpsc::Sender<Message>,
}

impl EventHandlerClient for NexusEventSender {
    fn register_event(&self, event: EventType, callback: TypedCallback) {
        // Send a registration message to the event handler
        self.event_sender.send(Message::RegisterEvent(event, callback)).unwrap();
    }

    fn unregister_event(&self, event: EventType) {
        // Send an unregistration message to the event handler
        self.event_sender.send(Message::UnregisterEvent(event)).unwrap();
    }

    fn send_event(&self, event: Box<dyn EventBehaviour>) {
        // Send the event to the event handler
        self.event_sender.send(Message::SendEvent(event)).unwrap();
    }
    
    fn terminate_event_threads(&self) -> Result<(), SendError<Message>>
    {
        self.event_sender.send(Message::Terminate)   
    }
}

impl NexusEventSender
{
    pub fn new(event_sender: mpsc::Sender<Message>) -> Self
    {
        NexusEventSender
        {
            event_sender
        }
    }
}

pub type GlobalEventSender = Arc<Option<NexusEventSender>>;
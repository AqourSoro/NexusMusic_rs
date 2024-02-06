use super::new_event::Event;
pub trait Eventable
{
    fn as_event(&self) -> Event
    {
        Event::None
    }
}

impl Eventable for String 
{
    fn as_event(&self) -> Event 
    {
        Event::Default(self.to_owned())
    }    
}
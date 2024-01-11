
#[derive(Debug)]
pub enum Event
{
    Key(KeyEvent),
    Click(ClickEvent),
}

#[derive(Debug)]
pub enum KeyEvent
{
    KeyPressed(char),
    KeyReleased(char)

}

#[derive(Debug)]
pub enum ClickEvent 
{
    SingleClick,
    DoubleClick,
    LongPress,    
}

pub trait EventHandler
{
    fn handle_event<'a>(&mut self, event: &'a Event);
}
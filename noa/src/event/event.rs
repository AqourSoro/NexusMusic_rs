
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event
{
    Key(KeyEvent),
    Click(ClickEvent),
    None
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyEvent
{
    KeyPressed(char),
    KeyReleased(char)

}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ClickEvent 
{
    SingleClick,
    DoubleClick,
    LongPress,    
}
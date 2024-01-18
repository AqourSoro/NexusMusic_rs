
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event
{
    Key(KeyEvent),
    Click(ClickEvent),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyEvent
{
    KeyPressed(char),
    KeyReleased(char)

}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ClickEvent 
{
    SingleClick,
    DoubleClick,
    LongPress,    
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event
{
    Key(KeyEvent),
    Click(ClickEvent),
    Test,
    None
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KeyEvent
{
    KeyPressed(char),
    KeyReleased(char)

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ClickEvent 
{
    SingleClick,
    DoubleClick,
    LongPress,    
}

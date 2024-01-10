use crate::log::log::LogLevel as LogLevel;

#[derive(Debug)]
pub enum Event<'a>
{
    Key(KeyEvent),
    Click(ClickEvent),
    Log(LogEvent<'a>),
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

#[derive(Debug)]
pub enum LogEvent<'a>
{
    ConsoleLog(LogLevel<'a>),
    UILog(LogLevel<'a>)
}
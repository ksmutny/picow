#[derive(Debug, PartialEq)]
pub enum Event {
    Key(KeyCode, u8),
    Mouse(Mouse),
    Paste(String),
}

#[derive(Debug, PartialEq)]
pub enum KeyCode {
    Char(char),
    Esc,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Up,
    Down,
    Right,
    Left,
    Home,
    End,
    PageUp,
    PageDown,
}

pub const CTRL: u8 = 0b0001;
pub const ALT: u8 = 0b0010;
pub const SHIFT: u8 = 0b0100;

#[derive(Debug, PartialEq)]
pub enum Mouse {
    Button(MouseButton, MouseEvent, u16, u16),
    WheelUp(u16, u16),
    WheelDown(u16, u16),
}

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum MouseEvent {
    Press,
    Drag,
    Release,
}

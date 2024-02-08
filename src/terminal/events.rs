#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Key(Key),
    Mouse(Mouse),
    Paste(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Char(char),
    Esc,
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

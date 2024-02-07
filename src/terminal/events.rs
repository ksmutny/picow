#[derive(Debug, PartialEq)]
pub enum Event {
    Key(Key),
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

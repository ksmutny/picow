#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Key(Key),
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

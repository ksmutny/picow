pub enum Command {
    Clear,
    ClearLine,
    ClearToEndOfLine,
    Print(String),
    SetBackgroundColor(u8),

    MoveTo(u16, u16),
    MoveUp(u16),
    MoveLeft(u16),
    MoveRight(u16),
    MoveDown(u16),
    HideCursor,
    ShowCursor,

    EnterAlternateScreen,
    LeaveAlternateScreen,

    EnableMouseCapture,
    DisableMouseCapture,

    EnableBracketedPaste,
    DisableBracketedPaste,

    SetWindowTitle(String),
}

pub enum Command {
    Clear,
    ClearLine,
    Print(String),

    MoveTo(u16, u16),
    MoveUp(u16),
    MoveLeft(u16),
    MoveRight(u16),
    MoveDown(u16),

    EnterAlternateScreen,
    LeaveAlternateScreen,
}

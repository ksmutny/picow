use crate::terminal::commands::Command::{self, *};

macro_rules! csi {
    ($($arg:tt)*) => {{
        format!("\x1b[{}", format!($($arg)*))
    }};
}

macro_rules! osc {
    ($($arg:tt)*) => {{
        format!("\x1b]{}\x1b\x5c", format!($($arg)*))
    }};
}

pub fn ansi(command: &Command) -> String {
    match command {
        Clear => csi!("2J{}", ansi(&MoveTo(1, 1))),
        ClearLine => csi!("2K"),
        Print(s) => s.to_string(),

        MoveTo(x, y) => csi!("{};{}H", y, x),
        MoveUp(n) => csi!("{}A", n),
        MoveDown(n) => csi!("{}B", n),
        MoveRight(n) => csi!("{}C", n),
        MoveLeft(n) => csi!("{}D", n),
        HideCursor => csi!("?25l"),
        ShowCursor => csi!("?25h"),

        EnterAlternateScreen => csi!("?1049h{}", ansi(&MoveTo(1  , 1))),
        LeaveAlternateScreen => csi!("?1049l"),

        EnableMouseCapture => csi!("?1002h"),
        DisableMouseCapture => csi!("?1002l"),

        EnableBracketedPaste => csi!("?2004h"),
        DisableBracketedPaste => csi!("?2004l"),

        SetWindowTitle(title) => osc!("0;{}", title),
    }
}

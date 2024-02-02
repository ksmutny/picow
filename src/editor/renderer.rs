use std::{io, cmp::min};

use crate::terminal;

use super::{commands::Command, state::EditorState, CommandBuffer, CommandExecutor};

pub struct EditorRenderer {
    commands: CommandBuffer,
}

impl EditorRenderer {
    pub fn new() -> Self {
        Self { commands: CommandBuffer::new() }
    }

    pub fn open() -> io::Result<()> {
        terminal::init()?;
        Command::EnterAlternateScreen.execute()
    }

    pub fn close() -> io::Result<()> {
        Command::LeaveAlternateScreen.execute()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.commands.execute()
    }

    pub fn refresh(&mut self, state: &EditorState) {
        self.commands.queue(Command::Clear);

        for i in 0..state.viewport_height() {
            if let Some(row) = state.lines.get(state.scroll_top() + i as usize) {
                self.commands.queue(Command::MoveTo(1, i + 1));

                let start = state.scroll_left() as usize;
                let len = min(row.len() - min(start, row.len()), state.viewport_width() as usize);
                let slice = if row.len() > start && len > 0 { &row[start..start + len] } else { "" };

                self.commands.queue(Command::Print(slice.to_string()));
            } else {
                break;
            }
        }
    }

    pub fn refresh_cursor(&mut self, state: &EditorState) {
        let (x_abs, y_abs) = state.cursor_pos;
        let (scroll_left, scroll_top) = state.scroll_pos;
        let (x, y) = ((x_abs - scroll_left + 1) as u16, (y_abs - scroll_top + 1) as u16);
        self.commands.queue(Command::MoveTo(x, y))
    }

    pub fn refresh_status_bar(&mut self, state: &EditorState, delimiter: &str) {
        let (width, height) = state.viewport_size;
        let (x, y) = state.cursor_pos;

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x + 1, y + 1, state.scroll_top() + 1, self.delimiter_label(delimiter));

        self.commands.queue(Command::MoveTo(1, state.viewport_height() + 1));
        self.commands.queue(Command::ClearLine);
        self.commands.queue(Command::Print(status));
        self.refresh_cursor(state);
    }

    fn delimiter_label(&self, delimiter: &str) -> &str {
        use crate::file::{CRLF, CR, LF};

        match delimiter {
            CRLF => "CRLF",
            CR => "CR",
            LF => "LF",
            _ => "?"
        }
    }
}

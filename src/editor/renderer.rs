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

        for (i, line) in self.visible_lines(state).iter().enumerate() {
            self.commands.queue(Command::MoveTo(1, 1 + i as u16));
            let slice = self.visible_part(line, state);
            self.commands.queue(Command::Print(slice.to_string()));
        }
    }

    fn visible_lines<'a>(&self, state: &'a EditorState) -> &'a [String] {
        let top = state.scroll_top();
        let height = state.viewport_height() as usize;
        let bottom = min(top + height, state.lines.len());

        &state.lines[top..bottom]
    }

    fn visible_part<'a>(&self, line: &'a str, state: &EditorState) -> &'a str {
        let start = state.scroll_left();
        if line.len() <= start { return "" }

        let width = state.viewport_width() as usize;
        let len = min(line.len() - start, width);

        &line[start..start + len]
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

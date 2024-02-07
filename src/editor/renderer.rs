use std::{io, cmp::min};

use crate::terminal::{self, commands::Command, CommandBuffer, CommandExecutor};
use super::state::{EditorState, Viewport};


pub struct EditorRenderer {
    commands: CommandBuffer,
    cursor_hidden: bool,
}

impl EditorRenderer {
    pub fn new() -> Self {
        Self {
            commands: CommandBuffer::new(),
            cursor_hidden: false,
        }
    }

    pub fn open(file_name: String) -> io::Result<()> {
        terminal::init()?;
        Command::EnterAlternateScreen.execute()?;
        Command::EnableMouseCapture.execute()?;
        Command::SetWindowTitle(file_name).execute()
    }

    pub fn create_viewport() -> io::Result<Viewport> {
        let (width, height) = terminal::terminal_size()?;
        Ok(Viewport::new(0, 0, width, height - 1))
    }

    pub fn close() -> io::Result<()> {
        Command::DisableMouseCapture.execute()?;
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
        let top = state.viewport.top;
        let height = state.viewport.height as usize;
        let bottom = min(top + height, state.content.lines.len());

        &state.content.lines[top..bottom]
    }

    fn visible_part<'a>(&self, line: &'a str, state: &EditorState) -> &'a str {
        let start = state.viewport.left;
        if line.len() <= start { return "" }

        let width = state.viewport.width as usize;
        let len = min(line.len() - start, width);

        &line[start..start + len]
    }

    pub fn refresh_cursor(&mut self, state: &EditorState) {
        let (x_abs, y_abs) = state.cursor_pos;

        if !state.viewport.cursor_within(state.cursor_pos) {
            self.hide_cursor();
        } else {
            self.show_cursor()
        }

        if !self.cursor_hidden {
            let (x, y) = state.viewport.to_relative((x_abs, y_abs));
            self.commands.queue(Command::MoveTo(x, y))
        }
    }

    fn show_cursor(&mut self) {
        if !self.cursor_hidden { return }
        self.commands.queue(Command::ShowCursor);
        self.cursor_hidden = false;
    }

    fn hide_cursor(&mut self) {
        if self.cursor_hidden { return }
        self.commands.queue(Command::HideCursor);
        self.cursor_hidden = true;
    }

    pub fn refresh_status_bar(&mut self, state: &EditorState) {
        let Viewport { top, width, height, .. } = state.viewport;
        let (x, y) = state.cursor_pos;

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x + 1, y + 1, top + 1, self.delimiter_label(&state.content.delimiter));

        self.commands.queue(Command::MoveTo(1, state.viewport.height + 1));
        self.commands.queue(Command::ClearLine);
        self.commands.queue(Command::Print(status));
        self.refresh_cursor(state);
    }

    fn delimiter_label(&self, delimiter: &str) -> &str {
        use super::content::{CRLF, CR, LF};

        match delimiter {
            CRLF => "CRLF",
            CR => "CR",
            LF => "LF",
            _ => "?"
        }
    }
}

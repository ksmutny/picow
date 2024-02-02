pub mod state;
pub mod navigation;

use std::{cmp, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use crate::terminal::{self, *, commands::Command};

use self::{navigation::{CursorCommand, NavigationCommand, ScrollCommand}, state::{EditorState, VerticalNavigation}};


pub struct Editor {
    state: EditorState,
    delimiter: String,
    commands: CommandBuffer,
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let terminal_size = terminal::terminal_size().unwrap();
        Self {
            state: EditorState {
                viewport_size: terminal_size,
                scroll_pos: (0, 0),
                cursor_pos: (0, 0),
                lines: rows,
                vertical_nav: VerticalNavigation::new(),
            },
            delimiter,
            commands: CommandBuffer::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        Editor::open()?;
        self.refresh();
        self.refresh_cursor();
        self.event_loop()?;
        Editor::close()
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            match event::read()? {
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, modifiers, .. }) => {
                    use KeyCode::*;
                    const CTRL: KeyModifiers = KeyModifiers::CONTROL;

                    match code {
                        Up | Down | PageUp | PageDown => self.state.vertical_nav.start(self.state.cursor_x()),
                        _ => self.state.vertical_nav.end(),
                    }

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        (Char(c), _) => self.commands.queue(Command::Print(c.to_string())),

                        (Up, CTRL) => self.queue(self.state.scroll_up(1)),
                        (Down, CTRL) => self.queue(self.state.scroll_down(1)),
                        (Home, CTRL) =>  self.queue(self.state.move_document_start()),
                        (End, CTRL) =>  self.queue(self.state.move_document_end()),

                        (Right, _) => self.queue(self.state.move_right()),
                        (Left, _) => self.queue(self.state.move_left()),
                        (Up, _) => self.queue(self.state.move_up(1)),
                        (Down, _) => self.queue(self.state.move_down(1)),
                        (Home, _) => self.queue(self.state.move_line_start()),
                        (End, _) => self.queue(self.state.move_line_end()),
                        (PageUp, _) => self.queue(self.state.move_up(self.state.viewport_height() as usize - 1)),
                        (PageDown, _) =>  self.queue(self.state.move_down(self.state.viewport_height() as usize - 1)),
                        _ => {}
                    }
                },
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    use MouseButton::*;

                    match kind {
                        MouseEventKind::Down(Left) => self.queue(self.state.click(column + 1, row + 1)),
                        MouseEventKind::ScrollDown => self.queue(self.state.scroll_down(1)),
                        MouseEventKind::ScrollUp => self.queue(self.state.scroll_up(1)),
                        _ => {}
                    }
                },
                Event::Resize(width, height) => self.resize((width, height)),
                _ => {}
            }
            self.status_bar();
            self.commands.execute()?;
        }
    }

    fn queue(&mut self, (scroll_cmd, cursor_cmd): NavigationCommand) {
        if let ScrollCommand::ScrollTo(x, y) = scroll_cmd {
            self.state.scroll_pos = (x, y);
            self.refresh();
        }
        if let CursorCommand::MoveTo(x, y) = cursor_cmd {
            self.state.cursor_pos = (x, y);
            self.refresh_cursor();
        }
    }

    fn refresh(&mut self) {
        self.commands.queue(Command::Clear);

        for i in 0..self.state.viewport_height() {
            if let Some(row) = self.state.lines.get(self.state.scroll_top() + i as usize) {
                self.commands.queue(Command::MoveTo(1, i + 1));

                let start = self.state.scroll_left() as usize;
                let len = cmp::min(row.len() - cmp::min(start, row.len()), self.state.viewport_width() as usize);
                let slice = if row.len() > start && len > 0 { &row[start..start + len] } else { "" };

                self.commands.queue(Command::Print(slice.to_string()));
            } else {
                break;
            }
        }
    }

    fn refresh_cursor(&mut self) {
        let (x_abs, y_abs) = self.state.cursor_pos;
        let (scroll_left, scroll_top) = self.state.scroll_pos;
        let (x, y) = ((x_abs - scroll_left + 1) as u16, (y_abs - scroll_top + 1) as u16);
        self.commands.queue(Command::MoveTo(x, y))
    }

    fn resize(&mut self, terminal_size: Coordinates) {
        self.state.viewport_size = (terminal_size.0, terminal_size.1 - 1);
        self.refresh()
    }

    fn status_bar(&mut self) {
        let (width, height) = self.state.viewport_size;
        let (x, y) = self.state.cursor_pos;

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x + 1, y + 1, self.state.scroll_top() + 1, self.delimiter_label());

        self.commands.queue(Command::MoveTo(1, self.terminal_height()));
        self.commands.queue(Command::ClearLine);
        self.commands.queue(Command::Print(status));
        self.refresh_cursor();
    }

    fn delimiter_label(&self) -> &str {
        use crate::file::{CRLF, CR, LF};

        match self.delimiter.as_str() {
            CRLF => "CRLF",
            CR => "CR",
            LF => "LF",
            _ => "?"
        }
    }

    fn terminal_height(&self) -> u16 { self.state.viewport_height() + 1 }

    fn open() -> io::Result<()> {
        terminal::init()?;
        Command::EnterAlternateScreen.execute()
    }

    fn close() -> io::Result<()> {
        Command::LeaveAlternateScreen.execute()
    }
}

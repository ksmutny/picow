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
                cursor_pos: (1, 1),
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
        self.move_to(1, 1);
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
                        Up | Down | PageUp | PageDown => self.state.vertical_nav.start(self.state.cursor_x_abs()),
                        _ => self.state.vertical_nav.end(),
                    }

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        (Char(c), _) => self.commands.queue(Command::Print(c.to_string())),

                        (Up, CTRL) => self.scroll_up(1),
                        (Down, CTRL) => self.scroll_down(1),
                        (Home, CTRL) =>  self.queue(navigation::move_document_start(&self.state)),
                        (End, CTRL) =>  self.queue(navigation::move_document_end(&self.state)),

                        (Right, _) => self.queue(navigation::move_right(&self.state)),
                        (Left, _) => self.queue(navigation::move_left(&self.state)),
                        (Up, _) => self.queue(navigation::move_up(&self.state, 1)),
                        (Down, _) => self.queue(navigation::move_down(&self.state, 1)),
                        (Home, _) => self.queue(navigation::move_line_start(&self.state)),
                        (End, _) => self.queue(navigation::move_line_end(&self.state)),
                        (PageUp, _) => self.queue(navigation::move_up(&self.state, self.state.viewport_height() as usize - 1)),
                        (PageDown, _) =>  self.queue(navigation::move_down(&self.state, self.state.viewport_height() as usize - 1)),
                        _ => {}
                    }
                },
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    use MouseButton::*;

                    match kind {
                        MouseEventKind::Down(Left) => self.queue(navigation::click(&self.state, column + 1, row + 1)),
                        MouseEventKind::ScrollDown => self.scroll_down(1),
                        MouseEventKind::ScrollUp => self.scroll_up(1),
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

    fn queue(&mut self, commands: NavigationCommand) {
        use CursorCommand::*;
        use ScrollCommand::*;

        let (scroll_cmd, cursor_cmd) = commands;
        match scroll_cmd {
            ScrollTo(x, y) => self.scroll_to(x, y),
            NoScroll => {}
        }
        match cursor_cmd {
            MoveTo(x, y) => {
                self.commands.queue(Command::MoveTo(x, y));
                self.state.cursor_pos = (x, y);
            },
            NoMove => {}
        }
    }

    fn move_to(&mut self, x: u16, y: u16) {
        let eof_y = self.state.lines.len() - self.state.scroll_top();
        let new_y = y.clamp(1, eof_y as u16);

        let eol = self.line_at(new_y).len() as u16 + 1;
        let new_x = x.clamp(1, eol);

        self.state.cursor_pos = (new_x, new_y);
        self.commands.queue(Command::MoveTo(new_x, new_y))
    }

    fn scroll_to(&mut self, x: usize, y: usize) {
        self.state.scroll_pos = (x, cmp::min(y, self.state.lines.len() - 1));
        self.refresh()
    }

    fn scroll_up(&mut self, delta: usize) {
        let delta = cmp::min(delta, self.state.scroll_top());
        self.scroll_to(self.state.scroll_left(), self.state.scroll_top() - delta);
    }

    fn scroll_down(&mut self, delta: usize) {
        self.scroll_to(self.state.scroll_left(), self.state.scroll_top() + delta);
    }

    fn line_at(&self, y: u16) -> &str {
        &self.state.lines[self.state.scroll_top() + y as usize - 1]
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

    fn resize(&mut self, terminal_size: Coordinates) {
        self.state.viewport_size = (terminal_size.0, terminal_size.1 - 1);
        self.refresh()
    }

    fn status_bar(&mut self) {
        let (width, height) = self.state.viewport_size;
        let (x, y) = self.state.cursor_pos;
        let (x_abs, y_abs) = self.state.cursor_pos_abs();

        let status = format!("{}x{} | {} {} | {} | {}", width, height, (x_abs + 1), (y_abs + 1), self.state.scroll_top(), self.delimiter_label());

        self.commands.queue(Command::MoveTo(1, self.terminal_height()));
        self.commands.queue(Command::ClearLine);
        self.commands.queue(Command::Print(status));
        self.commands.queue(Command::MoveTo(x, y))
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

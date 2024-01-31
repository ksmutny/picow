use std::{cmp, io};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind}, style::Print};

use crate::terminal::{self, *, commands::Command};


pub struct Editor {
    terminal_size: Coordinates,
    top: usize,
    rows: Vec<String>,
    delimiter: String,
    vertical_nav: VerticalNavigation,
    cursor: Coordinates,
    commands: CommandBuffer,
}

struct VerticalNavigation {
    in_progress: bool,
    last_x: u16,
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let terminal_size = terminal::terminal_size().unwrap();
        Self {
            rows, delimiter, terminal_size, top: 0,
            vertical_nav: VerticalNavigation { in_progress: false, last_x: 0 },
            cursor: (1, 1),
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

                    self.update_vertical_nav(code);

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        (Char(c), _) => self.commands.queue(Command::Print(c.to_string())),

                        (Up, CTRL) => self.scroll_up(1),
                        (Down, CTRL) => self.scroll_down(1),
                        (Home, CTRL) =>  self.move_home_document(),
                        (End, CTRL) =>  self.move_end_document(),

                        (Right, _) =>  self.move_right(1),
                        (Left, _) =>  self.move_left(1),
                        (Up, _) =>  self.move_up(1),
                        (Down, _) =>  self.move_down(1),
                        (Home, _) =>  self.move_home_line(),
                        (End, _) =>  self.move_end_line(),
                        (PageUp, _) =>  self.move_up(self.viewport_height() - 1),
                        (PageDown, _) =>  self.move_down(self.viewport_height() - 1),
                        _ => {}
                    }
                },
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    use MouseButton::*;

                    match kind {
                        MouseEventKind::Down(Left) => self.move_to(column + 1, row + 1),
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

    fn update_vertical_nav(&mut self, key_code: KeyCode) {
        use KeyCode::*;

        match key_code {
            Up | Down | PageUp | PageDown =>
                if !self.vertical_nav.in_progress {
                    self.vertical_nav.in_progress = true;
                    self.vertical_nav.last_x = self.cursor_x();
                },
            _ => self.vertical_nav.in_progress = false
        }
    }

    fn vertical_nav_x(&self, x: u16) -> u16 {
        if self.vertical_nav.in_progress {
            self.vertical_nav.last_x
        } else {
            x
        }
    }

    fn move_to(&mut self, x: u16, y: u16) {
        let eof_y = self.rows.len() - self.top;
        let new_y = y.clamp(1, eof_y as u16);

        let eol = self.line_at(new_y).len() as u16 + 1;
        let new_x = x.clamp(1, eol);

        self.cursor = (new_x, new_y);
        self.commands.queue(Command::MoveTo(new_x, new_y))
    }

    fn move_up(&mut self, n: u16) {
        let (x, y) = self.cursor;
        let new_x = self.vertical_nav_x(x);

        if y == 1 && self.top > 0 {
            self.scroll_up(n as usize);
            self.move_to(new_x, y)
        } else {
            let delta = n.clamp(1, y);
            self.move_to(new_x, y - delta)
        }
    }

    fn move_down(&mut self, n: u16) {
        let (x, y) = self.cursor;
        let new_x = self.vertical_nav_x(x);
        let height = self.viewport_height();

        let scroll_below_viewport = (y + n) > height;
        let rows_below_viewport = (self.top + height as usize) < self.rows.len();

        if scroll_below_viewport && rows_below_viewport {
            self.scroll_down(n as usize);
            self.move_to(new_x, y)
        } else {
            let delta = cmp::min(y + n, height) - y;
            self.move_to(new_x, y + delta)
        }
    }

    fn move_right(&mut self, n: u16) {
        let (x, y) = self.cursor;
        let row_len = self.line_at(y).len() as u16;

        if x + n > row_len + 1 {
            self.move_to(1, y + 1)
        } else {
            self.move_to(x + n, y)
        }
    }

    fn move_left(&mut self, n: u16) {
        let (x, y) = self.cursor;

        if x <= n && self.curr_line_idx() > 0 {
            self.move_to(self.line_at(y - 1).len() as u16 + 1, y - 1)
        } else {
            self.move_to(x - n, y)
        }
    }

    fn move_home_line(&mut self) {
        self.move_to(1, self.cursor_y())
    }

    fn move_end_line(&mut self) {
        let y = self.cursor_y();
        let row_len = self.line_at(y).len() as u16;

        self.move_to(row_len + 1, y)
    }

    fn move_home_document(&mut self) {
        self.scroll_to(0);
        self.move_to(1, 1)
    }

    fn move_end_document(&mut self) {
        self.scroll_to(cmp::max(self.top, self.rows.len() - self.viewport_height() as usize));

         let eof_y = self.rows.len() - self.top;
        let eof_x = self.line_at(eof_y as u16).len() as u16 + 1;
        self.move_to(eof_x, eof_y as u16)
    }

    fn scroll_to(&mut self, y: usize) {
        self.top = cmp::min(y, self.rows.len() - 1);
        self.refresh()
    }

    fn scroll_up(&mut self, delta: usize) {
        let delta = cmp::min(delta, self.top);
        self.scroll_to(self.top - delta);
    }

    fn scroll_down(&mut self, delta: usize) {
        self.scroll_to(self.top + delta);
    }

    fn curr_line_idx(&self) -> usize {
        self.top + self.cursor_y() as usize - 1
    }

    fn line_at(&self, y: u16) -> &str {
        &self.rows[self.top + y as usize - 1]
    }


    fn refresh(&mut self) {
        self.commands.queue(Command::Clear);

        for i in 0..self.viewport_height() {
            if let Some(row) = self.rows.get(self.top + i as usize) {
                self.commands.queue(Command::MoveTo(1, i + 1));
                self.commands.queue(Command::Print(row.to_string()));
            } else {
                break;
            }
        }
    }

    fn resize(&mut self, terminal_size: Coordinates) {
        self.terminal_size = terminal_size;
        self.refresh()
    }

    fn status_bar(&mut self) {
        let (width, height) = self.viewport_size();
        let (x, y) = self.cursor;

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x, y, self.top, self.delimiter_label());

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

    fn cursor_x(&self) -> u16 { self.cursor.0 }
    fn cursor_y(&self) -> u16 { self.cursor.1 }

    fn terminal_height(&self) -> u16 { self.terminal_size.1 }

    fn viewport_size(&self) -> Coordinates { (self.terminal_size.0, self.terminal_height() - 1) }
    fn viewport_height(&self) -> u16 { self.viewport_size().1 }

    fn open() -> io::Result<()> {
        terminal::init()?;
        Command::EnterAlternateScreen.execute()
    }

    fn close() -> io::Result<()> {
        Command::LeaveAlternateScreen.execute()
    }
}

use std::{cmp, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::terminal::{self, *, commands::Command::{self, *}};


pub struct Editor {
    terminal_size: Coordinates,
    top: u16,
    rows: Vec<String>,
    delimiter: String
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let terminal_size = terminal::terminal_size().unwrap();
        Self { rows, delimiter, terminal_size, top: 0 }
    }

    pub fn run(&mut self) -> io::Result<()> {
        Editor::open()?;
        self.refresh()?;
        self.event_loop()?;
        Editor::close()
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            match event::read()? {
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, modifiers, .. }) => {
                    use KeyCode::*;
                    const CTRL: KeyModifiers = KeyModifiers::CONTROL;

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        (Up, CTRL) => self.scroll(-1)?,
                        (Down, CTRL) => self.scroll(1)?,

                        (Right, _) =>  MoveRight(1).queue()?,
                        (Left, _) =>  MoveLeft(1).queue()?,
                        (Up, _) =>  self.move_up(1)?,
                        (Down, _) =>  self.move_down(1)?,
                        (Home, _) =>  self.move_home_line()?,
                        (End, _) =>  self.move_end_line()?,
                        (PageUp, _) =>  self.move_up(self.viewport_height() - 1)?,
                        (PageDown, _) =>  self.move_down(self.viewport_height() - 1)?,
                        _ => {}
                    }
                    terminal::flush()?;
                },
                Event::Resize(width, height) => self.resize((width, height))?,
                _ => {}
            }
            self.status_bar()?;
            terminal::flush()?;
        }
    }

    fn move_up(&mut self, n: u16) -> io::Result<()> {
        let (x, y) = self.cursor();

        if y == 1 && self.top > 0 {
            self.scroll(-(n as i16))?;
            MoveTo(x, y).queue()
        } else {
            MoveUp(n).queue()
        }
    }

    fn move_down(&mut self, n: u16) -> io::Result<()> {
        let (x, y) = self.cursor();
        let height = self.viewport_height();

        let scroll_below_viewport = (y + n) > height;
        let rows_below_viewport = (self.top + height) < self.rows.len() as u16;

        if scroll_below_viewport && rows_below_viewport {
            self.scroll(n as i16)?;
            MoveTo(x, y).queue()
        } else if !scroll_below_viewport {
            let delta = cmp::min(y + n, height) - y;
            MoveDown(delta).queue()
        } else {
            Ok(())
        }
    }

    fn move_home_line(&self) -> io::Result<()> {
        MoveTo(1, self.cursor_y()).queue()
    }

    fn move_end_line(&self) -> io::Result<()> {
        let y = self.cursor_y();
        let row_len = self.rows[y as usize - 1].len() as u16;

        MoveTo(row_len + 1, y).queue()
    }

    fn scroll(&mut self, delta: i16) -> io::Result<()> {
        let (x, y) = self.cursor();

        self.top = (self.top as i16 + delta).clamp(0, self.rows.len() as i16) as u16;
        self.refresh()?;

        let new_y = (y as i16 - delta).clamp(1, self.viewport_height() as i16 - 1) as u16;

        MoveTo(x, new_y).queue()
    }

    fn refresh(&self) -> io::Result<()> {
        let mut commands = vec![Command::Clear];

        for i in 0..self.viewport_height() {
            if let Some(row) = self.rows.get((self.top + i) as usize) {
                commands.push(Command::MoveTo(1, i + 1));
                commands.push(Command::Print(row.to_string()));
            } else {
                break;
            }
        }
        commands.queue()
    }

    fn resize(&mut self, terminal_size: Coordinates) -> io::Result<()> {
        self.terminal_size = terminal_size;
        self.refresh()
    }

    fn status_bar(&self) -> io::Result<()> {
        let (width, height) = self.viewport_size();
        let (x, y) = self.cursor();

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x, y, self.top, self.delimiter_label());

        vec![
            MoveTo(1, self.terminal_height()),
            ClearLine,
            Print(status),
            MoveTo(x, y)
        ].queue()
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

    fn cursor(&self) -> Coordinates {
        terminal::cursor_position().unwrap()
    }

    fn cursor_y(&self) -> u16 { self.cursor().1 }

    fn terminal_height(&self) -> u16 { self.terminal_size.1 }

    fn viewport_size(&self) -> Coordinates { (self.terminal_size.0, self.terminal_height() - 1) }
    fn viewport_height(&self) -> u16 { self.viewport_size().1 }

    fn open() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        EnterAlternateScreen.execute()
    }

    fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        LeaveAlternateScreen.execute()
    }
}

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::terminal::{self, *, commands::Command::{self, *}};


pub struct Editor {
    size: Coordinates,
    top: u16,
    rows: Vec<String>,
    delimiter: String
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let size = terminal::terminal_size().unwrap();
        Self { rows, delimiter, size, top: 0 }
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
        let (_, height) = self.size;

        let at_screen_bottom = y == height;
        let at_file_bottom = (self.top + height) >= self.rows.len() as u16;

        if at_screen_bottom {
            if at_file_bottom { Ok(()) } else {
                self.scroll(n as i16)?;
                MoveTo(x, y).queue()
            }
        } else {
            MoveDown(n).queue()
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

        let new_y = (y as i16 - delta).clamp(1, self.size.1 as i16 - 1) as u16;

        MoveTo(x, new_y).queue()
    }

    fn refresh(&self) -> io::Result<()> {
        let mut commands = vec![Command::Clear];

        for i in 0..self.size.1 - 1 {
            if let Some(row) = self.rows.get((self.top + i) as usize) {
                commands.push(Command::MoveTo(1, i + 1));
                commands.push(Command::Print(row.to_string()));
            } else {
                break;
            }
        }
        commands.queue()
    }

    fn resize(&mut self, size: Coordinates) -> io::Result<()> {
        self.size = size;
        self.refresh()
    }

    fn status_bar(&self) -> io::Result<()> {
        let (width, height) = self.size;
        let (x, y) = self.cursor();

        let status = format!("{}x{} | {} {} | {} | {}", width, height, x, y, self.top, self.delimiter_label());

        vec![
            MoveTo(1, height),
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

    fn open() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        EnterAlternateScreen.execute()
    }

    fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        LeaveAlternateScreen.execute()
    }
}

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::terminal::{self, *, commands::Command::{self, *}};


pub struct Editor {
    size: Coordinates,
    rows: Vec<String>,
    delimiter: String
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let size = terminal::terminal_size().unwrap();
        Self { rows, delimiter, size }
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
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, .. }) => match code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Right =>  MoveRight(1).execute()?,
                    KeyCode::Left =>  MoveLeft(1).execute()?,
                    KeyCode::Up =>  MoveUp(1).execute()?,
                    KeyCode::Down =>  MoveDown(1).execute()?,
                    _ => {}
                },
                Event::Resize(width, height) => self.resize((width, height))?,
                _ => {}
            }
            self.status_bar()?;
        }
    }

    fn refresh(&self) -> io::Result<()> {
        let mut commands = vec![Command::Clear];

        for (y, row) in self.rows.iter().enumerate() {
            commands.push(Command::MoveTo(1, y as u16 + 1));
            commands.push(Command::Print(row.to_string()));
        }
        commands.execute()?;
        self.status_bar()
    }

    fn resize(&mut self, size: Coordinates) -> io::Result<()> {
        self.size = size;
        self.refresh()
    }

    fn status_bar(&self) -> io::Result<()> {
        let (width, height) = self.size;
        let (x, y) = terminal::cursor_position()?;

        let status = format!("{}x{} | {} {} | {}", width, height, x, y, self.delimiter_label());

        vec![
            MoveTo(1, height),
            ClearLine,
            Print(status),
            MoveTo(x, y)
        ].execute()
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

    fn open() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        EnterAlternateScreen.execute()
    }

    fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        LeaveAlternateScreen.execute()
    }
}

use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal};

use crate::{commands::Command::{self, *}, terminal::CommandExecutor};


pub struct Editor {
    rows: Vec<String>
}

impl Editor {
    pub fn new(rows: Vec<String>) -> Self {
        Self { rows }
    }

    pub fn run(&self) -> io::Result<()> {
        Editor::open()?;
        Editor::refresh(&self.rows)?;
        Editor::event_loop()?;
        Editor::close()
    }

    fn event_loop() -> io::Result<()> {
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
                _ => {}
            }
        }
    }

    fn refresh(rows: &Vec<String>) -> io::Result<()> {
        let mut commands = vec![Command::Clear];

        for (y, row) in rows.iter().enumerate() {
            commands.push(Command::MoveTo(1, y as u16 + 1));
            commands.push(Command::Print(row.to_string()));
        }
        commands.execute()
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

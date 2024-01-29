use std::io::{self, Write};

use crossterm::{cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp}, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, queue, style::Print, terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};


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
        let mut stdout = io::stdout();
        loop {
            match event::read()? {
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, .. }) => match code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Right =>  { execute!(stdout, MoveRight(1))?; }
                    KeyCode::Left =>  { execute!(stdout, MoveLeft(1))?; }
                    KeyCode::Up =>  { execute!(stdout, MoveUp(1))?; }
                    KeyCode::Down =>  { execute!(stdout, MoveDown(1))?; }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn refresh(rows: &Vec<String>) -> io::Result<()> {
        let mut stdout = io::stdout();
        queue!(stdout, Clear(ClearType::All))?;

        for (y, row) in rows.iter().enumerate() {
            queue!(stdout,
                MoveTo(0, y as u16),
                Print(row)
            )?;
        }
        stdout.flush()
    }

    fn open() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)
    }

    fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)
    }
}

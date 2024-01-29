use std::io::{self};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp}, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, style::Print, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}
};


fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    let file_content = "This is the hardcoded file content.\nIt will be displayed on the alternate screen.";
    execute!(stdout,
        EnterAlternateScreen,
        MoveTo(0, 0),
        Print(file_content),
        MoveTo(0, 0),
    )?;

    loop {
        match event::read()? {
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code, .. }) => match code {
                KeyCode::Esc => break,
                KeyCode::Right =>  { execute!(stdout, MoveRight(1))?; }
                KeyCode::Left =>  { execute!(stdout, MoveLeft(1))?; }
                KeyCode::Up =>  { execute!(stdout, MoveUp(1))?; }
                KeyCode::Down =>  { execute!(stdout, MoveDown(1))?; }
                _ => {}
            },
            _ => {}
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

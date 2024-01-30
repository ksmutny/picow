mod ansi;
pub mod commands;
mod winapi;

use std::io::{self, Write};

use ansi::ansi;
use commands::Command;


pub trait CommandExecutor {
    fn queue(self) -> io::Result<()>;
    fn execute(self) -> io::Result<()>;
}

impl CommandExecutor for Command {
    fn execute(self) -> io::Result<()> {
        self.queue()?;
        flush()
    }

    fn queue(self) -> io::Result<()> {
        let mut stdout = std::io::stdout();
        write!(stdout, "{}", ansi(self))
    }
}

impl CommandExecutor for Vec<Command> {
    fn queue(self) -> io::Result<()> {
        for command in self {
            command.queue()?;
        }
        Ok(())
    }
    fn execute(self) -> io::Result<()> {
        for command in self {
            command.execute()?;
        }
        Ok(())
    }
}

pub fn flush() -> io::Result<()> {
    io::stdout().flush()
}

pub fn enable_raw_mode() -> io::Result<()> {
    winapi::enable_raw_mode()
}

pub fn disable_raw_mode() -> io::Result<()> {
    winapi::disable_raw_mode()
}

type Coordinates = (u16, u16);

pub fn terminal_size() -> io::Result<Coordinates> {
    winapi::terminal_size()
}

pub fn cursor_position() -> io::Result<Coordinates> {
    winapi::cursor_position()
}

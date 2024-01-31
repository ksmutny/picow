mod ansi;
pub mod commands;
mod winapi;

use std::io::{self, Write};

use ansi::ansi;
use commands::Command;

pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn queue(&mut self, command: Command) {
        self.commands.push(command);
    }

    fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn execute(&mut self) -> io::Result<()> {
        self.commands.execute()?;
        self.clear();
        Ok(())
    }
}

pub trait CommandExecutor {
    fn queue(&self) -> io::Result<()>;
    fn execute(&self) -> io::Result<()>;
}

impl CommandExecutor for Command {
    fn execute(&self) -> io::Result<()> {
        self.queue()?;
        flush()
    }

    fn queue(&self) -> io::Result<()> {
        let mut stdout = std::io::stdout();
        write!(stdout, "{}", ansi(&self))
    }
}

impl CommandExecutor for Vec<Command> {
    fn queue(&self) -> io::Result<()> {
        for command in self {
            command.queue()?;
        }
        Ok(())
    }
    fn execute(&self) -> io::Result<()> {
        self.queue()?;
        flush()
    }
}

fn flush() -> io::Result<()> {
    io::stdout().flush()
}

pub fn enable_raw_mode() -> io::Result<()> {
    winapi::enable_raw_mode()
}

pub fn disable_raw_mode() -> io::Result<()> {
    winapi::disable_raw_mode()
}

pub type Coordinates = (u16, u16);

pub fn terminal_size() -> io::Result<Coordinates> {
    winapi::terminal_size()
}

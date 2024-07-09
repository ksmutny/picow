use std::io::{self, Write};

use super::ansi_out::ansi;
use super::commands::Command;


pub trait CommandExecutor {
    fn queue(&self) -> io::Result<()>;

    fn execute(&self) -> io::Result<()> {
        self.queue()?;
        io::stdout().flush()
    }
}

impl CommandExecutor for Command {
    fn queue(&self) -> io::Result<()> {
        write!(std::io::stdout(), "{}", ansi(&self))
    }
}

impl CommandExecutor for Vec<Command> {
    fn queue(&self) -> io::Result<()> {
        for command in self {
            command.queue()?;
        }
        Ok(())
    }
}

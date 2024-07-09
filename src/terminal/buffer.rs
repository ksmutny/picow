use std::io::{self, Write};

use super::ansi_out::ansi;
use super::commands::Command;


pub trait CommandBuffer {
    fn queue(&self) -> io::Result<()>;

    fn execute(&self) -> io::Result<()> {
        self.queue()?;
        io::stdout().flush()
    }
}

impl CommandBuffer for Command {
    fn queue(&self) -> io::Result<()> {
        write!(std::io::stdout(), "{}", ansi(&self))
    }
}

impl CommandBuffer for Vec<Command> {
    fn queue(&self) -> io::Result<()> {
        for command in self {
            command.queue()?;
        }
        Ok(())
    }
}

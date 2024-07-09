use std::io::{self, Write};

use super::ansi_out::ansi;
use super::commands::Command;


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

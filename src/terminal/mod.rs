mod ansi_in;
mod ansi_out;
mod buffer;
mod winapi;

pub mod commands;
pub mod events;
pub mod reader;

use std::io;
use commands::Command;

use self::{buffer::CommandBuffer, commands::Command::*};


pub fn on_alternate_screen(window_title: &str, run: impl FnOnce() -> io::Result<()>) -> io::Result<()> {
    let console_mode = init_alternate_screen(window_title)?;
    run()?;
    close_alternate_screen(console_mode)
}

fn init_alternate_screen(window_title: &str) -> io::Result<u32> {
    let orig_console_mode = winapi::init_console()?;

    vec!(
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste,
        SetWindowTitle(window_title.to_owned())
    ).execute()?;

    Ok(orig_console_mode)
}

fn close_alternate_screen(console_mode: u32) -> io::Result<()> {
    vec!(
        DisableBracketedPaste,
        DisableMouseCapture,
        LeaveAlternateScreen
    ).execute()?;

    winapi::restore_console_mode(console_mode)
}

pub type Coordinates = (u16, u16);

pub fn terminal_size() -> io::Result<Coordinates> {
    winapi::terminal_size()
}

pub fn output(commands: Vec<Command>) -> io::Result<()> {
    commands.execute()
}

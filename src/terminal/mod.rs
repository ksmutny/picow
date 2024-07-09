mod ansi_in;
mod ansi_out;
mod buffer;
mod reader;
mod winapi;

pub mod commands;
pub mod events;

use std::io;

use self::{buffer::CommandBuffer, commands::Command::{self, *}, events::Event};


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


pub fn terminal_size() -> io::Result<(u16, u16)> {
    winapi::terminal_size()
}

pub fn read_event() -> io::Result<Event> {
    reader::read_event()
}

pub fn output(commands: Vec<Command>) -> io::Result<()> {
    commands.execute()
}

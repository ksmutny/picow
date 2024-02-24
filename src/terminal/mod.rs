pub mod ansi_in;
mod ansi_out;
pub mod buffer;
pub mod commands;
pub mod events;
pub mod reader;
mod winapi;

use std::io;


pub fn init() -> io::Result<u32> {
    winapi::init_console()
}

pub fn restore_console_mode(console_mode: u32) -> io::Result<()> {
        winapi::restore_console_mode(console_mode)
}

pub type Coordinates = (u16, u16);

pub fn terminal_size() -> io::Result<Coordinates> {
    winapi::terminal_size()
}

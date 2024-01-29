use std::io::{self, Error};

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, ENABLE_PROCESSED_INPUT, ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT};


pub fn enable_raw_mode() -> io::Result<()> {
    modify_console_mode(|mode| mode & !(ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT))
}

pub fn disable_raw_mode() -> io::Result<()> {
    modify_console_mode(|mode| mode | (ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT))
}

// https://learn.microsoft.com/en-us/windows/console/setconsolemode
fn modify_console_mode<F>(modify: F) -> io::Result<()> where F: FnOnce(u32) -> u32 {
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(Error::last_os_error());
        }

        mode = modify(mode);

        if SetConsoleMode(handle, mode) == 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(())
}


pub fn terminal_size() -> Result<(u16, u16), Error> {
    with_console_info(|info| {
        let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;
        (width, height)
    })
}

pub fn cursor_position() -> Result<(u16, u16), Error> {
    with_console_info(|info| {
        let x = info.dwCursorPosition.X as u16 + 1;
        let y = info.dwCursorPosition.Y as u16 + 1;
        (x, y)
    })
}

fn with_console_info<T, F>(extract: F) -> Result<T, Error> where F: FnOnce(CONSOLE_SCREEN_BUFFER_INFO) -> T {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info = CONSOLE_SCREEN_BUFFER_INFO::default();

        if GetConsoleScreenBufferInfo(handle, &mut info) == 0 {
            return Err(Error::last_os_error());
        }

        Ok(extract(info))
    }
}

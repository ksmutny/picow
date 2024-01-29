use std::io::{self, Error};

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;
use winapi::um::wincon::{ENABLE_PROCESSED_INPUT, ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT};


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

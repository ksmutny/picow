use std::io::{self, Error};

use winapi::um::{
    consoleapi::SetConsoleMode,
    handleapi::INVALID_HANDLE_VALUE,
    processenv::GetStdHandle,
    winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, ENABLE_AUTO_POSITION, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_WINDOW_INPUT},
    winnt::HANDLE
};


const CONSOLE_MODE: u32 =
    // ENABLE_PROCESSED_INPUT |
    // ENABLE_LINE_INPUT |
    // ENABLE_ECHO_INPUT |
    ENABLE_WINDOW_INPUT |
    ENABLE_MOUSE_INPUT |
    ENABLE_AUTO_POSITION |
    ENABLE_VIRTUAL_TERMINAL_INPUT;

pub fn init_console() -> io::Result<()> {
    let handle = get_std_in_handle()?;
    set_console_mode(handle, CONSOLE_MODE)
}

pub fn set_console_mode(handle: HANDLE, mode: u32) -> io::Result<()> {
    unsafe {
        result(SetConsoleMode(handle, mode), 0, ())
    }
}

pub fn get_std_in_handle() -> io::Result<HANDLE> {
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        result(handle, INVALID_HANDLE_VALUE, handle)
    }
}

fn result<T: std::cmp::PartialEq, U>(value: T, err_value: T, ret_value: U) -> io::Result<U> {
    if value == err_value {
        Err(Error::last_os_error())
    } else {
        Ok(ret_value)
    }
}


pub fn terminal_size() -> Result<(u16, u16), Error> {
    with_console_info(|info| {
        let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;
        (width, height)
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

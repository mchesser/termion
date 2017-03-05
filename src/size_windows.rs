use std::io;
use std::mem;

use winapi;
use kernel32;

/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    let handle = unsafe { kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE) };
    if handle == winapi::INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    let mut buffer_info: winapi::wincon::CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem::zeroed() };
    if unsafe { kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) } == 0 {
        return Err(io::Error::last_os_error());
    }

    Ok((buffer_info.dwSize.X as u16, buffer_info.dwSize.Y as u16))
}

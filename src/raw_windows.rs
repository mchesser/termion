//! Managing raw mode.
//!
//! Raw mode is a particular state a TTY can have. It signifies that:
//!
//! 1. No line buffering (the input is given byte-by-byte).
//! 2. The input is not written out, instead it has to be done manually by the programmer.
//! 3. The output is not canonicalized (for example, `\n` means "go one line down", not "line
//!    break").
//!
//! It is essential to design terminal programs.
//!
//! # Example
//!
//! ```rust,no_run
//! use termion::raw::IntoRawMode;
//! use std::io::{Write, stdout};
//!
//! fn main() {
//!     let mut stdout = stdout().into_raw_mode().unwrap();
//!
//!     write!(stdout, "Hey there.").unwrap();
//! }
//! ```

use std::io::{self, Write};
use std::ops;

use winapi;
use winapi::wincon::*;

use kernel32;

const ENABLE_VIRTUAL_TERMINAL_PROCESSING: winapi::DWORD = 0x0004;
const DISABLE_NEWLINE_AUTO_RETURN: winapi::DWORD = 0x0008;
const ENABLE_VIRTUAL_TERMINAL_INPUT: winapi::DWORD = 0x0200;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
///
/// Restoring will entirely bring back the old TTY state.
pub struct RawTerminal<W: Write> {
    output_prev: winapi::DWORD,
    input_prev: winapi::DWORD,
    output: W,
}

impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        if let Ok(handle) = get_std_handle(winapi::STD_OUTPUT_HANDLE) {
            set_console_mode(handle, self.output_prev).unwrap();
        }

        if let Ok(handle) = get_std_handle(winapi::STD_INPUT_HANDLE) {
            set_console_mode(handle, self.input_prev).unwrap();
        }
    }
}

impl<W: Write> ops::Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write> ops::DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

impl<W: Write> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
///
/// (Unfortunately, this is not true on Windows, different flags need to be set for both the input
/// and output handles.)
pub trait IntoRawMode: Write + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W {
    fn into_raw_mode(mut self) -> io::Result<RawTerminal<W>> {
        let output_prev = try!(enable_vt_mode_output());
        let input_prev = try!(enable_vt_mode_input());

        Ok(RawTerminal {
            output_prev: output_prev,
            input_prev: input_prev,
            output: self
        })
    }
}

/// Enables VT mode on the 
pub fn enable_vt_mode_output() -> io::Result<winapi::DWORD> {
    let handle = try!(get_std_handle(winapi::STD_OUTPUT_HANDLE));
    
    let console_mode = try!(get_console_mode(handle));
    let new_console_mode = console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | 
        DISABLE_NEWLINE_AUTO_RETURN | ENABLE_PROCESSED_OUTPUT;

    try!(set_console_mode(handle, new_console_mode));

    Ok(console_mode)
}

pub fn enable_vt_mode_input() -> io::Result<winapi::DWORD> {
    let handle = try!(get_std_handle(winapi::STD_INPUT_HANDLE));

    let mut console_mode = try!(get_console_mode(handle));
    
    let mut new_console_mode = console_mode & !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT);
    new_console_mode |= ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_WINDOW_INPUT;

    try!(set_console_mode(handle, new_console_mode));

    Ok(console_mode)
}

fn get_std_handle(handle_type: winapi::DWORD) -> io::Result<winapi::HANDLE> {
    let handle = unsafe { kernel32::GetStdHandle(handle_type) };

    if handle == winapi::INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    Ok(handle)
}

fn get_console_mode(handle: winapi::HANDLE) -> io::Result<winapi::DWORD> {
    let mut console_mode = 0;
    
    if unsafe { kernel32::GetConsoleMode(handle, &mut console_mode) } == 0 {
        return Err(io::Error::last_os_error());
    }
    
    Ok(console_mode)
}

fn set_console_mode(handle: winapi::HANDLE, console_mode: winapi::DWORD) -> io::Result<()> {
    if unsafe { kernel32::SetConsoleMode(handle, console_mode) } == 0 {
        return Err(io::Error::last_os_error());
    }
    
    Ok(())
}


//! Termion is a pure Rust, bindless library for low-level handling, manipulating
//! and reading information about terminals. This provides a full-featured
//! alternative to Termbox.
//!
//! Termion aims to be simple and yet expressive. It is bindless, meaning that it
//! is not a front-end to some other library (e.g., ncurses or termbox), but a
//! standalone library directly talking to the TTY.
//!
//! Supports Redox, Mac OS X, and Linux (or, in general, ANSI terminals).
//!
//! For more information refer to the [README](https://github.com/ticki/termion).
#![warn(missing_docs)]

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

#[cfg(not(any(target_os = "redox", windows)))]
mod termios;

mod async;
pub use async::{AsyncReader, async_stdin};


#[cfg(not(windows))]
mod size;

#[cfg(windows)]
#[path = "size_windows.rs"]
mod size;

pub use size::terminal_size;

#[cfg(not(windows))]
mod tty;

#[cfg(windows)]
#[path = "tty_windows.rs"]
mod tty;

pub use tty::{is_tty, get_tty};

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod cursor;
pub mod event;
pub mod input;

#[cfg(not(windows))]
pub mod raw;

#[cfg(windows)]
#[path = "raw_windows.rs"]
pub mod raw;

pub mod screen;
pub mod scroll;
pub mod style;

use std::io;

/// This will panic.
pub fn is_tty(_stream: ()) -> bool {
    unimplemented!();
}

/// This will panic.
pub fn get_tty() -> io::Result<()> {
    unimplemented!()
}

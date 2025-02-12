use core::fmt::{self, Write};

use crate::io::{write_buf, STDOUT_FILENO};

pub struct StdoutFmt;

impl Write for StdoutFmt {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }
        let written = write_buf(STDOUT_FILENO, s.as_bytes()).map_err(|_| fmt::Error)?;
        assert_eq!(written, s.len());
        Ok(())
    }
}

pub macro print($($arg:tt)*) {
    let mut stdout_fmt = $crate::fmt::StdoutFmt;
    let _ = core::fmt::Write::write_fmt(&mut stdout_fmt, format_args!($($arg)*))
    .expect("failed printing to stdout");
}

pub macro println {
    () => {
        $crate::fmt::print!("\n")
    },
    ($($arg:tt)*) => {
        $crate::fmt::print!("{}\n", format_args!($($arg)*))
    }
}

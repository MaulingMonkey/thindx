use std::io::{self, Write};



/// Replaces all `\n`s with `\r\n`s on windows
pub struct EolRewriter<W: Write>(pub W);

impl<W: Write> Write for EolRewriter<W> {
    #[cfg(not(windows))] fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[cfg(windows)] fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let mut total_written = 0;
        while let Some(eol) = buf.iter().copied().position(|b| b == b'\n') {
            let written = self.0.write(&buf[..eol])?;
            total_written += written;
            if written < eol { return Ok(total_written); }
            self.0.write_all(b"\r\n")?;
            total_written += 1;
            buf = &buf[eol+1..];
        }
        Ok(total_written + self.0.write(buf)?)
    }

    fn flush(&mut self) -> io::Result<()> { self.0.flush() }
}

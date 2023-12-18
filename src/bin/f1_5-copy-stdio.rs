use std::io;
use std::io::{Error, Read, Write};

fn main() -> io::Result<()> {
    let mut buf: [u8; 1] = [0; 1];

    loop {
        let bytes_read = io::stdin()
            .read(&mut buf)
            .map_err(|e| Error::new(e.kind(), "input error"))?;
        if bytes_read == 0 {
            break;
        }

        let bytes_written = io::stdout()
            .write(&buf)
            .map_err(|e| Error::new(e.kind(), "output error"))?;
        if bytes_written == 0 {
            break;
        }
    }

    Ok(())
}

use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(b"HTTP/1.1 200 OK\ncontent-type: text/plain\n\nhello world")?;
    Ok(())
}

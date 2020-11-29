/// Client does drawing to terminal.
use crate::Color;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

/// Start client server, take control of event loop.
pub fn start(server: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("connecting to {}", server);
    let mut stream = TcpStream::connect(server)?;
    println!("connected");

    let mut color = [0; 8];
    stream.read(&mut color).unwrap();
    let color: Option<Color> = bincode::deserialize(&color).unwrap();
    let color = color.expect("server full");
    println!("joined as {:?}", color);
    let fgcolor = color.to_term_color();

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin();

    loop {
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        let mut key_bytes = [0];
        stdin.read(&mut key_bytes)?;

        write!(stdout, "{}{}", fgcolor, "o")?;

        if b"qkwjshald".contains(&key_bytes[0]) {
            stream.write_all(&key_bytes)?;
            if key_bytes[0] == b'q' {
                return Ok(());
            }
        }
    }
}

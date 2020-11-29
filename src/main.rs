use argh::FromArgs;
use snake::Color;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

/// Snake multiplayer.
#[derive(FromArgs)]
pub struct Args {
    /// connect to server IP and port
    #[argh(option, short = 'c')]
    connect: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: Args = argh::from_env();

    let (tx, mut rx) = oneshot::channel();

    // start a server if not connecting to other servers
    if arg.connect.is_none() {
        thread::spawn(|| snake::server::start(tx));

        // wait for signal to start
        loop {
            match rx.try_recv() {
                Ok(_) => break,
                Err(TryRecvError::Empty) => {}
                Err(e) => return Err(Box::new(e)),
            }
            std::sync::atomic::spin_loop_hint();
        }
    }

    let server = arg.connect.clone().unwrap_or("127.0.0.1:2345".to_string());
    println!("connecting");
    let mut stream = TcpStream::connect(server)?;
    println!("connected");

    let mut color = [0; 8];
    stream.read(&mut color).unwrap();
    let color: Option<Color> = bincode::deserialize(&color).unwrap();
    let color = color.expect("server full");

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin();

    println!("{:?}", color);

    loop {
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        let mut key_bytes = [0];
        stdin.read(&mut key_bytes)?;

        if b"qkwjshald".contains(&key_bytes[0]) {
            stream.write_all(&key_bytes)?;
            if key_bytes[0] == b'q' {
                return Ok(());
            }
        }
    }
}

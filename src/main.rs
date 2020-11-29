use argh::FromArgs;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::sync::Arc;
use std::thread;
use std::{collections::VecDeque, sync::Mutex};
use termion::{color, cursor};
use termion::raw::IntoRawMode;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::oneshot;

/// Snake multiplayer.
#[derive(FromArgs)]
struct Args {
    /// connect to server IP and port
    #[argh(option, short = 'c')]
    connect: Option<String>,
}

/// Game state passed arough through network.
#[derive(Default, Serialize, Deserialize)]
struct GameState {
    snakes: Vec<Snake>,
}

/// Point.
type Point = (u8, u8);

// Direction.
#[derive(Serialize, Deserialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Color.
#[derive(Debug, Serialize, Deserialize)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn to_term_color(&self) -> Box<dyn color::Color> {
        match self {
            Color::Red => Box::new(color::Red),
            Color::Green => Box::new(color::Green),
            Color::Blue => Box::new(color::Blue),
        }
    }
}

/// Snake.
#[derive(Serialize, Deserialize)]
struct Snake {
    color: Color,
    direction: Direction,
    body: VecDeque<Point>,
    /// Turns for the snake to grow.
    shed_in: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: Args = argh::from_env();

    let (tx, rx) = oneshot::channel();

    let server = arg.connect.clone().unwrap_or("127.0.0.1:10212".to_string());
    let client_handle = tokio::task::spawn(async move {
        let stdout = io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let mut stdin = termion::async_stdin();

        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        // wait for signal to start
        rx.await.unwrap();

        let mut stream = TcpStream::connect(server).await.unwrap();

        let mut color = [0];
        stream.read_exact(&mut color).await.unwrap();
        let color: Color = bincode::deserialize(&color).expect("server full");

        println!("{:?}", color);

        loop {
            let mut key_bytes = [0];
            stdin.read(&mut key_bytes).unwrap();

            if b"qkwjshald".contains(&key_bytes[0]) {
                stream.write_all(&key_bytes).await.unwrap();
                if key_bytes[0] == b'q' {
                    return;
                }
            }
        }
    });

    // start a server if not connecting to other servers
    if arg.connect.is_none() {
        let server_handle = tokio::spawn(async move {
            println!("server");
            let listener = TcpListener::bind("127.0.0.1:10212").await.unwrap();
            let mut game = GameState::default();
            // let colors = Rc::new(vec![Color::Red, Color::Blue, Color::Green]);
            let colors: Arc<Mutex<Vec<Color>>> = Arc::new(Mutex::new(vec![]));

            // good to start client after server is started
            tx.send(()).unwrap();

            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let colors = colors.clone();

                tokio::spawn(async move {
                    let mut key_bytes = [0];

                    // send a color on connect if available
                    let color = colors.lock().unwrap().pop();
                    let color = bincode::serialize(&color).unwrap();
                    if socket.write_all(&color).await.is_err() {
                        return;
                    }

                    loop {
                        match socket.read(&mut key_bytes).await {
                            Ok(0) => {
                                // destroy snake

                                return;
                            }
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("failed to read from socket; err = {:?}", e);
                                return;
                            }
                        }
                    }
                });
            }
        });

        tokio::try_join!(server_handle, client_handle)?;
    } else {
        tx.send(()).unwrap();
        client_handle.await?;
    }

    Ok(())
}

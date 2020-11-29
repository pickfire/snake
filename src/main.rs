use argh::FromArgs;
use std::thread;
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

    // start a server if not connecting to other servers
    if arg.connect.is_none() {
        let (tx, mut rx) = oneshot::channel();
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

    let server = arg.connect.unwrap_or("127.0.0.1:2345".to_string());
    snake::client::start(&server)
}

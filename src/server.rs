/// Server handles backend logic and timing.
use crate::{Color, GameState};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::oneshot::Sender;

#[tokio::main]
pub async fn start(tx: Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
    let _game = GameState::default();
    let colors = Arc::new(Mutex::new(vec![Color::Red, Color::Blue, Color::Green]));

    tx.send(()).unwrap();
    println!("listening");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("connected {:?}", socket);
        let colors = colors.clone();

        tokio::spawn(async move {
            let mut key_bytes = [0];

            // send a color on connect if available
            let color = colors.lock().unwrap().pop();
            let color = bincode::serialize(&color).unwrap();
            if socket.write_all(&color).await.is_err() {
                return;
            }
            println!("sent {:?}", color);

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
}

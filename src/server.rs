use crate::{Color, GameState};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::oneshot::Sender;
use tokio::prelude::*;

pub async fn start(tx: Sender<()>) {
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
        let _game = GameState::default();
        // let colors = Rc::new(vec![Color::Red, Color::Blue, Color::Green]);
        let colors: Arc<Mutex<Vec<Color>>> = Arc::new(Mutex::new(vec![]));

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
}


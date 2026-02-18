use std::{error::Error, sync::Arc};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod store;
use store::{Command, Storage, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store = Arc::new(Store::new());
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;

        let store = Arc::clone(&store);

        println!("New client connected: {:?}", addr);
        tokio::spawn(async move { handle_client(socket, store).await });
    }
}

async fn handle_client(socket: TcpStream, store: Arc<Store<String>>) {
    let mut reader = BufReader::new(socket);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let command = Command::from_str(&line);
                let response = match command {
                    Command::Get(key) => store.get(&key).unwrap_or_else(|| "(nil)".to_string()),
                    Command::Set(key, val) => {
                        store.set(key, val);
                        "OK".to_string()
                    }
                    Command::Del(key) => {
                        if store.delete(&key) {
                            "(integer) 1".to_string()
                        } else {
                            "(integer) 0".to_string()
                        }
                    }
                    Command::Exists(key) => {
                        if store.get(&key).is_some() {
                            "(integer) 1".to_string()
                        } else {
                            "(integer) 0".to_string()
                        }
                    },
                    Command::Ping => "PONG".to_string(),
                    Command::Unknown => "ERR unknown command".to_string(),
                };

                if let Err(e) = reader
                    .get_mut()
                    .write_all(format!("{}\n", response).as_bytes())
                    .await
                {
                    eprintln!("Failed to write to socket: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}

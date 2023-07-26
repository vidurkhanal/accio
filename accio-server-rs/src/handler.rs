use crate::utils;
use crate::Command;
use crate::DataAccessLayer;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

pub struct Handler {
    pub connection: Connection,
    pub db: DataAccessLayer,
    pub shutdown: Shutdown,

    _shutdown_complete: mpsc::Sender<()>,
}

pub struct Connection {
    pub stream: tokio::net::TcpStream,
}

pub struct Shutdown {
    shutdown: bool,
    nofify: broadcast::Receiver<()>,
}

impl Connection {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read_buf_data(&mut self) -> Option<Command> {
        let mut buf = [0; 1024];

        match self.stream.read(&mut buf).await {
            Ok(size) => {
                if size == 0 {
                    return None;
                }
            }
            Err(e) => {
                println!("[error] Failed to read data with error: {err}", err = e);
                return None;
            }
        }
        let args = utils::buffer_to_vec(&mut buf);
        let command = Command::from_buffer(args);
        Some(command)
        // utils::process_query(command, &mut tcp_stream, &mut self.listener.db).await;
    }
}

impl Shutdown {
    pub fn new(shutdown: bool, nofify: broadcast::Receiver<()>) -> Self {
        Shutdown { shutdown, nofify }
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub async fn listen_recv(&mut self) -> Result<(), tokio::sync::broadcast::error::RecvError> {
        self.nofify.recv().await?;
        self.shutdown = true;
        Ok(())
    }
}

impl Handler {
    pub fn new(listener: &mut crate::Listener, socket: tokio::net::TcpStream) -> Self {
        Handler {
            connection: Connection::new(socket),
            db: listener.db.clone(),
            shutdown: Shutdown::new(false, listener.notify_shutdown.subscribe()),
            _shutdown_complete: listener.shutdown_complete_tx.clone(),
        }
    }

    pub async fn process_query(&mut self, command: Command) {
        let tcp_stream = &mut self.connection.stream;
        let db = &mut self.db;

        match command {
            Command::Ping => match tcp_stream.write(b"Pong").await {
                Ok(_) => {
                    println!("[info] Sent data: Pong");
                }

                Err(e) => {
                    println!("[error] Failed to send data with error: {err}", err = e);
                }
            },

            Command::Get(key) => match db.get(key) {
                Some(value) => match tcp_stream.write(value.as_bytes()).await {
                    Ok(_) => {
                        println!("[info] Sent data: {}", value);
                    }
                    Err(e) => {
                        println!("[error] Failed to send data with error: {err}", err = e);
                    }
                },
                None => match tcp_stream.write(b"Key not found").await {
                    Ok(_) => {
                        println!("[info] Sent data: Key not found");
                    }
                    Err(e) => {
                        println!("[error] Failed to send data with error: {err}", err = e);
                    }
                },
            },

            Command::Set(key, value) => {
                db.set(key, value);
                match tcp_stream.write(b"Key set").await {
                    Ok(_) => {
                        println!("[info] Sent data: Key set");
                    }
                    Err(e) => {
                        println!("[error] Failed to send data with error: {err}", err = e);
                    }
                }
            }

            Command::Del(key) => {
                db.del(key);
                match tcp_stream.write(b"Key deleted").await {
                    Ok(_) => {
                        println!("[info] Sent data: Key deleted");
                    }
                    Err(e) => {
                        println!("[error] Failed to send data with error: {err}", err = e);
                    }
                }
            }

            Command::Invalid => match tcp_stream.write(b"Invalid command").await {
                Ok(_) => {
                    println!("[info] Sent data: Invalid command");
                }
                Err(e) => {
                    println!("[error] Failed to send data with error: {err}", err = e);
                }
            },
        }
    }
}

use crate::{cli::CLI, command::Command};
use std::io::Result;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Client {
    pub addr: String,
}

impl Client {
    pub fn new(addr: String) -> Client {
        Client { addr }
    }

    pub async fn run(&self, args: CLI) -> Result<()> {
        let mut stream = TcpStream::connect(&self.addr).await?;
        match args.command {
            Command::Ping => {
                stream.write_all("ping".as_bytes()).await?;
                let mut buf = [0; 4];
                stream.read(&mut buf).await?;
                println!("Ping response: {}", String::from_utf8_lossy(&buf));
            }
            Command::Get { key } => {
                stream.write_all(format!("get {}", key).as_bytes()).await?;
                let mut buf = [0; 1024];
                stream.read(&mut buf).await?;
                println!("Get response: {}", String::from_utf8_lossy(&buf));
            }
            Command::Set { key, val } => {
                stream
                    .write_all(format!("set {} {}", key, val).as_bytes())
                    .await?;
                let mut buf = [0; 1024];
                stream.read(&mut buf).await?;
                println!("Set response: {}", String::from_utf8_lossy(&buf));
            }
            Command::Del { key } => {
                stream.write_all(format!("del {}", key).as_bytes()).await?;
                let mut buf = [0; 1024];
                stream.read(&mut buf).await?;
                println!("Del response: {}", String::from_utf8_lossy(&buf));
            }
            Command::Invalid => {
                println!("Invalid command");
            }
        }

        Ok(())
    }
}

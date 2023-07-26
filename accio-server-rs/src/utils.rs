use tokio::io::AsyncWriteExt;

use crate::{Command, DataAccessLayer};

pub fn buffer_to_vec(buf: &mut [u8]) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();

    for &byte in buf.iter() {
        if byte.is_ascii_whitespace() || byte == 0 {
            if !word.is_empty() {
                words.push(word.clone());
                word.clear();
            }
        } else {
            word.push(byte as char);
        }
    }

    words
}

pub async fn process_query(
    command: Command,
    tcp_stream: &mut tokio::net::TcpStream,
    db: &mut DataAccessLayer,
) {
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

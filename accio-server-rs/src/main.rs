mod commands;
mod data_access_layer;
mod handler;
mod listener;
mod server;
mod utils;

pub use commands::Command;
pub use data_access_layer::DataAccessLayer;
pub use handler::Handler;
pub use listener::Listener;
use server::Server;
use std::io::Error;
use std::result::Result;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "[::1]:55001";
    println!("[info] Trying to setup a server at {}", addr);
    let tcp_listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            println!(
                "[error] Failed to bind to {addr} with error: {err}",
                addr = addr,
                err = e
            );
            return Err(e.into());
        }
    };
    println!(
        "[info] Successfully setup a server at {}\n[info] Waiting for TCP Connections",
        addr
    );
    let db = DataAccessLayer::new();
    let shutdown = signal::ctrl_c();
    let (notify_shutdown, _) = tokio::sync::broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = tokio::sync::mpsc::channel(1);
    let mut l = Listener {
        db,
        tcp_listener,
        notify_shutdown,
        shutdown_complete_rx,
        shutdown_complete_tx,
    };
    let mut accio_server = Server::new();

    tokio::select! {
        res = accio_server.run(&mut l) => {
            println!("[error] Server stopped with error: {:?}", res);
        }
        _ = shutdown => {
            println!("\n[info] Server shutting down");
        }
    }
    drop(l.notify_shutdown);
    drop(l.shutdown_complete_tx);

    let _ = l.shutdown_complete_rx;

    println!("[info] Server shutdown complete");

    Ok(())
}

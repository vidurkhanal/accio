use crate::Listener;
use std::io::Error;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub async fn run(&mut self, listener: &mut Listener) -> Result<(), Error> {
        loop {
            let socket = match listener.tcp_listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    println!(
                        "[error] Failed to accept connection with error: {err}",
                        err = e
                    );
                    continue;
                }
            };
            println!("[info] Connection Established for: {}", socket.1);
            let mut handler = crate::Handler::new(listener, socket.0);

            tokio::spawn(async move {
                if let Err(e) = Server::process_method(&mut handler).await {
                    println!(
                        "[error] Failed to process method with error: {err}",
                        err = e
                    );
                }
            });

            // let mut buf = [0; 1024];
            // match tcp_stream.read(&mut buf).await {
            //     Ok(_) => {
            //         let args = utils::buffer_to_vec(&mut buf);
            //         let command = Command::from_buffer(args);
            //         utils::process_query(command, &mut tcp_stream, &mut self.listener.db).await;
            //     }
            //     Err(e) => {
            //         println!("[error] Failed to read data with error: {err}", err = e);
            //     }
            // }
        }
    }

    pub async fn process_method(handler: &mut crate::Handler) -> Result<(), std::io::Error> {
        while !handler.shutdown.is_shutdown() {
            let result = tokio::select! {
                _ = handler.shutdown.listen_recv() => {
                    return Ok(());
                },
                result = handler.connection.read_buf_data() => {
                    result
                }
            };

            let cmd = match result {
                Some(cmd) => cmd,
                None => return Ok(()),
            };

            handler.process_query(cmd).await;
        }
        Ok(())
    }
}

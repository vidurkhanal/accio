use crate::DataAccessLayer;

pub struct Listener {
    pub db: DataAccessLayer,
    pub tcp_listener: tokio::net::TcpListener,
    pub notify_shutdown: tokio::sync::broadcast::Sender<()>,
    pub shutdown_complete_rx: tokio::sync::mpsc::Receiver<()>,
    pub shutdown_complete_tx: tokio::sync::mpsc::Sender<()>,
}

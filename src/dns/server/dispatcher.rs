use crate::cli;
use tokio::net::UdpSocket;
use std::sync::Arc;

use super::responder;

pub async fn start() {
    // TODO: Add TCP implementation.
    let addr = cli::Args::get_params().get_bind();
    let socket: UdpSocket = UdpSocket::bind(addr.to_string()).await
        .expect(&format!("couldn't bind to address: {}", addr.to_string()));
    let arc_socket = Arc::new(socket);

    // Start on a thread to add tcp implementation?
    dispatch_udp_requests(arc_socket).await;
}

async fn dispatch_udp_requests(arc_socket: Arc<UdpSocket>) {
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let arc_socket_clone = Arc::clone(&arc_socket);

        let result = (*arc_socket_clone).recv_from(&mut buf);

        match result.await {
            Ok((bytes, client_address)) => {
                log::info!(
                    "Received connection from {} of length {}",
                    client_address,
                    bytes
                );
                log::debug!(
                    "Spawning thread to handle connection from {}",
                    client_address
                );
                let arc_socket_clone = Arc::clone(&arc_socket);
                tokio::spawn(async move {
                    responder::handle(buf, client_address, arc_socket_clone).await;
                });
            }
            Err(e) => {
                // Omit device busy error, as we force them.
                if e.raw_os_error().unwrap_or(0) != 11 {
                    log::error!("{}", e.to_string());
                }
            }
        }
    }
}

use std::sync::{Arc, Mutex};
use std::time::{Duration};
use std::thread;
use std::net::UdpSocket;
use crate::cli;

use super::responder;


pub fn start() {
    // TODO: Add TCP implementation.
    let addr = cli::Args::get_params().get_bind();
    let socket: UdpSocket = UdpSocket::bind(addr.to_string()).expect(&format!("couldn't bind to address: {}", addr.to_string()));
    socket.set_read_timeout(Some(Duration::from_micros(1))).unwrap();
    let arc_socket = Arc::new(Mutex::new(socket));

    // Start on a thread to add tcp implementation?
    dispatch_udp_requests(arc_socket);
}

fn dispatch_udp_requests(arc_socket: Arc<Mutex<UdpSocket>>) {
    loop {
        let mut buf = [0; 1024];
        let arc_socket_clone = Arc::clone(&arc_socket);
       
        let result = (*arc_socket_clone.lock().unwrap()).recv_from(&mut buf);
        drop(arc_socket_clone);
        
        match result {
            Ok((bytes, client_address)) => {
                log::info!("Received connection from {} of length {}", client_address.to_string(), bytes);
                log::debug!("Spawning thread to handle connection from {}", client_address.to_string());
                let arc_socket_clone = Arc::clone(&arc_socket);
                thread::spawn(move || {
                    responder::handle(buf, client_address, arc_socket_clone);
                });
            },
            Err(e) => {
                // Omit device busy error, as we force them.
                if e.raw_os_error().unwrap_or(0) != 11 {
                    log::error!("{}", e.to_string());
                }
            }
        }
        thread::sleep(Duration::from_nanos(100));
    }
}

use std::net::{SocketAddr, UdpSocket};
use std::sync::{Mutex, Arc};
use std::time::Instant;
use crate::dns::dto::{datagram::Datagram, header::{Header, RCODE}, enums::TYPE};
use crate::cli;

pub fn handle(buf: [u8; 1024], address: SocketAddr, socket: Arc<Mutex<UdpSocket>>) {
    let recv_time = Instant::now();
    let datagram = Datagram::unserialize(&buf);
    log::debug!("Received datagram from {}\n{}", address.to_string(), datagram.to_string());

    // TODO: Handle questions async?
    // or maybe prepare all questions asynchronously, then
    // merge them for a single request containing all requests.
    // for i in 0..datagram.header.qdcount as usize {
    let question = &datagram.questions[0];
    let reply: Datagram;

    // TODO: Should probably match opcode first.
    match question.qtype {
        TYPE::A | TYPE::AAAA => {
            reply = respond_question(datagram);
        },
        _ => {
            // Forward request as normal if function type not supported
            reply = forward_request(&datagram);
        }
    }
    (*socket).lock().unwrap().send_to(&reply.serialize(), address).expect(&format!("Couldn't reply to {}", &address));
    drop(socket);
    log::debug!("Sent reply to {} in {} ms", address, recv_time.elapsed().as_millis());
}

fn respond_question(datagram: Datagram) -> Datagram {
    let question = &datagram.questions[0];
    let header = &datagram.header;
    if crate::blocklist::file::is_blocked(&question.qname) {
        return Datagram{
            header: Header{
                id: header.id,
                qr: true,
                opcode: header.opcode.clone(),
                aa: false,
                tc: false,
                rd: header.rd,
                ra: true,
                z: false,
                ad: header.ad,
                cd: header.cd,
                rcode: RCODE::NXDomain,
                qdcount: 1,
                ancount: 0,
                nscount: 0,
                arcount: 0, 
            },
            questions: Box::new([question.clone()]),
            answers: Box::new([]),
            authorities: Box::new([]),
            additionals: Box::new([]),
        }
    } else {
        return forward_request(&datagram);
    }
}

fn forward_request(datagram: &Datagram) -> Datagram {
    // TODO: Add TCP capabilities logic
    let upstream_addr: SocketAddr = cli::Args::get_params().get_upstream();
    let client_socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't create a receiving socket");
    client_socket.connect(upstream_addr).expect(&format!("Couldn't connect to {}", upstream_addr));
    client_socket.send(&datagram.serialize()).expect(&format!("Couldn't send message to {}", upstream_addr));
    let send_time = Instant::now();
    log::debug!("Forwarded request to {}", upstream_addr);

    let mut buf = [0;1024];
    client_socket.recv(&mut buf).expect(&format!("Couldn't receive message from {}", upstream_addr));
    
    let receiving_delay = send_time.elapsed().as_millis();
    let reply = Datagram::unserialize(&buf);
    log::debug!("Received reply from {} in {} ms\n{}", upstream_addr, receiving_delay, reply.to_string());
    return reply;
}
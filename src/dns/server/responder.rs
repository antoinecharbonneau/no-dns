use crate::cli;
use crate::dns::cache::Cache;
use crate::dns::dto::{
    datagram::Datagram,
    enums::TYPE,
    header::{Header, RCODE},
};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn handle(buf: [u8; 1024], address: SocketAddr, socket: Arc<Mutex<UdpSocket>>) {
    let recv_time = Instant::now();
    let datagram = Datagram::unserialize(&buf);
    log::debug!("Received datagram from {}\n{}", address, datagram);

    // TODO: Handle questions async?
    // or maybe prepare all questions asynchronously, then
    // merge them for a single request containing all requests.
    // for i in 0..datagram.header.qdcount as usize {
    let question = &datagram.questions[0];
    let reply: Datagram;

    // TODO: Should probably match opcode first.
    match question.qtype {
        TYPE::A | TYPE::AAAA => {
            reply = respond_question(&datagram, &address);
        }
        _ => {
            // Forward request as normal if function type not supported
            reply = get_forwarded_answer(&datagram).unwrap();
        }
    }
    (*socket)
        .lock()
        .unwrap()
        .send_to(&reply.serialize(), address)
        .expect(&format!("Couldn't reply to {}", &address));
    drop(socket);
    log::debug!(
        "Sent reply to {} in {} ms",
        address,
        recv_time.elapsed().as_millis()
    );
}

fn respond_question(datagram: &Datagram, address: &SocketAddr) -> Datagram {
    if let Some(blocked_answer) = get_blocked_answer(datagram) {
        log::info!(
            "Blocked {} for {}",
            datagram.questions.get(0).unwrap().qname,
            address
        );
        return blocked_answer;
    }
    if let Some(cached_answer) = get_cached_answer(datagram) {
        log::debug!(
            "Cache hit on {} for {}",
            datagram.questions.get(0).unwrap().qname,
            address
        );
        return cached_answer;
    }
    if let Some(forwarded_answer) = get_forwarded_answer(datagram) {
        log::debug!(
            "Forwarded {} request for {}",
            datagram.questions.get(0).unwrap().qname,
            address
        );
        return forwarded_answer;
    } else {
        log::error!("Couldn't connect to upstream.");
        return empty_answer(datagram);
    }
}

fn get_blocked_answer(datagram: &Datagram) -> Option<Datagram> {
    // TODO: Add address so that blocking can be done on a per address basis ?
    let question = &datagram.questions[0];
    if crate::blocklist::file::is_blocked(&question.qname) {
        return Some(empty_answer(&datagram));
    } else {
        return None;
    }
}

fn get_cached_answer(datagram: &Datagram) -> Option<Datagram> {
    let question = &datagram.questions[0];
    let header = &datagram.header;
    let cache_result = Cache::get(question);
    match cache_result {
        Some(answer) => {
            return Some(Datagram {
                header: Header {
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
                    rcode: RCODE::NoError,
                    qdcount: 1,
                    ancount: 1,
                    nscount: 0,
                    arcount: 0,
                },
                questions: Box::new([question.clone()]),
                answers: Box::new([answer]),
                authorities: Box::new([]),
                additionals: Box::new([]),
            })
        }
        None => return None,
    }
}

fn get_forwarded_answer(datagram: &Datagram) -> Option<Datagram> {
    // TODO: Add TCP capabilities logic
    let upstream_addr: SocketAddr = cli::Args::get_params().get_upstream();
    let client_socket: UdpSocket =
        UdpSocket::bind("0.0.0.0:0").expect("Couldn't create a receiving socket");
    client_socket
        .connect(upstream_addr)
        .expect(&format!("Couldn't connect to {}", upstream_addr));
    client_socket
        .send(&datagram.serialize())
        .expect(&format!("Couldn't send message to {}", upstream_addr));
    let send_time = Instant::now();
    log::debug!("Forwarded request to {}", upstream_addr);

    let mut buf = [0; 1024];
    client_socket
        .recv(&mut buf)
        .expect(&format!("Couldn't receive message from {}", upstream_addr));

    let receiving_delay = send_time.elapsed().as_millis();
    let reply = Datagram::unserialize(&buf);

    for i in 0..reply.header.ancount as usize {
        let answer = reply.answers.get(i)?;

        Cache::insert(&answer.get_question(), answer.clone());
    }

    log::debug!(
        "Received reply from {} in {} ms\n{}",
        upstream_addr,
        receiving_delay,
        reply
    );
    return Some(reply);
}

fn empty_answer(datagram: &Datagram) -> Datagram {
    Datagram {
        header: Header {
            id: datagram.header.id,
            qr: true,
            opcode: datagram.header.opcode.clone(),
            aa: false,
            tc: false,
            rd: datagram.header.rd,
            ra: true,
            z: false,
            ad: datagram.header.ad,
            cd: datagram.header.cd,
            rcode: RCODE::NXDomain,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        },
        questions: Box::new([datagram.questions.get(0).unwrap().clone()]),
        answers: Box::new([]),
        authorities: Box::new([]),
        additionals: Box::new([]),
    }
}

use crate::cli;
use crate::dns::cache as Cache;
use crate::dns::dto::{
    datagram::Datagram,
    enums::TYPE,
    header::RCODE,
};
use std::net::{SocketAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Instant;
use tokio::net::UdpSocket;

pub async fn handle(buf: [u8; 1024], address: SocketAddr, socket: Arc<UdpSocket>) {
    let recv_time = Instant::now();
    let datagram = Datagram::unserialize(&buf);
    log::debug!("Rcvd pkt from {}\n{}", address, datagram);

    // TODO: Handle questions async?
    // or maybe prepare all questions asynchronously, then
    // merge them for a single request containing all requests.
    // for i in 0..datagram.header.qdcount as usize {
    let question = &datagram.questions[0];
    let reply: Datagram;

    // TODO: Should probably match opcode first.
    match question.get_type() {
        TYPE::A | TYPE::AAAA => {
            reply = respond_question(&datagram, &address).await;
        }
        _ => {
            // Forward request as normal if function type not supported
            reply = get_forwarded_answer(&datagram).await.unwrap();
        }
    }

    match socket.send_to(&reply.serialize(), address).await {
        Ok(_) => {
            log::debug!(
                "Sent reply to {} in {} ms",
                address,
                recv_time.elapsed().as_millis()
            )
        },
        Err(_) => {
            log::warn!(
                "Couldn't reply to {}",
                address
            )
        }
    }
}

async fn respond_question(datagram: &Datagram, address: &SocketAddr) -> Datagram {
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
    if let Some(forwarded_answer) = get_forwarded_answer(datagram).await {
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
    let mut header = datagram.header.clone();
    header.set_question(false);
    header.set_recursion_available(true);
    header.set_authoritative_answer(false);
    header.set_truncated(false);
    header.set_answer_count(1);
    header.set_authority_count(0);
    header.set_additional_count(0);
    let cache_result = Cache::get(question);
    match cache_result {
        Some(answer) => {
            return Some(Datagram {
                header,
                questions: vec![question.clone()],
                answers: vec![answer],
                authorities: vec![],
                additionals: vec![],
            })
        }
        None => return None,
    }
}

static DEFAULT_SOCKET: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

async fn get_forwarded_answer(datagram: &Datagram) -> Option<Datagram> {
    // TODO: Add TCP capabilities logic
    let upstream_addr: SocketAddr = cli::Args::get_params().get_upstream();
    let client_socket: UdpSocket =
        UdpSocket::bind(DEFAULT_SOCKET).await.expect("Couldn't create a receiving socket");
    client_socket
        .connect(upstream_addr).await
        .expect(&format!("Couldn't connect to {}", upstream_addr));
    client_socket
        .send(&datagram.clone().serialize()).await
        .expect(&format!("Couldn't send message to {}", upstream_addr));
    let send_time = Instant::now();
    log::debug!("Forwarded request to {}", upstream_addr);

    let mut buf = [0; 1024];
    client_socket
        .recv(&mut buf).await
        .expect(&format!("Couldn't receive message from {}", upstream_addr));

    let receiving_delay = send_time.elapsed().as_millis();
    let reply = Datagram::unserialize(&buf);

    for i in 0..reply.header.answer_count() as usize {
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
    let mut header = datagram.header.clone();
    header.set_question(false);
    header.set_authoritative_answer(false);
    header.set_truncated(false);
    header.set_recursion_available(true);
    header.set_rcode(RCODE::NXDomain);
    header.set_answer_count(0);
    header.set_authority_count(0);
    header.set_additional_count(0);
    Datagram {
        header,
        questions: vec![datagram.questions[0].clone()],
        answers: vec![],
        authorities: vec![],
        additionals: vec![],
    }
}

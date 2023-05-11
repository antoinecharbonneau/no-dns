use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "NO DistractioNS")]
#[command(author = "Antoine Charbonneau <antoine@charbonneau.dev>")]
#[command(about = "NO DistractioNS: A DNS Proxy against distractions")]
#[command(version, long_about = None)]
pub struct Args {
    /// Filename containing a packet of a dns request.
    #[arg(short, long, default_value_t = String::from("blocklist.txt"))]
    pub file: String,

    /// Socket address to bind the application to.
    #[arg(short, long, default_value_t = String::from("0.0.0.0:1053"))]
    pub bind: String,

    /// Upstream DNS server
    #[arg(short, long, default_value_t = String::from("8.8.8.8:53"))]
    pub upstream: String
}

impl Args {
    pub fn get_params() -> Args {
        return Args::parse();
    }

    pub fn get_bind(&self) -> SocketAddr {
        return self.bind.parse()
        .expect("Unable to parse the bind socket address");
    }

    pub fn get_upstream(&self) -> SocketAddr {
        return self.upstream.parse()
        .expect("Unable to parse the upstream socket address.");
    }
}
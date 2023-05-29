use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "No DNS")]
#[command(author = "Antoine Charbonneau <antoine@charbonneau.dev>")]
#[command(about = "No DNS (No DistractioNS): A multithreaded DNS Proxy against distractions")]
#[command(version, long_about = None)]
pub struct Args {
    /// File path to the blocklist.
    #[arg(short, long, default_value_t = String::from("blocklist.txt"))]
    pub file: String,

    /// Socket address to bind the application to.
    #[arg(short, long, default_value_t = String::from("0.0.0.0:53"))]
    pub bind: String,

    /// Upstream DNS server IP
    #[arg(short, long, default_value_t = String::from("8.8.8.8"))]
    pub upstream: String,

    /// Upstream DNS server port
    #[arg(long, default_value_t = 53)]
    pub upstream_port: u16,
}

impl Args {
    pub fn get_params() -> Args {
        return Args::parse();
    }

    pub fn get_bind(&self) -> SocketAddr {
        return self
            .bind
            .parse()
            .expect("Unable to parse the bind socket address");
    }

    pub fn get_upstream(&self) -> SocketAddr {
        return format!("{}:{}", self.upstream, self.upstream_port)
            .parse()
            .expect("Unable to parse the upstream socket address");
    }
}

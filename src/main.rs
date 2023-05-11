mod dns;
mod cli;
mod blocklist;
use log::info;
use env_logger;
use dns::server;

fn main() {
    env_logger::init();
    info!("Starting NO DistractioNS");
    server::dispatcher::start();
}

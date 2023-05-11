mod dns;
mod cli;
mod blocklist;
use env_logger;
use dns::server;

fn main() {
    env_logger::init();
    log::info!("Starting No DNS on {}", cli::Args::get_params().get_bind().to_string());
    server::dispatcher::start();
}

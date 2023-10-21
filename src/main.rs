mod blocklist;
mod cli;
mod dns;
use dns::server;
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting No DNS on {}", cli::Args::get_params().get_bind());
    server::dispatcher::start().await;
}

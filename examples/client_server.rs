#[path = "./client.rs"]
mod client;

use client::client;

use stun_server::{error::Result, server::server};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // start the server in a separate thread
    tokio::spawn(async { server().await });

    client().await
}

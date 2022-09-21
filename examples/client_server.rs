#[path = "./client.rs"]
mod client;

#[path = "./server.rs"]
mod server;

use client::client;
use server::server;

use stun_server::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // start the server in a separate thread
    tokio::spawn(async { server().await });

    client().await
}

mod client;
mod config;
mod error;
mod message;
mod server;
mod utils;

use crate::client::client;
use crate::error::{Error, Result};
use crate::server::server;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run the binary as a client or a server
    #[clap(short, long)]
    r#type: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();

    match args.r#type.as_ref() {
        "server" => server().await,
        "client" => client().await,
        _ => Err(Error::Arguments(args.r#type)),
    }
}

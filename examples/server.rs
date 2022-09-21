use stun_server::{error::Result, server::server};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    server().await
}

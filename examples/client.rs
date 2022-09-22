use std::net::SocketAddr;

use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;

use stun_server::{
    config::CONFIG,
    error::{Error, Result},
    message::Message,
};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    client().await
}

pub async fn client() -> Result<()> {
    let client_addr: SocketAddr = (*CONFIG).client.parse()?;
    let server_addr: SocketAddr = (*CONFIG).server.parse()?;
    let socket = UdpSocket::bind(client_addr)
        .await
        .map_err(|e| Error::Startup(e.to_string()))?;

    log::info!(
        "Started stun client on {}, connected to a stun server on {}",
        client_addr,
        server_addr
    );

    let message = Message::binding_request(vec![]);

    log::info!("sending binding request to the server: {:?}", message);

    // encode the binding request
    let mut bytes_mut = BytesMut::new();
    message.encode(&mut bytes_mut);

    // send the encoded binding request to the server
    socket
        .send_to(bytes_mut.as_ref(), server_addr)
        .await
        .map_err(|e| Error::BindingRequest(e.to_string()))?;

    // listen for a response
    loop {
        let mut buf = [0u8; 1024];
        let (bytes_received, address) = socket
            .recv_from(&mut buf)
            .await
            .map_err(|e| Error::Receive(e.to_string()))?;
        let mut bytes = Bytes::copy_from_slice(&buf);
        let message = Message::decode(&mut bytes).map_err(|e| Error::Decode(e.to_string()))?;

        log::info!(
            "received {} bytes from {}: {:?}",
            bytes_received,
            address,
            message
        );
    }
}

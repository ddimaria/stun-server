use bytes::{Bytes, BytesMut};
use std::net::SocketAddr;
use stun_server::{
    config::CONFIG,
    error::{Error, Result},
    message::attribute::Attribute,
    message::class::Class,
    message::message::Message,
    message::method::Method,
    utils::Address,
};
use tokio::net::UdpSocket;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    server().await
}

pub async fn server() -> Result<()> {
    let server_addr: SocketAddr = (*CONFIG).server.parse()?;
    let socket = UdpSocket::bind(server_addr)
        .await
        .map_err(|e| Error::Startup(e.to_string()))?;

    log::info!("Started stun server on {}", server_addr);

    let mut buf = [0u8; 1024];

    loop {
        let (bytes_received, client_address) = socket
            .recv_from(&mut buf)
            .await
            .map_err(|e| Error::Receive(e.to_string()))?;
        let mut bytes = Bytes::copy_from_slice(&buf);
        let message = Message::decode(&mut bytes)?;

        log::info!(
            "received {} bytes from {}: {:?}",
            bytes_received,
            client_address,
            message
        );

        match (message.class, message.method) {
            (Class::Request, Method::Binding) => {
                let message = Message::binding_response(vec![Attribute::XorMappedAddress(
                    Address::parse_address(client_address),
                )]);

                log::info!("sending message to client: {:?}", message);

                // encode the binding response
                let mut buf = BytesMut::new();
                message.encode(&mut buf);

                // send the encoded binding response to the client
                socket
                    .send_to(&mut buf.as_ref(), client_address)
                    .await
                    .map_err(|e| Error::BindingResponse(e.to_string()))?;
            }
            _ => unimplemented!("This service is only setup to receive a binding request message"),
        }
    }
}

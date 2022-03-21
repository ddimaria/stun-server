use crate::{
    config::CONFIG,
    error::{Error, Result},
    message::attribute::Attribute,
    message::class::Class,
    message::message::Message,
    message::method::Method,
    utils::Address,
};
use bytes::{Bytes, BytesMut};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub(crate) async fn server() -> Result<()> {
    let addr: SocketAddr = (*CONFIG).server.parse()?;
    let socket = UdpSocket::bind(addr)
        .await
        .map_err(|e| Error::Startup(e.to_string()))?;

    log::info!("Started stun server on {}", addr);

    let mut buf = [0u8; 1024];

    loop {
        let (bytes_received, address) = socket
            .recv_from(&mut buf)
            .await
            .map_err(|e| Error::Receive(e.to_string()))?;
        let mut bytes = Bytes::copy_from_slice(&buf);
        let message = Message::decode(&mut bytes)?;

        log::info!(
            "received {} bytes from {}: {:?}",
            bytes_received,
            address,
            message
        );

        match (message.class, message.method) {
            (Class::Request, Method::Binding) => {
                let message = Message::binding_response(vec![Attribute::XorMappedAddress(
                    Address::get_address(address),
                )]);

                log::info!("sending message to client: {:?}", message);

                let mut buf = BytesMut::new();
                message.encode(&mut buf);

                // send the encoded binding response to the client
                socket
                    .send_to(&mut buf.as_ref(), address)
                    .await
                    .map_err(|e| Error::BindingResponse(e.to_string()))?;
            }
            _ => unimplemented!("This service is only setup to receive a binding request message"),
        }
    }
}

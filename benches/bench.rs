use criterion::{criterion_group, criterion_main, Criterion};
use std::net::SocketAddr;

use bytes::BytesMut;
use tokio::net::UdpSocket;

use stun_server::{config::CONFIG, error::Error, message::Message, server::server};

async fn setup_server() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // start the server in a separate thread
    tokio::spawn(async { server().await });
}

async fn setup_client() -> (UdpSocket, BytesMut) {
    let client_addr: SocketAddr = (*CONFIG).client.parse().unwrap();
    let socket = UdpSocket::bind(client_addr)
        .await
        .map_err(|e| Error::Startup(e.to_string()))
        .unwrap();

    let message = Message::binding_request(vec![]);

    // encode the binding request
    let mut bytes_mut = BytesMut::new();
    message.encode(&mut bytes_mut);

    (socket, bytes_mut)
}

// send the encoded binding request to the server
async fn send_binding_request(socket: &UdpSocket, bytes: &BytesMut) {
    let server_addr: SocketAddr = (*CONFIG).server.parse().unwrap();
    socket
        .send_to(bytes.as_ref(), server_addr)
        .await
        .map_err(|e| Error::BindingRequest(e.to_string()))
        .unwrap();
}

fn bench(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        setup_server().await;
    });
    let (socket, bytes) = rt.block_on(async move { setup_client().await });

    c.bench_function("send-and-receive-binding-request-and-response", move |b| {
        b.to_async(&rt)
            .iter(|| async { send_binding_request(&socket, &bytes) })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

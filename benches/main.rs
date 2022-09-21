// use criterion::async_executor::async_tokio;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{net::SocketAddr, sync::Arc};

use bytes::BytesMut;
use tokio::{net::UdpSocket, sync::Mutex};

use stun_server::{config::CONFIG, error::Error, message::message::Message, server::server};

static client_addr: SocketAddr = (*CONFIG).client.parse().unwrap();
static server_addr: SocketAddr = (*CONFIG).server.parse().unwrap();

async fn setup_server() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // start the server in a separate thread
    tokio::spawn(async { server().await });
}

async fn setup_client() -> (UdpSocket, BytesMut) {
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
async fn send_binding_request(socket: Arc<Mutex<UdpSocket>>, bytes: BytesMut) {
    socket
        .send_to(bytes.as_ref(), server_addr)
        .await
        .map_err(|e| Error::BindingRequest(e.to_string()))
        .unwrap();
}

async fn criterion_benchmark(c: &mut Criterion) {
    setup_server().await;
    let (socket, bytes) = setup_client().await;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let socket = Arc::new(Mutex::new(socket));

    c.bench_function("send/receive binding request/response", |b| {
        b.to_async(&rt).iter(|| async move {
            send_binding_request(black_box(socket.clone()), black_box(bytes));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

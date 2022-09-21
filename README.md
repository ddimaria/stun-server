# STUN Client and Server
Session Traversal Utilities for NAT (STUN) is a protocol to discover a client's public ip address and determine any restrictions in the client's router/firewall that would prevent a direct peer connection.  This implementation only focuses on the Binding Request and Binding Response portion.

This STUN server receives _Binding Request_ messages, validates them, and replies with a _Binding Response_ message.  This STUN client sends _Binding Request_ messages and cosumes/decodes a _Binding Response_ from the server.

This is primarily a teaching tool for Rust systems programming (UDP, header encoding/decoding, ...etc.) in the WebRTC domain.

## Examples

### Configuration
Copy the .env.example file to .env

```shell
cp .env.example .env
```

Now update the values in .env as needed.

### Running the Server
First, run the server, which will listen for incoming UDP packets and accept Binding Request messages:

```shell
RUST_LOG=info cargo run --example server
```

#### Output

```shell
INFO  server > Started stun server on 0.0.0.0:8082
```

### Running the Client
Next, run the client, which will instantly send a Binding Request message (UDP packet):

```shell
RUST_LOG=info cargo run --example client
```

#### Output

```shell
INFO  client > Started stun client on 0.0.0.0:8081, connected to a stun server on 0.0.0.0:8082
INFO  client > sending binding request to the server: Message { class: Request, method: Binding, transaction_id: TransactionId([216, 15, 139, 140, 54, 166, 55, 187, 63, 53, 116, 133]), attributes: [] }
```

### Running the Client and Server Examples
Next, run the client, which will instantly send a Binding Request message (UDP packet):

```shell
RUST_LOG=info cargo run --example client_server
```

#### Output

```shell
INFO  client_server::server > Started stun server on 0.0.0.0:8082
INFO  client_server::client > Started stun client on 0.0.0.0:8081, connected to a stun server on 0.0.0.0:8082
INFO  client_server::client > sending binding request to the server: Message { class: Request, method: Binding, transaction_id: TransactionId([208, 75, 17, 165, 14, 198, 154, 57, 125, 86, 149, 161]), attributes: [] }
INFO  client_server::server > received 20 bytes from 127.0.0.1:8081: Message { class: Request, method: Binding, transaction_id: TransactionId([208, 75, 17, 165, 14, 198, 154, 57, 125, 86, 149, 161]), attributes: [] }
INFO  client_server::server > sending message to client: Message { class: SuccessResponse, method: Binding, transaction_id: TransactionId([185, 55, 136, 17, 149, 163, 157, 110, 142, 158, 190, 150]), attributes: [XorMappedAddress(Address { address: [127, 0, 0, 1], port: 8081, ip_kind: IPv4 })] }
INFO  client_server::client > received 20 bytes from 127.0.0.1:8082: Message { class: SuccessResponse, method: Binding, transaction_id: TransactionId([185, 55, 136, 17, 149, 163, 157, 110, 142, 158, 190, 150]), attributes: [] }
```
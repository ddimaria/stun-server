# STUN Client and Server
Session Traversal Utilities for NAT (STUN) is a protocol to discover a client's public ip address and determine any restrictions in the client's router/firewall that would prevent a direct peer connection.  This implementation only focuses on the Binding Request and Binding Response portion.

This STUN server receives _Binding Request_ messages, validates them, and replies with a _Binding Response_ message.  This STUN client sends _Binding Request_ messages and cosumes/decodes a _Binding Response_ from the server.

This is primarily a teaching tool for Rust systems programming (UDP, header encoding/decoding, ...etc.) in the WebRTC domain.

## Configuration
Copy the .env.example file to .env

```shell
cp .env.example .env
```

Now update the values in .env as needed.

## Running the Server
First, run the server, which will listen for incoming UDP packets and accept Binding Request messages:

```shell
RUST_LOG=info cargo run -- --type server
```

## Running the Client
Next, run the client, which will instantly send a Binding Request message (UDP packet):

```shell
RUST_LOG=info cargo run -- --type client
```
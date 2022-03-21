# STUN Client and Server

## Configuration
Copy the .env.example file to .env

```shell
cp .env.example .env
```

Now update the values in .env

## Running the Client
To run the client:

```shell
RUST_LOG=info cargo run -- --type client
```

## Running the Server
To run the server:

```shell
RUST_LOG=info cargo run -- --type server
```
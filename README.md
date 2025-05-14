# Rust Chat Application

A simple chat application written in Rust using Tokio for asynchronous networking. This application allows multiple clients to connect to a server and exchange messages in real-time.

## Features

- Asynchronous networking using Tokio
- Multiple client support
- Real-time message broadcasting
- Clean connection handling
- User join/leave notifications

## Building

Make sure you have Rust installed on your system. Then, build the project:

```bash
cargo build --release
```

## Usage

### Running the Server

To start the chat server:

```bash
cargo run -- -s -a 127.0.0.1:8080
```

The `-s` flag indicates server mode, and `-a` specifies the address to bind to.

### Running the Client

To start a chat client:

```bash
cargo run -- -a 127.0.0.1:8080
```

The `-a` flag specifies the server address to connect to.

## How to Use

1. Start the server first
2. Start multiple clients in different terminal windows
3. When a client connects, it will be prompted to enter a name
4. After entering the name, you can start sending messages
5. Messages will be broadcast to all connected clients
6. When a client disconnects, other clients will be notified

## Example Session

Server:
```bash
$ cargo run -- -s
Server running on 127.0.0.1:8080
New connection from: 127.0.0.1:54321
New connection from: 127.0.0.1:54322
```

Client 1:
```bash
$ cargo run -- -a 127.0.0.1:8080
Enter your name: Alice
Alice has joined the chat
Bob has joined the chat
Bob: Hello everyone!
Alice: Hi Bob!
```

Client 2:
```bash
$ cargo run -- -a 127.0.0.1:8080
Enter your name: Bob
Bob has joined the chat
Alice: Hi Bob!
Bob: Hello everyone!
```

## Note

This is a basic implementation and doesn't include features like:
- Message persistence
- Private messaging
- User authentication
- Message encryption

Feel free to extend the functionality as needed! 
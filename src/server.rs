use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

type Clients = Arc<Mutex<HashMap<String, tokio::net::tcp::OwnedWriteHalf>>>;

pub struct ChatServer {
    clients: Clients,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server running on {}", addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("New connection from: {}", addr);

            let clients = self.clients.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(socket, addr, clients).await {
                    eprintln!("Error handling client {}: {}", addr, e);
                }
            });
        }
    }

    async fn handle_client(
        socket: TcpStream,
        addr: std::net::SocketAddr,
        clients: Clients,
    ) -> Result<()> {
        let (reader, mut writer) = socket.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        // Get client name
        writer.write_all(b"Enter your name: ").await?;
        reader.read_line(&mut line).await?;
        let name = line.trim().to_string();
        line.clear();

        // Add client to the list
        let mut clients = clients.lock().await;
        clients.insert(name.clone(), writer);

        // Broadcast new user joined
        Self::broadcast_message(
            &mut clients,
            format!("{} has joined the chat", name).as_bytes(),
        )
        .await?;

        // Handle messages
        loop {
            line.clear();
            if reader.read_line(&mut line).await? == 0 {
                break;
            }

            let message = format!("{}: {}", name, line.trim());
            Self::broadcast_message(&mut clients, message.as_bytes()).await?;
        }

        // Remove client and broadcast departure
        clients.remove(&name);
        Self::broadcast_message(
            &mut clients,
            format!("{} has left the chat", name).as_bytes(),
        )
        .await?;

        Ok(())
    }

    async fn broadcast_message(
        clients: &mut HashMap<String, tokio::net::tcp::OwnedWriteHalf>,
        message: &[u8],
    ) -> Result<()> {
        let mut failed_clients = Vec::new();

        for (name, client) in clients.iter_mut() {
            if let Err(e) = client.write_all(message).await {
                eprintln!("Failed to send message to {}: {}", name, e);
                failed_clients.push(name.clone());
            }
        }

        // Remove failed clients
        for name in failed_clients {
            clients.remove(&name);
        }

        Ok(())
    }
} 
mod client;
mod server;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as server
    #[arg(short, long)]
    server: bool,

    /// Address to connect to (for client) or bind to (for server)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.server {
        let server = server::ChatServer::new();
        server.run(&args.addr).await?;
    } else {
        let mut client = client::ChatClient::connect(&args.addr).await?;
        client.run().await?;
    }

    Ok(())
} 
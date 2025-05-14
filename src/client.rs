use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct ChatClient {
    stream: TcpStream,
}

impl ChatClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream })
    }

    pub async fn run(self) -> Result<()> {
        // Split the stream into read and write halves
        let (read_half, write_half) = self.stream.into_split();
        let mut reader = BufReader::new(read_half);
        let mut writer = write_half;
        let mut line = String::new();

        // Read welcome message and name prompt
        reader.read_line(&mut line).await?;
        print!("{}", line);
        line.clear();

        // Get user input for name
        std::io::stdin().read_line(&mut line)?;
        writer.write_all(line.as_bytes()).await?;
        line.clear();

        // Spawn a task to handle incoming messages
        let mut reader = BufReader::new(reader);
        let handle = tokio::spawn(async move {
            let mut line = String::new();
            loop {
                line.clear();
                if reader.read_line(&mut line).await? == 0 {
                    break;
                }
                print!("{}", line);
            }
            Ok::<_, anyhow::Error>(())
        });

        // Handle outgoing messages
        loop {
            line.clear();
            if std::io::stdin().read_line(&mut line)? == 0 {
                break;
            }
            writer.write_all(line.as_bytes()).await?;
        }

        handle.abort();
        Ok(())
    }
} 
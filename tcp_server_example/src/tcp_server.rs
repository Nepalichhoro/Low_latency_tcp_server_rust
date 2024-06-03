use crate::commands::Command;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_server(addr: &str) -> io::Result<()> {
    let addr = addr.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await.unwrap();
        });
    }
}

async fn handle_client(socket: TcpStream) -> io::Result<()> {
    let (reader, writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).await?;

        if buffer.is_empty() {
            break;
        }

        let command: Command = match serde_json::from_str(&buffer) {
            Ok(cmd) => cmd,
            Err(_) => break,
        };

        let response = match command {
            Command::GetHelloWorld => "Hello, world!".to_string(),
            Command::PostGreeting { message } => format!("Received greeting: {}", message),
        };

        writer.write_all(response.as_bytes()).await?;
        writer.flush().await?;
    }

    Ok(())
}

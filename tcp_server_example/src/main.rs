mod commands;
mod tcp_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tcp_server::start_server("127.0.0.1:8080").await
}

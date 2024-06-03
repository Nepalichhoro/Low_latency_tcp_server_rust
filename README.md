# Low_latency_tcp_server_rust

## Asynchronous Server: 

- The server uses tokio::net::TcpListener to accept incoming connections. 
- Each connection is handled in a separate asynchronous task using tokio::spawn.


## Command Handling: 
- The server reads commands from the client, processes them, and sends a response.

## Asynchronous Client: 
- The client connects to the server, sends commands, and reads responses.
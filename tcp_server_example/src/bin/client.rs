use serde_json::json;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut writer = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);

    // Send GetHelloWorld command
    let get_hello = json!({"GetHelloWorld": {}}).to_string() + "\n";
    writer.write_all(get_hello.as_bytes())?;
    writer.flush()?;

    // Read the response
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Response: {}", response.trim());

    // Send PostGreeting command
    let post_greeting = json!({"PostGreeting": {"message": "Hello, Rust!"}}).to_string() + "\n";
    writer.write_all(post_greeting.as_bytes())?;
    writer.flush()?;

    // Read the response
    response.clear();
    reader.read_line(&mut response)?;
    println!("Response: {}", response.trim());

    Ok(())
}

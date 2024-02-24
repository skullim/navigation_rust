use std::net::TcpStream;
use std::io::Write;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server!");

            // Send data to the server
            let message = "Hello from the client!";
            stream.write_all(message.as_bytes()).expect("Failed to write data");
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
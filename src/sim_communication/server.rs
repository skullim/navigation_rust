use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");
    println!("Server listening on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {:?}", stream.peer_addr().unwrap());
                handle_client(&mut stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: &mut TcpStream) {
    
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by client
                    println!("Connection closed by client: {}", stream.peer_addr().unwrap());
                    break;
                }
                
                // Process received data
                let pose_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received pose data: {}", pose_data);
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
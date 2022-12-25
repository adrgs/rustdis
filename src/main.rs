// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::{Write, BufRead}};

fn handle_request(request: &str) -> String {
    if request.starts_with("PING") {
        return "+PONG\r\n".to_string();
    }
    return "-ERR unknown command\r\n".to_string();
}

fn handle_connection(stream: std::net::TcpStream) {
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let response = handle_request(&buffer);
    writer.write(response.as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap())
    }
}

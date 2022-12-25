use std::{net::TcpListener, io::{Write, BufRead, Read}};
use anyhow::Error;

fn handle_request(reader: &mut std::io::BufReader<&std::net::TcpStream>) -> Result<String, Error> {
    // client requests are encoded as Arrays in RESP

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let mut command: Vec<String> = Vec::new();

    if buffer.starts_with("*") {
        let num_args = buffer[1..].trim().parse::<i32>()?;
        for _ in 0..num_args {
            let mut buffer = String::new();
            reader.read_line(&mut buffer)?;
            if buffer.starts_with("+") {
                // deal with simple string
                command.push(buffer[1..].trim().to_string());
            } else if buffer.starts_with("$") {
                // deal with bulk string
                let str_len = buffer[1..].trim().parse::<i32>()?;
                let mut buffer = vec![0u8; str_len as usize];
                reader.read_exact(&mut buffer)?;

                command.push(String::from_utf8(buffer)?);
                // read the trailing CRLF
                let mut buffer = String::new();
                reader.read_line(&mut buffer)?;
            } else if buffer.starts_with(":") {
                // deal with integer
            } else if buffer.starts_with("*") {
                // deal with array
            } else if buffer.starts_with("-") {
                // deal with error
            } else {
                return Err(anyhow::anyhow!("Invalid request: {}", buffer));
            }
        }
    } else {
        return Err(anyhow::anyhow!("Invalid request: {}", buffer));
    }

    if command.len() > 0 {
        match command[0].as_str().to_lowercase().as_str() {
            "ping" => return Ok("+PONG\r\n".to_string()),
            &_ => todo!()
        }
    }

    return Ok("-ERR unknown command\r\n".to_string());
}

fn handle_connection(stream: std::net::TcpStream) -> Result<(), Error>{
    let mut reader = std::io::BufReader::new(&stream);
    let mut writer = std::io::BufWriter::new(&stream);

    let response = handle_request(&mut reader)?;
    writer.write(response.as_bytes())?;

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match handle_connection(stream.unwrap()) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e.to_string().trim())
        }
    }
}

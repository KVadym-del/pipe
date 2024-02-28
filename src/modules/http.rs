use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::path::PathBuf;

pub fn http_handle_client(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    let mut request_parts = request_line.trim().split_whitespace();

  if let Some(method) = request_parts.next() {
    if let Some(resource) = request_parts.next() {
      match method.to_uppercase().as_str() {
        "GET" => {
          println!("Request for {}", resource);
            loop {
                let mut line = String::new();
                buf_reader.read_line(&mut line).unwrap();
                if line.trim().is_empty() {
                    break;
                }
            }
            let mut path = PathBuf::new();
            path.push("src/resources/html");
            path.push(resource.trim_start_matches("/"));
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            stream.write_all(&fs::read(path).unwrap()).unwrap();
        },
        _ => {
          eprintln!("Unsupported HTTP method: {}", method);
        }
      }
    } else {
      eprintln!("Invalid request format");
    }
  } else {
    eprintln!("Invalid request line");
  }
}

use std::fs;
use std::path::PathBuf;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write, BufReader, BufRead };
use std::thread;

pub enum ServerType {
    TCP,
    HTTP,
}

pub fn client() {
    let mut username = String::new();
    print!("Enter your username: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).map_err(|e| {
        eprintln!("ERROR: Failed to read from stdin => {}", e);
    }).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:9998").map_err(|e| {
        eprintln!("ERROR: Failed to connect to port 9998 => {}", e);
    }).unwrap();

    stream.write(username.trim().as_bytes()).unwrap();

    println!("Successfully connected to port 9998");

    loop {
        let mut input = String::new();
        print!("Enter a message to send: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).map_err(|e| {
            eprintln!("ERROR: Failed to read from stdin => {}", e);
        }).unwrap();
        stream.write(input.as_bytes()).unwrap();

        if input.contains("exit") {
            println!("Terminating connection with server");
            break;
        }
    }
}

fn tcp_handle_client(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());

    let mut username = String::new();
    let mut msg_count = 0;
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(_) => {
                if msg_count == 0 {
                    username = String::from_utf8_lossy(&buf).to_string();
                    println!("{} has joined the chat", username);
                    msg_count += 1;
                    continue;
                }
                let msg = String::from_utf8_lossy(&buf);
                if msg.contains("exit") {   
                    println!("Terminating connection with {}", stream.peer_addr().unwrap());
                    break;
                }
                println!("{}: {}",username ,msg);
                msg_count += 1;
            }
            Err(e) => {
                eprintln!("ERROR: Failed to read from connection => {}", e);
                break;
            }
        }
    }
}

fn http_handle_client(mut stream: TcpStream) {
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
            path.push(""); // src/resources/html
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


pub fn server(server_type: ServerType) {
    let listener = TcpListener::bind("127.0.0.1:9998").map_err(|e| {
        eprintln!("ERROR: Failed to bind to port 9998 => {}", e);
    }).unwrap();

    println!("Server listening on port 9998");

    for stream in listener.incoming() {
        match server_type {
            ServerType::TCP => {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            tcp_handle_client(stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to establish a TCP connection => {}", e);
                    }
                }
            }
            ServerType::HTTP => {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            http_handle_client(stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to establish a HTTP connection => {}", e);
                    }
                }
            }
        }
    } 
}

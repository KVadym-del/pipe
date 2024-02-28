use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[path = "tcp.rs"] mod tcp;
#[path = "http.rs"] mod http;

pub enum ServerType {
    TCP,
    HTTP,
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
                            tcp::tcp_handle_client(stream);
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
                            http::http_handle_client(stream);
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


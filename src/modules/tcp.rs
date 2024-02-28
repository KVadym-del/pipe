use std::net::TcpStream;
use std::io::Read;

pub fn tcp_handle_client(mut stream: TcpStream) {
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
                    println!("Terminating connection with [{username} => {}]", stream.peer_addr().unwrap());
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

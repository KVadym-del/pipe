use std::env;

#[path = "modules/pipe.rs"] mod pipe;
// #[path = "modules/editor.rs"] mod editor;

fn main() {
    let mut args = env::args();
    
    args.next();
    let pipe_state = args.next().ok_or_else(|| {
        eprintln!("ERROR: no pipe state provided");
    });

    match pipe_state.unwrap().as_str() {
        "server" => {
            let server_type = args.next().ok_or_else(|| {
                eprintln!("ERROR: no server type provided");
            }).unwrap();
            match server_type.as_str() {
                "tcp" => {
                    pipe::server(pipe::ServerType::TCP);
                }
                "http" => {
                    pipe::server(pipe::ServerType::HTTP);
                }
                _ => {
                    // TODO: combine tcp and http server into one
                    eprintln!("ERROR: invalid server type\n\t=> USAGE: cargo run server [tcp|http]");
                }
            }
        }
        "client" => {
            pipe::client();
        }
        "editor" => {
            // TODO: implement editor
        }
        _ => {
            eprintln!("ERROR: invalid pipe state\n\t=> USAGE: cargo run [server|client]");
        }
    } 
}

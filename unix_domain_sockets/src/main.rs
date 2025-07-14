use unix_domain_sockets::server;
use unix_domain_sockets::client;
use std::sync::Arc;
use std::env;
use std::process::exit;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().map(|s| s).collect();
    if args.len() < 3 {
        eprintln!("Usage: {} client|server name", args[0]);
        exit(1);
    }

    let socket_path = "/tmp/my_unix_socket.sock";

    let mode = args[1].clone();
    let name = Arc::new(args[2].clone());
    match mode.as_str() {
        "client" => {
            client(socket_path, name).await;
        },
        "server" => {
            server(socket_path, name).await;
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'client' or 'server'.", mode);
        }
    }
}

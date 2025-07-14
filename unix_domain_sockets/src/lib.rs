use tokio::net::UnixStream;
use std::sync::Arc;
use std::io;
use std::path::Path;

pub async fn server(socket_path: &str, name: Arc<String>) {
    let name = name.to_string();
    let socket_path = Path::new(&socket_path);
    if socket_path.exists() {
        tokio::fs::remove_file(socket_path).await.expect("Failed to remove existing Unix socket");
    }
    let listener = tokio::net::UnixListener::bind(socket_path)
        .expect(&format!("{name}: failed to bind Unix socket"));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(server_loop(stream, name.clone().into()));
    };
}

async fn server_loop(stream: UnixStream, name: Arc<String>) {
    let mut buffer = [0u8; 1024];
    loop {
        match stream.try_read(&mut buffer[..]) {
            Ok(0) => {
                println!("Server ({name}): client disconnected itself");
                break;
            }
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[..n]);
                println!("Server ({name}): received: {data}");
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                eprintln!("Server) ({name}): Error reading from stream: {e}");
                break;
            }
        }
    }
}

pub async fn client(socket_path: &str, name: Arc<String>) {
    let unix_stream = UnixStream::connect(Path::new(&socket_path)).await.expect("Cannot connect to Unix socket");
    println!("Client connected to {socket_path:?}");

    let mut i = 0u16;
    loop {
        i %= 1000u16;
        i += 1;
        let msg = format!("Hello from client: {name}! # {i}\n");
        let data = msg.clone().into_bytes();
        match unix_stream.try_write(&data) {
            Ok(n) => {
                println!("Client ({name}): sent {n} bytes: {msg}");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }
                eprintln!("Client ({name}): Error writing to stream: {e}");
                break;
            }
        }
    }
}
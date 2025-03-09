use std::io;
use tokio::net::UdpSocket;

// Test on macOS: `echo -n "hello" | nc -4u -w1 localhost 8080`

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = vec![0u8; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("Received {:?} bytes from {:?}", len, addr);
        
        let len = sock.send_to(&buf[..len], addr).await?;
        println!("Sent {:?} bytes", len);
    }
}
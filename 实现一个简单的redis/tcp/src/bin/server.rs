// 举一个tokio官方的代码例子，这段代码实现了一个tcp server，会将客户端发来的数据原封不动的发回去：
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::error::Error;

// cargo run --bin server
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    loop {
        let (socket, socket_addr) = listener.accept().await?;
        println!("accept socket: {:?}", socket_addr);
        tokio::spawn(handler(socket));
    }
}

async fn handler(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];

    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            return;
        }
        println!("recv: {}", String::from_utf8_lossy(&buf[..n]).trim());
        socket
            .write_all(&buf[0..n])
            .await
            .expect("failed to write data to socket");
    }
}

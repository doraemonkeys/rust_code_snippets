use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// 单线程版本可以修改为多线程甚至于线程池来实现并发处理，
// 但是线程还是太重了，使用 async 实现 Web 服务器才是最适合的。
fn main() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 阻塞等待请求的进入
    // incoming 会返回一个迭代器，它每一次迭代都会返回一个新的连接 stream(客户端发起，web服务器监听接收)，
    // 因此，接下来要做的就是从 stream 中读取数据，然后返回处理后的结果。
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let res = handle_connection(stream);
        if res.is_err() {
            println!("Error: {:?}", res);
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // 从连接中顺序读取 1024 字节数据
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;

    println!("Request:\n {}", String::from_utf8_lossy(&buffer[..n]));

    let get = b"GET / HTTP/1.1\r\n";

    // 处理HTTP协议头，若不符合则返回404和对应的 `html` 文件
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename)?;

    // 将回复内容写入连接缓存中
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes())?;
    // 使用 flush 将缓存中的内容发送到客户端
    stream.flush()?;
    Ok(())
}

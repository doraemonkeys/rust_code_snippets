use async_std::net::TcpListener;
// use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::StreamExt;

// 之前的例子有一个致命的缺陷：只能使用一个线程并发的处理用户请求。
// 是的，这样也可以实现并发，一秒处理几千次请求问题不大，
// 但是这毕竟没有利用上 CPU 的多核并行能力，无法实现性能最大化。

// async 并发和多线程其实并不冲突，而 async-std 包也允许我们使用多个线程去处理，
// 由于 handle_connection 实现了 Send 特征且不会阻塞，
// 因此使用 async_std::task::spawn 是非常安全的。

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            // 至此，我们实现了同时使用并行(多线程)和并发( `async` )来同时处理多个请求！
            async_std::task::spawn(handle_connection(stream));
        })
        .await;
}

// async fn handle_connection(mut stream: TcpStream)
// 为了方便测试， 我们修改 handle_connection 的函数签名让测试更简单。
// 之所以可以修改签名，原因在于 async_std::net::TcpStream 实际上并不是必须的，
// 只要任何结构体实现了 async_std::io::Read, async_std::io::Write 和 marker::Unpin 就可以替代它。
use async_std::io::{Read, Write};
async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = std::fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

use futures::io::Error;
use futures::task::{Context, Poll};
use std::cmp::min;
use std::pin::Pin;
// 下面，来构建一个 mock 的 `TcpStream` 并实现了上面这些特征，
// 它包含一些数据，这些数据将被拷贝到 `read` 缓存中, 然后返回 `Poll::Ready` 说明 `read` 已经结束：
struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Poll::Ready(Ok(size))
    }
}
impl Write for MockTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);

    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;

    let expected_contents = std::fs::read_to_string("hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}

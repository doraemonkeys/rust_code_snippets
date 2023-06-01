#[tokio::main]
async fn main() {
    let stream = tokio::net::TcpStream::connect("127.0.0.1:8081")
        .await
        .unwrap();

    /*
       loop {
           let mut input = String::new();
           println!("please input msg: ");
           std::io::stdin().read_line(&mut input).unwrap();
           stream.write_all(input.as_bytes()).await.unwrap();
           let n = stream.read(&mut buf).await.unwrap();
           let msg = String::from_utf8_lossy(&buf[..n]);
           println!("recv: {}", msg.trim());
       }
    */
    // 上面的代码必须先输入，才能接收到服务端的消息。
    // 想要实现并发读写，显然，使用同一个 socket 是不行的，
    // 为了实现目标功能，必须将 `socket` 分离成一个读取器和写入器。

    // let (reader, mut writer) = tokio::io::split(stream);

    // 实际上，任何一个读写器( reader + writer )都可以使用 io::split 方法进行分离，最终返回一个读取器和写入器，
    // 这两者可以独自的使用，例如可以放入不同的任务中。
    // io::split 可以用于任何同时实现了 AsyncRead 和 AsyncWrite 的值，
    // 它的内部使用了 Arc 和 Mutex 来实现相应的功能。

    // 如果大家觉得这种实现有些重，可以使用 Tokio 提供的 `TcpStream`，它提供了两种方式进行分离:
    // - TcpStream::split 会获取字节流的引用，然后将其分离成一个读取器和写入器。
    //   但由于使用了引用的方式，它们俩必须和 `split` 在同一个任务中。
    //   优点就是，这种实现没有性能开销，因为无需 `Arc` 和 `Mutex`。
    // - TcpStream::into_split 还提供了一种分离实现，分离出来的结果可以在任务间移动，内部是通过 `Arc` 实现。

    // TcpStream::split
    let (mut reader, mut writer) = stream.into_split();

    tokio::spawn(async move {
        loop {
            use tokio::io::AsyncReadExt;
            let mut buf = vec![0; 1024];
            let n = reader.read(&mut buf).await;
            if n.is_err() {
                println!("read error: {:?}", n.err());
                return;
            }
            let n = n.unwrap();
            let msg = String::from_utf8_lossy(&buf[..n]);
            println!("recv: {}", msg.trim());
        }
    });

    loop {
        use tokio::io::AsyncWriteExt;
        let mut input = String::new();
        println!("please input msg: ");
        std::io::stdin().read_line(&mut input).unwrap();
        writer.write_all(input.as_bytes()).await.unwrap();
    }
}

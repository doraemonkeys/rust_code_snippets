use async_stream::stream;

use futures_util::pin_mut;
use futures_util::stream::StreamExt;

// 在实际场景中，迭代一个集合，然后异步的去执行是很常见的需求，
// 好在 Tokio 为我们提供了 stream，我们可以在异步函数中对其进行迭代，
// 甚至和迭代器 Iterator 一样，stream 还能使用适配器，例如 map !
// Tokio 在 StreamExt 特征上定义了常用的适配器。

// stream 没有放在 `tokio` 包的原因在于标准库中的 `Stream` 特征还没有稳定，
// 一旦稳定后，`stream` 将移动到 `tokio` 中来
#[tokio::main]
async fn main() {
    // tokio_stream
    study_tokio_stream().await;

    // async_stream
    study_async_stream().await;
}

async fn study_tokio_stream() {
    println!("------------------ study_tokio_stream ------------------");
    println!("------------------ example1 ------------------");
    async fn example1() {
        let mut stream = tokio_stream::iter(&[1, 2, 3]);

        // 和迭代器 Iterator 类似，next() 方法返回一个 Option<T>，
        // 其中 T 是从 stream 中获取的值的类型。若收到 None 则意味着 stream 迭代已经结束。
        while let Some(v) = stream.next().await {
            println!("GOT = {:?}", v);
        }
    }
    example1().await;
}

async fn study_async_stream() {
    println!("------------------ study_async_stream ------------------");

    println!("------------------ example1 ------------------");
    async fn example1() {
        let s = stream! {
            for i in 0..3 {
                yield i;
            }
        };
        pin_mut!(s); // needed for iteration

        while let Some(value) = s.next().await {
            println!("got {}", value);
        }
    }
    example1().await;

    println!("------------------ example2 ------------------");
    async fn example2() {
        //  returned by using :impl Stream<Item = T>
        use futures_core::stream::Stream;
        fn zero_to_three() -> impl Stream<Item = u32> {
            stream! {
                for i in 0..3 {
                    yield i;
                }
            }
        }

        let s = zero_to_three();
        pin_mut!(s); // needed for iteration

        while let Some(value) = s.next().await {
            println!("got {}", value);
        }
    }
    example2().await;

    println!("------------------ example3 ------------------");
    // Streams may be implemented in terms of other streams - provides syntax
    // to assist with this:async-stream `for await`
    async fn example3() {
        use futures_core::stream::Stream;
        fn zero_to_three() -> impl Stream<Item = u32> {
            stream! {
                for i in 0..3 {
                    yield i;
                }
            }
        }
        fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
            stream! {
                for await value in input {
                    yield value * 2;
                }
            }
        }
        let s = double(zero_to_three());
        pin_mut!(s); // needed for iteration

        while let Some(value) = s.next().await {
            println!("got {}", value);
        }
    }
    example3().await;

    println!("------------------ example4 ------------------");
    async fn _example4() {
        use async_stream::try_stream;
        use futures_core::stream::Stream;
        use std::io;
        use std::net::SocketAddr;
        use tokio::net::{TcpListener, TcpStream};

        fn bind_and_accept(addr: SocketAddr) -> impl Stream<Item = io::Result<TcpStream>> {
            try_stream! {
                let  listener = TcpListener::bind(addr).await?;

                loop {
                    let (stream, addr) = listener.accept().await?;
                    println!("received on {:?}", addr);
                    yield stream;
                }
            }
        }
        use std::net::{IpAddr, Ipv4Addr};
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7777);
        let s = bind_and_accept(socket);
        pin_mut!(s); // needed for iteration

        while let Some(value) = s.next().await {
            println!("got {:?}", value);
            if let Ok(stream) = value {
                println!("stream: {:?}", stream);
            }
        }
    }
}

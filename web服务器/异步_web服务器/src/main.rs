use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    // 下面的例子将演示如何使用一个异步运行时 async-std 来让之前的 async fn 函数运行起来，
    // 该运行时允许使用属性 #[async_std::main] 将我们的 fn main 函数变成 async fn main ，
    // 这样就可以在 main 函数中直接调用其它 async 函数，否则你得用之前章节的 block_on
    // 方法来让 main 去阻塞等待异步函数的完成，但是这种简单粗暴的阻塞等待方式并不灵活。
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    // listener.incoming() 是阻塞的迭代器。当 listener 在等待连接时，执行器是无法执行其它 Future 的，
    // 而且只有在我们处理完已有的连接后，才能接收新的连接。
    // 解决方法是将 listener.incoming() 从一个阻塞的迭代器变成一个非阻塞的 Stream。
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     // 警告，这里无法并发
    //     handle_connection(stream).await;
    // }

    // 在将数据读写改造成异步后，现在该函数也彻底变成了异步的版本，因此一次慢请求不再会阻止其它请求的运行。
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            handle_connection(tcpstream).await;
        })
        .await;
}

// 该修改会将函数的返回值从 () 变成 Future<Output=()> ，因此直接运行将不再有任何效果，
// 只用通过 .await 或执行器的 poll 调用后才能获取 Future 的结果。
async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    // 现在运行服务器，并访问 127.0.0.1:7878/sleep， 你会发现只有在完成第一个用户请求(5 秒后)，
    //  才能开始处理第二个用户请求 127.0.0.1:7878。现在再来看看该如何解决这个问题，让请求并发起来。
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 在内部睡眠 5 秒，模拟一次用户慢请求，需要注意的是，
        // 我们并没有使用 std::thread::sleep 进行睡眠，原因是该函数是阻塞的，
        // 它会让当前线程陷入睡眠中，导致其它任务无法继续运行！
        // 因此我们需要一个睡眠函数 async_std::task::sleep，
        // 它仅会让当前的任务陷入睡眠，然后该任务会让出线程的控制权，这样线程就可以继续运行其它任务。
        // 光把函数变成 async 往往是不够的，还需要将它内部的代码也都变成异步兼容的，阻塞线程绝对是不可行的。
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

// 在之前的代码中，我们使用了自己实现的简单的执行器来进行 .await 或 poll ，
// 实际上这只是为了学习原理，在实际项目中，需要选择一个三方的 async 运行时来实现相关的功能。
// 现在先选择 async-std ，该包的最大优点就是跟标准库的 API 类似，相对来说更简单易用。

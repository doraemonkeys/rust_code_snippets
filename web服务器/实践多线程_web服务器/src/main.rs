use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use practice_thread_web_server::ThreadPool;

// 线程池包含一组已生成的线程，它们时刻等待着接收并处理新的任务。
// 当程序接收到新任务时，它会将线程池中的一个线程指派给该任务，在该线程忙着处理时，
// 新来的任务会交给池中剩余的线程进行处理。
// 最终，当执行任务的线程处理完后，它会被重新放入到线程池中，准备处理新任务。

// 假设线程池中包含 N 个线程，那么可以推断出，服务器将拥有并发处理 N 个请求连接的能力，从而增加服务器的吞吐量。
// 同时，我们将限制线程池中的线程数量，以保护服务器免受拒绝服务攻击（DoS）的影响：
// 如果针对每个请求创建一个新线程，那么一个人向我们的服务器发出1000万个请求，会直接耗尽资源，
// 导致后续用户的请求无法被处理，这也是拒绝服务名称的来源。

// 因此，还需对线程池进行一定的架构设计，首先是设定最大线程数的上限，其次维护一个请求队列。
// 池中的线程去队列中依次弹出请求并处理。这样就可以同时并发处理 N 个请求，其中 N 是线程数。
// 但聪明的读者可能会想到，假如每个请求依然耗时很长，那请求队列依然会堆积，
// 后续的用户请求还是需要等待较长的时间，毕竟你也就 N 个线程，但总归比单线程要强 N 倍吧 :D
// 当然，线程池依然是较为传统的提升吞吐方法，
// 比较新的有：单线程异步 IO，例如 redis；多线程异步 IO，例如 Rust 的主流 web 框架。

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            let err = handle_connection(stream);
            if let Err(e) = err {
                println!("handle_connection error: {}", e);
            }
        });
    }
    println!("Shutting down.");
    // 即便主线程退出，只要子线程还在运行，程序就不会终止。
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().ok_or("bad request")??;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = std::fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}

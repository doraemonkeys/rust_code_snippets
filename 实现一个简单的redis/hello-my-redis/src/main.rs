use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Tokio 中大多数类型的名称都和标准库中对应的同步类型名称相同，而且，
    // 如果没有特殊原因，Tokio 的 API 名称也和标准库保持一致，只不过用 async fn 取代 fn 来声明函数。

    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 为每一条连接都生成一个新的任务，
        // socket 的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // Connection 对于 redis 的读写进行了抽象封装，
    // 因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // Connection 是在 mini-redis 中定义
    let mut connection = Connection::new(socket);

    // 使用 hashmap 来存储 redis 的数据
    let mut db = std::collections::HashMap::new();

    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);

    //     回复一个错误
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    use mini_redis::Command::{Get, Set};
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match mini_redis::Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 将请求响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
}

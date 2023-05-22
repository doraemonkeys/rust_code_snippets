use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

// cargo run --bin server
#[tokio::main]
async fn main() {
    // 上一章节中，咱们搭建了一个异步的 redis 服务器，并成功的提供了服务，但是其隐藏了一个
    // 巨大的问题：状态(数据)无法在多个连接之间共享，下面一起来看看该如何解决。
    // 好在 Tokio 十分强大，上面问题对应的解决方法也不止一种：
    // - 使用 Mutex 来保护数据的共享访问
    // - 生成一个异步任务去管理状态，然后各个连接使用消息传递的方式与其进行交互

    // 其中，第一种方法适合比较简单的数据，而第二种方法适用于需要异步工作的，例如 I/O 原语。
    // 由于我们使用的数据存储类型是 HashMap，使用到的 相关操作是 insert 和 get ，
    // 又因为这两个操作都不是异步的，因此只要使用 Mutex 即可解决问题。

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));
    // 我们使用了 std::sync::Mutex 来保护 HashMap，而不是使用 tokio::sync::Mutex。
    // 在使用 Tokio 编写异步代码时，一个常见的错误无条件地使用 tokio::sync::Mutex ，
    // 而真相是：Tokio 提供的异步锁只应该在跨多个 .await调用时使用，
    // 而且 Tokio 的 Mutex 实际上内部使用的也是 std::sync::Mutex。

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // 将 handle 克隆一份
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

// 类型别名，简化类型定义
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// 在上一节中，我们使用 Vec<u8> 来保存目标数据，
// 但是它有一个问题，对它进行克隆时会将底层数据也整个复制一份，效率很低
// Bytes 是一个引用计数类型，跟 Arc 非常类似，或者准确的说，
// Bytes 就是基于 Arc 实现的，但相比后者Bytes 提供了一些额外的能力。
async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

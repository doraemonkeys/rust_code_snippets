use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

// 我们的例子中，任务需要将 `GET` 和 `SET` 命令处理的结果返回。
// 首先，我们需要定一个 `Command` 枚举用于代表命令：
/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // Tokio 提供了多种消息通道，可以满足不同场景的需求:
    // - mpsc, 多生产者，单消费者模式
    // - oneshot, 单生产者单消费，一次只能发送一条消息
    // - broadcast，多生产者，多消费者，其中每一条发送的消息都可以被所有接收者收到，因此是广播
    // - watch，单生产者，多消费者，只保存一条最新的消息，因此接收者只能看到最近的一条消息，
    //          例如，这种模式适用于配置文件变化的监听
    // 这里还少了一种类型：多生产者、多消费者，且每一条消息只能被其中一个消费者接收，
    // 如果有这种需求，可以使用 async-channel 包。
    // 在多线程章节中提到过的 std::sync::mpsc 和 crossbeam::channel，
    // 这些通道在等待消息时会阻塞当前的线程，因此不适用于 async 编程。

    // 创建一个新通道，缓冲队列长度是 32。
    // 通道的缓冲队列长度是 32，意味着如果消息发送的比接收的快，这些消息将被存储在缓冲队列中，
    // 一旦存满了 32 条消息，使用send(...).await的发送者会进入睡眠，
    // 直到缓冲队列可以放入新的消息(被接收者消费了)。
    let (tx, mut rx) = mpsc::channel(32);

    // 你可以使用 clone 方法克隆多个发送者，但是接收者无法被克隆，因为我们的通道是 mpsc 类型。
    // Clone a `tx` handle for the second f
    let tx2 = tx.clone();
    // 在我们的例子中，接收者在管理 redis 连接的任务中，当该任务发现所有发送者都关闭时，
    // 它知道它的使命可以完成了，因此它会关闭 redis 连接。

    // 创建一个管理任务，它会管理 redis 的连接，当然，首先需要创建一条到 redis 的连接。
    let manager = tokio::spawn(async move {
        // Open a connection to the mini-redis address.
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // 往 oneshot 中发送消息时，并没有使用 .await，
                    // 原因是该发送操作要么直接成功、要么失败，并不需要等待。
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Spawn two tasks, one setting a value and other querying for key that was
    // set.
    // 让两个任务发送命令到消息通道。
    // 当spawn被调用时，提供的future将立即在后台开始运行，即使你不等待返回的JoinHandle。
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send the GET request
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // Send the SET request
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Set) = {:?}", res);
    });

    // t1和t2的执行顺序是不确定的，因为它们是并发执行的。
    // 所以t1有可能GOT (Get) = Ok(Ok(None))，也有可能GOT (Get) = Ok(Ok(Some(b"bar")))
    // tokio::spawn 返回一个 JoinHandle，它是一个 future，因此可以使用.await等待它的执行结果。
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

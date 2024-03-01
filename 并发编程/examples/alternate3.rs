use tokio::sync::oneshot;

#[derive(Debug)]
struct DataSender<T> {
    data: T,
    sender: Box<oneshot::Sender<DataSender<T>>>,
}

async fn worker(mut receiver: oneshot::Receiver<DataSender<i32>>, id: i32) {
    loop {
        let s = receiver.await.unwrap();
        println!("worker {}: {}", id, s.data);
        let (tx, rx) = oneshot::channel();
        s.sender
            .send(DataSender {
                data: s.data + 1,
                sender: Box::new(tx),
            })
            .unwrap();
        receiver = rx;
    }
}

// 两个线程/协程交替打印数字
// Rust异步实现
#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(worker(rx, 1));
    tx.send(DataSender {
        data: 0,
        sender: Box::new(tx2),
    })
    .unwrap();
    worker(rx2, 2).await
}

use std::sync::{Arc, Mutex};
use tokio::task;

/*
https://users.rust-lang.org/t/how-to-handle-deterministic-but-not-static-lifetime-in-tokio/112089/3
https://github.com/tokio-rs/tokio/issues/3162#issuecomment-2134405903


 tokio::spawn 要求闭包是 'static 的，这意味着闭包不能借用任何生命周期短于 'static 的数据。
 这是因为 spawned 任务可能会在创建它的函数返回后继续执行，因此任何非 'static 的引用都可能变得无效。

然而，有几种方法可以解决这个问题，允许你在 Tokio 任务中使用非 'static 数据：

1. 使用 Arc 和 Mutex/RwLock (推荐)

这是最常见的也是最推荐的方法。将数据包装在 Arc<Mutex<T>> 或 Arc<RwLock<T>> 中，然后克隆 Arc 并将其移动到闭包中。

2. 在tokio外开一个新线程，使用thread::scope

3. 使用tokio::task::unconstrained，放弃异步阻塞当前线程直到任务完成

4. clone
*/

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3];
    let shared_data = Arc::new(Mutex::new(data));

    let handle = task::spawn({
        let shared_data = shared_data.clone(); // Clone the Arc
        async move {
            let mut data = shared_data.lock().unwrap();
            data.push(4);
            println!("Data inside task: {:?}", data);
        }
    });

    // ... Do other work ...

    handle.await.unwrap();
    println!("Final data: {:?}", shared_data.lock().unwrap());
}

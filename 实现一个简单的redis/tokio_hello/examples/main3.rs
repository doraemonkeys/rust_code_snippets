use std::rc::Rc;
use tokio::task;

// !Send (non-Send) futures in asynchronous Rust code.

/*
Tokio的Tokio::spawn函数用于并发地启动任务。它要求派生的future实现Send。这是一个安全保证：Tokio可能会把future安排在另一个线程上。

如果您尝试在tokio::spawn的future中使用!Send类型（如Rc），则会得到编译时错误。

LocalSet保证派生到它的所有任务（未来）将在同一线程上运行。这消除了Send的需要，因为没有跨线程数据共享。

 Inside a LocalSet, you use task::spawn_local instead of tokio::spawn.
*/

#[tokio::main]
async fn main() {
    let nonsend_data = Rc::new("my nonsend data...");

    // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // The run_until method can only be used in #[tokio::main], #[tokio::test] or directly inside a call to [Runtime::block_on].
    // It cannot be used inside a task spawned with tokio::spawn.
    local
        .run_until(async move {
            let nonsend_data = nonsend_data.clone();
            // `spawn_local` ensures that the future is spawned on the local
            // task set.
            task::spawn_local(async move {
                println!("{}", nonsend_data);
                // ...
            })
            .await
            .unwrap();
        })
        .await;

    /*
    run_until 阻塞当前线程，直到给定的 future 完成。它主要用于运行一个作为"根"的 future，
    这个 future 可能会生成其他任务（包括使用 spawn_local 生成的非 Send 任务）。


     spawn_local 在 LocalSet 上生成一个非 Send 的新任务。 它类似于 tokio::spawn，但用于非 Send 的 future。

     */
}

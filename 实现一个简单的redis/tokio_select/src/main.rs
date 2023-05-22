#[tokio::main]
async fn main() {
    // tokio::select!
    study_tokio_select().await;
}

async fn study_tokio_select() {
    println!("--------------------tokio::select!-------------------");
    let (tx1, rx1) = tokio::sync::oneshot::channel();
    let (tx2, rx2) = tokio::sync::oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // select 在从两个通道阻塞等待接收消息时，rx1 和 rx2 都有可能被打印出来。
    // 对于 Async Rust 来说，释放( drop )掉一个 Future 就意味着取消任务。
    //  async 操作会返回一个 Future，而后者是惰性的，直到被 poll 调用时，才会被执行。
    // 一旦 Future 被释放，那操作将无法继续，因为所有相关的状态都被释放。
    tokio::select! {
        // 任何一个 select 分支结束后，都会跳出 select! 宏
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

    tokio_select_example().await;

    my_select_example().await;

    // 模式匹配
    tokio_select_match_example().await;

    // 借用
    tokio_select_borrow_example().await;

    // Pin
    tokio_select_pin_example().await;

    // 修改一个分支
    tokio_select_modify_branch_example().await;

    // 学到现在，相信大家对于 tokio::spawn 和 select! 已经非常熟悉，
    // 它们的共同点就是都可以并发的运行异步操作。 然而它们使用的策略大相径庭。
    // tokio::spawn 函数会启动新的任务来运行一个异步操作，每个任务都是一个独立的对象可以单独被 Tokio 调度运行，
    // 因此两个不同的任务的调度都是独立进行的，甚至于它们可能会运行在两个不同的操作系统线程上。
    // 鉴于此，生成的任务和生成的线程有一个相同的限制：不允许对外部环境中的值进行借用。
    // 而 select! 宏就不一样了，它在同一个任务中并发运行所有的分支。
    // 正是因为这样，在同一个任务中，这些分支无法被同时运行。 select! 宏在单个任务中实现了多路复用的功能。

    tokio_select_other_example().await;
}

async fn tokio_select_other_example() {
    println!("--------------------tokio_select_other_example-------------------");
    async fn _example1() {
        use tokio::sync::oneshot;

        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        // 这段代码async 没有move 为什么依然获得了tx1 tx2的所有权？
        tokio::spawn(async {
            let _ = tx1.send("one");
        });
        // let _ = tx1.send("one"); // use of moved value

        tokio::spawn(async {
            let _ = tx2.send("two");
        });

        tokio::select! {
            val = rx1 => {
                println!("rx1 completed first with {:?}", val);
            }
            val = rx2 => {
                println!("rx2 completed first with {:?}", val);
            }
        }
    }

    async fn _example2() {
        use tokio::sync::mpsc;

        let (tx1, mut rx1) = mpsc::channel(10);
        let (tx2, mut rx2) = mpsc::channel(10);

        // 换成mspc就需要move?
        tokio::spawn(async move {
            let _ = tx1.send("one");
        });

        tokio::spawn(async move {
            let _ = tx2.send("two");
        });

        tokio::select! {
            val = rx1.recv() => {
                println!("rx1 completed first with {:?}", val);
            }
            val = rx2.recv() => {
                println!("rx2 completed first with {:?}", val);
            }
        }
    }
    // mpsc::Sender::send(&self, value: T) 使用是Sender的引用，不使用move不会取得Sender的所有权，
    // 而oneshot::Sender::send(mut self, t: T)使用是self不是引用，不使用move也会以取得Sender的所有权，
}

async fn tokio_select_modify_branch_example() {
    println!("--------------------tokio_select_modify_branch_example-------------------");
    // 想要实现的逻辑是：
    // - 在消息通道中等待一个偶数出现
    // - 使用该偶数作为输入来启动一个异步操作
    // - 等待异步操作完成，与此同时监听消息通道以获取更多的偶数
    // - 若在异步操作完成前一个新的偶数到来了，终止当前的异步操作，然后接着使用新的偶数开始异步操作
    async fn action(input: Option<i32>) -> Option<String> {
        // 若 input（输入）是None，则返回 None
        // 事实上也可以这么写: let i = input?;
        let i = match input {
            Some(input) => input,
            None => return None,
        };
        Some(format!("action({})", i))
    }

    async fn exampl1() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(128);

        let mut done = false;
        let operation = action(None);
        tokio::pin!(operation);

        tokio::spawn(async move {
            let _ = tx.send(1).await;
            let _ = tx.send(3).await;
            let _ = tx.send(2).await;
        });

        loop {
            tokio::select! {
                res = &mut operation, if !done => {// 分支中可以使用 if 来过滤
                    done = true;

                    if let Some(v) = res {
                        println!("GOT = {}", v);
                        return;
                    }
                    println!("GOT = None");
                }
                Some(v) = rx.recv() => {
                    if v % 2 == 0 {
                        // `.set` 是 `Pin` 上定义的方法
                        operation.set(action(Some(v)));
                        done = false;
                    }
                }
            }
        }
    }
    exampl1().await;
}

async fn tokio_select_pin_example() {
    println!("--------------------tokio_select_pin_example-------------------");
    async fn _my_async_fn() {
        // async logic here
    }
    async fn _test1() {
        let future = _my_async_fn(); //  默认情况下，future 是固定住的，没有实现 Unpin 。
        tokio::pin!(future); // 注释掉这行代码，编译器会报错
        (&mut future).await; // 需要一个不能移动的引用(Pin<&mut T> 实现Unpin特征)，才能调用 await
    }

    // 恢复之前的异步操作
    println!("--------------------恢复之前的异步操作-------------------");
    async fn _action() {
        // 一些异步逻辑
    }
    async fn _test2() {
        let (_tx, mut rx) = tokio::sync::mpsc::channel::<i32>(128);

        let operation = _action();

        // Pins a value on the stack.
        tokio::pin!(operation);

        loop {
            tokio::select! {
                _ = &mut operation => break,
                Some(v) = rx.recv() => {
                    if v % 2 == 0 {
                        break;
                    }
                }
            }
        }
        // 在上面代码中，我们没有直接在 select! 分支中调用 action() ，
        // 而是在 loop 循环外面先将 action() 赋值给 operation，因此 operation 是一个 Future。
        // 重点来了，在 select! 循环中，我们使用了一个奇怪的语法 &mut operation，
        // 大家想象一下，如果不加 &mut 会如何？ 答案是，每一次循环调用的都是一次全新的 action()调用，
        // 第一次进入循环operation的所有权就会被转移，所以第二次循环就会报错。
        // 但是当加了 &mut operatoion 后，每一次循环调用就变成了对同一次 action() 的调用。
        // 也就是我们实现了在每次循环中恢复了之前的异步操作！

        // select! 的另一个分支从消息通道收取消息，一旦收到值是偶数，就跳出循环，否则就继续循环。
        // 还有一个就是我们使用了 tokio::pin!，具体的细节之前已经讲过了，这里就不再赘述。
        // 值得注意的点是：如果要在一个引用上使用 .await，那么引用的值就必须是不能移动的或者实现了 Unpin，
        // 一旦移除 tokio::pin! 所在行的代码，然后试图编译，就会获得错误。
    }
}

async fn tokio_select_borrow_example() {
    println!("--------------------tokio_select_borrow_example-------------------");
    // 当在 Tokio 中生成( spawn )任务时，其 async 语句块必须拥有其中数据的所有权。
    // 而 `select!` 并没有这个限制，它的每个分支表达式可以直接借用数据，然后进行并发操作。
    // 只要遵循 Rust 的借用规则，多个分支表达式可以不可变的借用同一个数据，或者在一个表达式可变的借用某个数据。
    async fn example1() {
        let (tx1, rx1) = tokio::sync::oneshot::channel::<i32>();
        let (_tx2, rx2) = tokio::sync::oneshot::channel::<i32>();

        let mut out = String::new();

        tokio::spawn(async move {
            let _ = tx1.send(1);
        });

        tokio::select! {
            _ = rx1 => {
                out.push_str("rx1 completed");
            }
            _ = rx2 => {
                out.push_str("rx2 completed");
            }
        }
        // 以上代码，就在两个分支的结果处理中分别进行了可变借用，并不会报错。原因就在于：
        // select!会保证只有一个分支的结果处理会被运行，然后在运行结束后，另一个分支会被直接丢弃。

        println!("{}", out);
    }
    example1().await;
}

async fn tokio_select_match_example() {
    println!("--------------------tokio_select_match_example-------------------");
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(128);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<i32>(128);

    tokio::spawn(async move {
        // 用 tx1 和 tx2 干一些不为人知的事
        // ...
        // 然后关闭它们
        drop(tx1);
        drop(tx2);
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        // rx 通道关闭后，recv() 方法会返回一个 None，
        // 可以看到没有任何模式能够匹配这个 None，那为何不会报错？
        // 当使用模式去匹配分支时，若之前的所有分支都无法被匹配，那 else 分支将被执行。
        else => {
            println!("Both channels closed");
        }
    }
}

async fn my_select_example() {
    println!("--------------------my_select_example-------------------");
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::sync::oneshot;

    struct MySelect {
        rx1: oneshot::Receiver<&'static str>,
        rx2: oneshot::Receiver<&'static str>,
    }

    impl Future for MySelect {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            // 参数 cx 被传入了内层的 poll 调用
            if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
                println!("rx1 completed first with {:?}", val);
                return Poll::Ready(());
            }

            if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
                println!("rx2 completed first with {:?}", val);
                return Poll::Ready(());
            }
            println!("both are not ready, pending");
            // 当一个 Future 返回 Poll::Pending 时，它必须确保会在某一个时刻通过 Waker 来唤醒，
            // 不然该 Future 将永远地被挂起。
            // 但是仔细观察我们之前的代码，里面并没有任何的 wake 调用！
            // 事实上，这是因为参数 cx 被传入了内层的 poll 调用。
            // 只要内部的 Future 实现了唤醒并且返回了 Poll::Pending，那 MySelect 也等于实现了唤醒！
            Poll::Pending
        }
    }
    async fn excute() {
        let (tx1, rx1) = oneshot::channel();
        let (_tx2, rx2) = oneshot::channel();

        tokio::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            let _ = tx1.send("one");
            // 任务完成，tokio的runtime会自动调用wake
        });

        MySelect { rx1, rx2 }.await;
    }
    // 这里是一个简化版本，在实际中，select! 会包含一些额外的功能，
    // 例如一开始会随机选择一个分支进行 poll。
    excute().await;
}

async fn tokio_select_example() {
    println!("--------------------tokio_select_example-------------------");
    let (mut tx1, rx1) = tokio::sync::oneshot::channel();
    let (tx2, rx2) = tokio::sync::oneshot::channel();

    async fn some_operation() -> String {
        println!("future running");
        // sleep 100ms
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("future completed");
        "some_operation".to_string()
    }
    // 目前来说，select! 最多可以支持 64 个分支，
    // 当 select 宏开始执行后，所有的分支会开始并发的执行(因为会对分支进行poll)，
    // 当任何一个表达式完成时，会将结果跟模式进行匹配。若匹配成功，则剩下的表达式会被释放。
    tokio::spawn(async {
        // 等待 `some_operation` 的完成
        // 或者处理 `oneshot` 的关闭通知
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // 收到了发送端发来的关闭信号
                // `select` 即将结束，此时，正在进行的 `some_operation()` 任务会被取消，任务自动完成，
                // tx1 被释放
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
    // 上面代码的重点就在于 `tx1.closed` 所在的分支，一旦发送端被关闭，那该分支就会被执行，
    // 然后 `select` 会退出，并清理掉还没执行的第一个分支 `val = some_operation()` ，
    // 这其中 `some_operation` 返回的 `Future` 也会被清理，
    // 根据之前的内容，`Future` 被清理那相应的任务会立即取消，因此 `some_operation` 会被取消，不再执行。
}

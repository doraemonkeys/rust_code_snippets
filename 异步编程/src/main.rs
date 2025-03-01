fn main() {
    // async 初识
    study_async_and_await();

    // Future 执行器与任务调度
    study_future_executor();

    // Pin 和 Unpin
    study_pin_and_unpin();

    // async/await 和 Stream 流处理
    study_async_await_and_stream();

    // 使用 join! 和 select! 同时运行多个 Future
    study_join_and_select();

    // 一些疑难问题的解决办法
    study_difficulties();
}

fn study_difficulties() {
    println!("------------------study_difficulties------------------");
    // `async` 语句块和 `async fn` 最大的区别就是前者无法显式的声明返回值，
    // 在大多数时候这都不是问题，但是当配合 `?` 一起使用时，问题就有所不同:
    async fn _foo() -> Result<u8, String> {
        Ok(1)
    }
    async fn _bar() -> Result<u8, String> {
        Ok(1)
    }
    pub fn _main() {
        // 目前还没有办法为 `async` 语句块指定返回类型。
        // 只能在返回值进行显式的类型注释
        let _fut = async {
            _foo().await?;
            _bar().await?;
            // Ok(()) // ERROR: 无法推断出返回类型
            Ok::<(), String>(()) // 在这一行进行显式的类型注释
        };
        let res = futures::executor::block_on(_fut);
        println!("{:?}", res);
    }

    // async 函数和 Send 特征
    study_async_fn_and_send();
}

fn study_async_fn_and_send() {
    println!("------------------async 函数和 Send 特征------------------");
    // 在多线程章节我们深入讲过 Send 特征对于多线程间数据传递的重要性，
    // 对于 async fn 也是如此，它返回的 Future 能否在线程间传递的关键在于 .await 运行过程中
    // 作用域中的变量类型是否是 Send。
    // 学到这里，相信大家已经很清楚 Rc 无法在多线程环境使用，
    // 原因就在于它并未实现 Send 特征，那咱就用它来做例子:
    use std::rc::Rc;
    #[derive(Default)]
    struct NotSend(Rc<()>);
    async fn bar2() {}

    // async fn foo2() {
    //     // 返回的 `Future` 是 `Send`， 但是在它内部短暂的使用 `NotSend` 依然是安全的，
    //     // 原因在于它的作用域并没有影响到 `.await`
    //     NotSend::default();
    //     bar2().await;
    // }
    async fn foo2() {
        /*
                // 下面来试试声明一个变量，然后让 `.await` 的调用处于变量的作用域中试试
                let x = NotSend::default();
                bar2().await; // ERROR: .await 在运行时处于 x 的作用域内。
        */

        // 不知道有多少同学还记得语句块 `{ ... }` 在 Rust 中其实具有非常重要的作用
        // 可以将变量声明在语句块内，当语句块结束时，变量会自动被 Drop，
        // 这个规则可以帮助我们解决很多借用冲突问题
        {
            let _x = NotSend::default();
        }
        bar2().await;
    }

    fn require_send(_: impl Send) {}
    require_send(foo2());
}

fn study_join_and_select() {
    println!("------------------study_join_and_select------------------");

    // join! and try_join!
    join_and_try_join();

    // select!
    study_select();

    // 在 select 循环中并发
    study_select_loop();
}

fn study_select_loop() {
    println!("------------------在 select 循环中并发------------------");
    // 一个很实用但又鲜为人知的函数是 `Fuse::terminated()` ，
    // 可以使用它构建一个空的 `Future` ，空自然没啥用，但是如果它能在后面再被填充呢？
    // 考虑以下场景：当你要在 `select` 循环中运行一个任务，
    // 但是该任务却是在 `select` 循环内部创建时，上面的函数就非常好用了。
    use futures::{
        future::{Fuse, FusedFuture, FutureExt},
        pin_mut, select,
        stream::{FusedStream, Stream, StreamExt},
    };

    async fn get_new_num() -> u8 {
        let new = 5;
        println!("get_new_num: {}", new);
        new
    }

    async fn run_on_new_num(n: u8) {
        println!("run_on_new_num: {}", n);
    }

    async fn run_loop(
        mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
        starting_num: u8,
    ) {
        let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
        let get_new_num_fut = Fuse::terminated();
        pin_mut!(run_on_new_num_fut, get_new_num_fut);
        println!(
            "get_new_num_fut.is_terminated: {}",
            get_new_num_fut.is_terminated()
        );
        loop {
            // 若所有分支都没有就绪(所有分支都返回了Poll::Pending)，select! 将阻塞，
            // 直到 Future/Stream 调用Waker::wake (准备就绪)
            select! {
                () = interval_timer.select_next_some() => {
                    // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                    println!("定时器已结束!!!");
                    if get_new_num_fut.is_terminated() {
                        get_new_num_fut.set(get_new_num().fuse());
                        println!(
                            "get_new_num_fut.is_terminated: {}",
                            get_new_num_fut.is_terminated()
                        );
                    }
                },
                new_num = get_new_num_fut => {
                    // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                    println!("收到新的数字: {}", new_num);
                    run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
                },
                // 运行 `run_on_new_num_fut`
                () = run_on_new_num_fut => {},
                // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
                //后，执行到 `complete` 分支
                // complete => println!("`interval_timer` completed unexpectedly"),
                complete => {
                    println!("get_new_num_fut.is_terminated: {}", get_new_num_fut.is_terminated());
                    println!("`timer_once` completed");
                    break;
                }
               //default => println!("`interval_timer` is not ready yet"),
            }
        }
    }
    struct SharedState {
        /// 定时(睡眠)是否结束
        completed: bool,

        /// 当睡眠结束后，线程可以用`waker`通知执行器唤醒任务
        waker: Option<std::task::Waker>,
    }
    struct TickerOnce {
        tick: std::sync::Arc<std::sync::Mutex<SharedState>>,
        ticked: bool,
    }
    impl TickerOnce {
        fn new(time: u64) -> Self {
            let ret = TickerOnce {
                tick: std::sync::Arc::new(std::sync::Mutex::new(SharedState {
                    completed: false,
                    waker: None,
                })),
                ticked: false,
            };
            let state = ret.tick.clone();
            let timer = move || {
                println!("定时器开始");
                std::thread::sleep(std::time::Duration::from_millis(time));
                let mut state = state.lock().unwrap();
                state.completed = true;
                println!("定时器结束");
                if let Some(waker) = state.waker.take() {
                    println!("wake");
                    waker.wake();
                } else {
                    println!("waker is None");
                }
            };
            std::thread::spawn(timer);
            ret
        }
    }
    impl Stream for TickerOnce {
        type Item = ();
        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            if self.ticked {
                println!("retuen Ready(None)"); // 迭代器结束
                return std::task::Poll::Ready(None);
            }
            {
                let mut state = self.tick.lock().unwrap();
                if !state.completed {
                    println!("retuen Pending");
                    state.waker = Some(cx.waker().clone());
                    return std::task::Poll::Pending;
                }
            }
            self.ticked = true;
            println!("retuen Ready(Some(()))");
            std::task::Poll::Ready(Some(()))
        }
    }
    use futures::executor::block_on;
    block_on(run_loop(TickerOnce::new(1000).fuse(), 1));
}

fn study_select() {
    println!("------------------select!------------------");
    // join! 只有等所有 Future 结束后，才能集中处理结果，如果你想同时等待多个 Future ，
    // 且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!。

    async fn task_one() { /* ... */
    }
    async fn task_two() { /* ... */
    }

    async fn race_tasks() {
        use futures::{
            future::FutureExt, // for `.fuse()`
            pin_mut,
            select,
        };

        let t1 = task_one().fuse();
        let t2 = task_two().fuse();

        pin_mut!(t1, t2);

        select! {
            () = t1 => println!("任务1率先完成"),
            () = t2 => println!("任务2率先完成"),
        }
    }
    use futures::executor::block_on;
    // 上面的代码会同时并发地运行 t1 和 t2， 无论两者哪个先完成，
    // 都会调用对应的 println! 打印相应的输出，然后函数结束且不会等待另一个任务的完成。
    block_on(race_tasks());

    // default 和 complete
    // select!还支持 default 和 complete 分支:
    // - complete 分支当所有的 Future 和 Stream 完成后才会被执行，
    //   它往往配合 loop 使用，loop 用于循环完成所有的 Future
    // - default 分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行

    let mut a_fut = futures::future::ready(4);
    let mut b_fut = futures::future::ready(6);
    let mut total = 0;
    loop {
        use futures::select;
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            //complete => println!("completed"), // 如果你希望 default 也有机会露下脸，可以取消break
            default => panic!(), // 该分支永远不会运行，因为 Future 会先运行，然后是 complete
        };
    }
    assert_eq!(total, 10);

    // 跟 Unpin 和 FusedFuture 进行交互
    //  首先，.fuse() 方法可以让 Future 实现 FusedFuture 特征，
    // 而 pin_mut! 宏会为 Future 实现 Unpin 特征，这两个特征恰恰是使用 select 所必须的:
    // - Unpin，由于 select 不会通过拿走所有权的方式使用 Future，而是通过可变引用的方式去使用，
    //    这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用。
    // - FusedFuture 的原因跟上面类似，当 Future 一旦完成后，
    //    那 select 就不能再对其进行轮询使用。Fuse 意味着熔断，
    //    相当于 Future 一旦完成，再次调用 poll 会直接返回 Poll::Pending。
    // 只有实现了 FusedFuture，select 才能配合 loop 一起使用。
    // 假如没有实现，就算一个 Future 已经完成了，它依然会被 `select` 不停的轮询执行。

    async fn _race_tasks2() {
        use futures::{
            future::FutureExt, // for `.fuse()`
            pin_mut,
            select,
        };

        let t1 = task_one().fuse();
        let t2 = task_two().fuse();
        pin_mut!(t1, t2);
        // 要求其实现Unpin + FusedFuture
        select! {
            () = t1 => println!("任务1率先完成"),
            () = t2 => println!("任务2率先完成"),
        }

        // Ok! only FusedFuture，自动 from 进行变换 实现 Unpin
        select! {
            () = task_one().fuse() => println!("任务1率先完成"),
            () = task_two().fuse() => println!("任务2率先完成"),
        }
    }

    // Stream 稍有不同，它们使用的特征是 FusedStream。
    // 通过 .fuse()(也可以手动实现)实现了该特征的 Stream，
    // 对其调用 .next() 或 .try_next() 方法可以获取实现了 FusedFuture 特征的Future:

    use futures::stream::{FusedStream, Stream, StreamExt};
    async fn add_two_streams(
        mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
        mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
    ) -> u8 {
        use futures::select;
        let mut total = 0;

        loop {
            let item = select! {
                x = s1.next() => x,
                x = s2.next() => x,
                complete => break,
            };
            if let Some(next_num) = item {
                total += next_num;
            }
        }
        total
    }

    let s1 = futures::stream::iter(vec![1u8, 2, 3]);
    let s2 = futures::stream::iter(vec![4, 5, 6]);
    assert_eq!(block_on(add_two_streams(s1.fuse(), s2.fuse())), 21);
}

fn join_and_try_join() {
    println!("---------------------join_and_try_join---------------------");
    // futures 包中提供了很多实用的工具，其中一个就是 join! 宏，
    // 它允许我们同时等待多个不同 Future 的完成，且可以并发地运行这些 Future。

    async fn enjoy_book_and_music() -> (Book, Music) {
        use futures::join;
        let book_fut = enjoy_book();
        let music_fut = enjoy_music();
        join!(book_fut, music_fut)
    }
    async fn enjoy_book() -> Book {
        println!("enjoy_book");
        Book
    }
    async fn enjoy_music() -> Music {
        println!("enjoy_music");
        Music
    }
    #[derive(Debug)]
    struct Book;
    #[derive(Debug)]
    struct Music;

    use futures::executor::block_on;
    let result = block_on(enjoy_book_and_music());
    println!("{:?}", result);

    // 如果希望同时运行一个数组里的多个异步任务，可以使用 futures::future::join_all 方法
    async fn enjoy_books() -> Vec<Book> {
        use futures::future::join_all;
        let book_futs = vec![enjoy_book(), enjoy_book(), enjoy_book()];
        join_all(book_futs).await
    }
    let result = block_on(enjoy_books());
    println!("{:?}", result);

    // try_join!
    println!("------------------try_join!------------------");
    // 由于 join! 必须等待它管理的所有 Future 完成后才能完成，
    // 如果你希望在某一个 Future 报错后就立即停止所有 Future 的执行，
    // 可以使用 try_join!，特别是当 Future 返回 Result 时:

    async fn get_book() -> Result<Book, String> {
        /* ... */
        Ok(Book)
    }
    async fn get_music() -> Result<Music, String> {
        /* ... */
        Ok(Music)
    }
    async fn get_book_and_music() -> Result<(Book, Music), String> {
        use futures::try_join;
        let book_fut = get_book();
        let music_fut = get_music();
        try_join!(book_fut, music_fut)
    }
    let result = block_on(get_book_and_music());
    println!("{:?}", result);

    // 传给 try_join! 的所有 Future 都必须拥有相同的错误类型。如果错误类型不同，
    // 可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err 和 err_info 方法将错误进行转换:

    async fn get_book2() -> Result<Book, ()> {
        /* ... */
        Ok(Book)
    }
    async fn get_music2() -> Result<Music, String> {
        /* ... */
        Ok(Music)
    }

    async fn get_book_and_music2() -> Result<(Book, Music), String> {
        use futures::{future::TryFutureExt, try_join};
        let book_fut = get_book2().map_err(|()| "Unable to get book".to_string());
        let music_fut = get_music2();
        try_join!(book_fut, music_fut)
    }
    let result = block_on(get_book_and_music2());
    println!("{:?}", result);
}

fn study_async_await_and_stream() {
    println!("------------------study_async_await_and_stream------------------");
    // async/.await 是 Rust 语法的一部分，它在遇到阻塞操作时( 例如 IO )
    // 会让出当前线程的所有权而不是阻塞当前线程，这样就允许当前线程继续去执行其它代码，最终实现并发。
    // 有两种方式可以使用 async： async fn 用于声明函数，async { ... } 用于声明语句块，
    // 它们会返回一个实现 Future 特征的值。

    // async 的生命周期
    println!("------------------async的生命周期------------------");
    use futures::Future;
    // `async fn` 函数如果拥有引用类型的参数，那它返回的 `Future` 的生命周期就会被这些参数的生命周期所限制:
    async fn _foo(x: &u8) -> u8 {
        *x
    }
    // 上面的函数跟下面的函数是等价的:
    // 简单来说就是 x 必须比 Future 活得更久。
    fn _foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
        async move { *x }
    }

    // fn bad() -> impl Future<Output = u8> {
    //     let x = 5;
    //     _borrow_x(&x) // ERROR: x does not live long enough
    //     // 以上代码会报错，因为 x 的生命周期只到 bad 函数的结尾。 但是 Future 显然会活得更久。
    // }
    async fn _borrow_x(x: &u8) -> u8 {
        *x
    }

    // 其中一个常用的解决方法就是将具有引用参数的 async fn 函数
    // 转变成一个具有 'static 生命周期的 Future 。
    async fn borrow_x(x: &u8) -> u8 {
        *x
    }
    fn good() -> impl Future<Output = u8> {
        // 通过将参数移动到 async 语句块内， 我们将它的生命周期扩展到 'static，
        // 并跟返回的 Future 保持了一致。
        async {
            let x = 5;
            borrow_x(&x).await
        }
    }
    let _ = good();

    // async move
    println!("------------------async move------------------");
    // `async` 允许我们使用 `move` 关键字来将环境中变量的所有权转移到语句块内，就像闭包那样，
    // 好处是你不再发愁该如何解决借用生命周期的问题，坏处就是无法跟其它代码实现对变量的共享。
    // 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
    async fn _blocks() {
        let my_string = "foo".to_string();

        let future_one = async {
            // ...
            println!("{my_string}");
        };

        let future_two = async {
            // ...
            println!("{my_string}");
        };

        // 运行两个 Future 直到完成
        let ((), ()) = futures::join!(future_one, future_two);
    }

    // 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
    // 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
    fn move_block() -> impl Future<Output = ()> {
        let my_string = "foo".to_string();
        async move {
            // ...
            println!("{my_string}");
        }
    }
    let _ = move_block();

    // 当.await 遇见多线程执行器
    println!("------------------当.await遇见多线程执行器------------------");
    // 需要注意的是，当使用多线程 Future 执行器( executor )时，
    //  Future 可能会在线程间被移动，因此 async 语句块中的变量必须要能在线程间传递。
    // 至于 Future 会在线程间移动的原因是：它内部的任何.await都可能导致它被切换到一个新线程上去执行。
    // 由于需要在多线程环境使用，意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、
    // 没有实现 Sync 的引用类型，它们都是不安全的，因此无法被使用
    // 需要注意！实际上它们还是有可能被使用的，只要在 .await 调用期间，它们没有在作用域范围内。
    // 类似的原因，在 .await 时使用普通的锁也不安全，例如 Mutex 。原因是，它可能会导致线程池被锁：
    // 当一个任务获取锁 A 后，若它将线程的控制权还给执行器，然后执行器又调度运行另一个任务，
    // 该任务也去尝试获取了锁 A ，结果当前线程会直接卡死，最终陷入死锁中。
    // 因此，为了避免这种情况的发生，我们需要使用 futures 包下的锁 futures::lock 来替代 Mutex 完成任务。

    // Stream 流处理
    println!("------------------Stream流处理------------------");
    // Stream 特征类似于 Future 特征，但是前者在完成前可以生成多个值，
    // 这种行为跟标准库中的 Iterator 特征倒是颇为相似
    use std::pin::Pin;
    use std::task::{Context, Poll};
    trait _Stream {
        // Stream生成的值的类型
        type Item;

        // 尝试去解析Stream中的下一个值,
        // 若无数据，返回Poll::Pending, 若有数据，返回 Poll::Ready(Some(x)),
        // Stream完成则返回 Poll::Ready(None)
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    }
    // 关于 Stream 的一个常见例子是消息通道（ futures 包中的）的消费者 Receiver。
    // 每次有消息从 Send 端发送后，它都可以接收到一个 Some(val) 值，
    // 一旦 Send 端关闭( drop )，且消息通道中没有消息后，它会接收到一个 None 值。
    async fn _send_recv() {
        use futures::stream::StreamExt;
        use tokio::sync::mpsc;
        const BUFFER_SIZE: usize = 10;
        let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        drop(tx);

        // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，
        // 而是一个 `Future<Output = Option<T>>`，
        // 因此还需要使用`.await`来获取具体的值
        assert_eq!(Some(1), rx.next().await);
        assert_eq!(Some(2), rx.next().await);
        assert_eq!(None, rx.next().await);
    }

    // 迭代和并发
    println!("------------------迭代和并发------------------");
    // 跟迭代器类似，我们也可以迭代一个 Stream。 例如使用 map，filter，fold 方法，
    // 以及它们的遇到错误提前返回的版本： try_map，try_filter，try_fold。
    // 但是跟迭代器又有所不同，for 循环无法在这里使用，但是命令式风格的循环while let是可以用的，
    // 同时还可以使用next 和 try_next 方法:
    use futures::stream::Stream;
    async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
        use futures::StreamExt;
        let mut sum = 0;
        while let Some(item) = stream.next().await {
            sum += item;
        }
        sum
    }

    async fn sum_with_try_next(
        mut stream: Pin<&mut dyn Stream<Item = Result<i32, std::io::Error>>>,
    ) -> Result<i32, std::io::Error> {
        use futures::stream::TryStreamExt; // 引入 try_next
        let mut sum = 0;
        while let Some(item) = stream.try_next().await? {
            sum += item;
        }
        Ok(sum)
    }
    let mut stream = futures::stream::iter(vec![2, 3]);
    let pin: Pin<&mut futures::stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(5, futures::executor::block_on(sum_with_next(pin)));

    let mut stream = futures::stream::iter(vec![Ok(2), Ok(3)]);
    let pin: Pin<&mut futures::stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(
        5,
        futures::executor::block_on(sum_with_try_next(pin)).unwrap()
    );

    // 上面代码是一次处理一个值的模式，但是需要注意的是：如果你选择一次处理一个值的模式，
    // 可能会造成无法并发，这就失去了异步编程的意义。
    // 因此，如果可以的话我们还是要选择从一个 Stream 并发处理多个值的方式，
    // 通过 for_each_concurrent 或 try_for_each_concurrent 方法来实现:
    #[allow(unused)]
    async fn jump_n_times(_: u8) -> Result<(), std::io::Error> {
        Ok(())
    }
    #[allow(unused)]
    async fn report_n_jumps(_: u8) -> Result<(), std::io::Error> {
        Ok(())
    }
    #[allow(unused)]
    async fn jump_around(
        mut stream: Pin<&mut dyn Stream<Item = Result<u8, std::io::Error>>>,
    ) -> Result<(), std::io::Error> {
        use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
        const MAX_CONCURRENT_JUMPERS: usize = 100;

        stream
            .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
                jump_n_times(num).await?;
                report_n_jumps(num).await?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}

fn study_pin_and_unpin() {
    println!("------------------study_Pin_and_Unpin------------------");
    //  在 Rust 中，所有的类型可以分为两类:
    // - 类型的值可以在内存中安全地被移动，例如数值、字符串、布尔值、结构体、枚举，
    //   总之你能想到的几乎所有类型都可以落入到此范畴内
    // - 自引用类型，
    //   引用、Box、Vec、String、Future 等等，这些类型的值在内存中的位置是不固定的，
    //   也就是说，它们的值可能会被移动到其他地方，这就导致了一个问题，
    //   如果我们在内存中保存了一个指向它们的指针，那么当这个类型的值被移动后，
    //   指针就会指向一个无效的内存地址，这就是所谓的悬垂指针(dangling pointer)。
    //   如果内存地址可能会变动，那存储指针地址将毫无意义！
    //   我们想要这些引用类型永远指向同一个内存地址，这就是 Pin 的作用。

    // 下面就是一个自引用类型
    struct _SelfRef {
        value: String,
        pointer_to_value: *mut String,
    }
    // 在上面的结构体中，pointer_to_value 是一个裸指针，指向第一个字段 value 持有的字符串 String。
    // 很简单对吧？现在考虑一个情况， 若`String` 被移动了怎么办？
    // 此时一个致命的问题就出现了：新的字符串的内存地址变了，而 pointer_to_value 依然
    // 指向之前的地址，一个重大 bug 就出现了！
    // 灾难发生，英雄在哪？只见 Pin 闪亮登场，它可以防止一个类型在内存中被移动。
    // 再来回忆下之前在 Future 章节中，我们提到过在 poll 方法的签名中有一个 self: Pin<&mut Self> ，
    // 那么为何要在这里使用 Pin 呢？

    // 其实 Pin 还有一个小伙伴 UnPin ，与前者相反，后者表示类型可以在内存中安全地移动。
    // 事实上，绝大多数类型都不在意是否被移动(开篇提到的第一种类型)，因此它们都自动实现了 `Unpin` 特征。

    // Pin 是一个结构体，它包裹一个指针，并且能确保该指针指向的数据不会被移动，
    // 例如 Pin<&mut T> , Pin<&T> , Pin<Box<T>> ，都能确保 T 不会被移动。
    pub struct _Pin<P> {
        pointer: P,
    }

    // 而 Unpin 是一个特征 !!!，它表明一个类型可以随意被移动，
    // 那么问题来了，可以被 Pin 住的值，它有没有实现什么特征呢？
    // 答案很出乎意料，可以被 Pin 住的值实现的特征是 !Unpin ，
    //  大家可能之前没有见过，但是它其实很简单，! 代表没有实现某个特征的意思，
    // !Unpin 说明类型没有实现 Unpin 特征，那自然就可以被 Pin 了。

    // 那是不是意味着类型如果实现了 Unpin 特征，就不能被 Pin 了？
    // 其实，还是可以 Pin 的，毕竟它只是一个结构体，你可以随意使用，
    // 但是不再有任何效果而已，该值一样可以被移动！

    // 例如 Pin<&mut u8> ，显然 u8 实现了 Unpin 特征，它可以在内存中被移动，
    // 因此 Pin<&mut u8> 跟 &mut u8 实际上并无区别，一样可以被移动。

    // 如果将 Unpin 与之前章节学过的 Send/Sync 进行下对比，会发现它们都很像：
    // - 都是标记特征( marker trait )，该特征未定义任何行为，非常适用于标记
    // - 都可以通过`!`语法去除实现
    // - 绝大多数情况都是自动实现, 无需我们的操心

    #[derive(Debug)]
    struct MyTest {
        a: String,
        b: *const String,
    }
    // `Test` 提供了方法用于获取字段 `a` 和 `b` 的值的引用。这里`b` 是 `a` 的一个引用，
    // 但是我们并没有使用引用类型而是用了裸指针，
    // 原因是：Rust 的借用规则不允许我们这样用，因为不符合生命周期的要求。
    // 此时的 `Test` 就是一个自引用结构体。

    impl MyTest {
        fn new(txt: &str) -> Self {
            MyTest {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }
    // 如果不移动任何值，那么上面的例子将没有任何问题，例如:
    let mut test1 = MyTest::new("test1");
    test1.init();
    let mut test2 = MyTest::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b()); // a: test1, b: test1
    println!("a: {}, b: {}", test2.a(), test2.b()); // a: test2, b: test2

    println!("------------------------------------");
    // 既然移动数据会导致指针不合法，那我们就移动下数据试试，将 test1 和 test2 进行下交换：
    let mut test1 = MyTest::new("test1");
    test1.init();
    let mut test2 = MyTest::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b()); // a: test1, b: test1
    std::mem::swap(&mut test1, &mut test2);
    // https://folyd.com/blog/rust-pin-unpin/
    // 注意test1与test2这两个结构体都在栈上，只是a字段的值是在堆上的。
    // Test结构体中的字段b是一个指向字段a的指针，字段b在栈上存的是字段a在栈上的地址。
    // test1移动到test2的栈内存后，test1中a字段的值依旧指向原来的堆内存地址，
    // 但是test1中b字段的值却指向了test2中a字段在栈上的地址(即指向了旧的栈内存地址)。
    // test1.b 与 test2.b 指针依然指向了旧的地址，而该地址对应的值已经被移动了，
    println!("a: {}, b: {}", test1.a(), test1.b()); // a: test2, b: test1
    println!("a: {}, b: {}", test2.a(), test2.b()); // a: test1, b: test2

    /*
        #include <iostream>
        using namespace std;
        int main() {
            struct foo {
                int a;
                int* b;
            };

            foo f;
            f.a = 100;
            f.b = &f.a;
            cout << f.a << endl;
            cout << *f.b << endl;

            foo f2;
            f2.a = 200;
            f2.b = &f2.a;
            cout << f2.a << endl;
            cout << *f2.b << endl;

            foo temp = f;
            f = f2;
            f2 = temp;
            cout << "After swap" << endl;

            cout << f.a << endl;
            cout << *f.b << endl;

            cout << f2.a << endl;
            cout << *f2.b << endl;
        }
    */

    // 在理解了 Pin 的作用后，我们再来看看它怎么帮我们解决问题。
    example_study_pin_and_unpin();

    // 将固定住的 Future 变为 Unpin
    println!("------------------将固定住的 Future 变为 Unpin------------------");
    // 之前的章节我们有提到 async 函数返回的 Future 默认就是 !Unpin 的。
    // 但是，在实际应用中，一些函数会要求它们处理的 Future 是 Unpin 的，此时，
    // 若你使用的 Future 是 !Unpin 的，必须要使用以下的方法先将 Future 进行固定:
    // - Box::pin， 创建一个 Pin<Box<T>>
    // - pin_utils::pin_mut!， 创建一个 Pin<&mut T>
    use pin_utils::pin_mut; //  pin_utils  可以在crates.io中找到

    // 函数的参数是一个 Future ，但是要求该 Future 实现 Unpin
    use futures::Future;
    fn execute_unpin_future(_x: impl Future<Output = ()> + Unpin) {
        println!("execute_unpin_future：ok");
    }

    let _fut = async { /* ... */ };
    // 下面代码报错: 默认情况下，future 是固定住的，没有实现 Unpin 。
    // execute_unpin_future(_fut);

    // Pin 是一个结构体，它包裹一个指针，并且能确保该指针指向的数据不会被移动，
    // 但Pin本身是可以移动的，因此 Pin 本身是 Unpin 的。

    // 使用 Box 进行固定
    let fut = async { /* ... */ };
    let fut = Box::pin(fut); // 返回 Pin<Box<T>>
    execute_unpin_future(fut); // OK

    // 使用 pin_mut! 进行固定
    let fut = async { /* ... */ };
    // // pin_mut! 宏会为 Future 实现 Unpin 特征
    // (Pin本身是可以移动的,但Pin包裹的数据不可以移动(前提是没有Unpin特征))
    pin_mut!(fut);
    execute_unpin_future(fut); // OK

    // Unpin 是一个特征 !!!，它表明一个类型可以随意被移动。
    // 如果实现了 Unpin 特征，还是可以包裹 Pin 的，毕竟它只是一个结构体，你可以随意使用，
    // 但是不再有任何效果而已，该值一样可以被移动！
}

fn example_study_pin_and_unpin() {
    println!("------------------Pin 在实践中的运用------------------");

    // 固定到栈上
    example_study_pin_and_unpin_1();

    // 固定到堆上
    example_study_pin_and_unpin_2();
}

fn example_study_pin_and_unpin_2() {
    println!("------------------Pin 固定到堆上------------------");
    // 将一个 `!Unpin` 类型的值固定到堆上，会给予该值一个稳定的内存地址，
    // 它指向的堆中的值在 `Pin` 后是无法被移动的。
    // 而且与固定在栈上不同，我们知道堆上的值在整个生命周期内都会被稳稳地固定住。
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        // 用Box包裹，固定到堆上
        fn new(txt: &str) -> Pin<Box<Self>> {
            let t = Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

            boxed
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }

    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}

fn example_study_pin_and_unpin_1() {
    println!("------------------Pin 固定到栈上------------------");
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        // 我们使用了一个标记类型 `PhantomPinned` 将自定义结构体 `Test` 变成了 `!Unpin`
        // (编译器会自动帮我们实现)，因此该结构体无法再被移动。
        _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
    }

    impl Test {
        // 返回了Self , 即栈上的Test
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
            // cannot assign to data in dereference of `Pin<&mut Test>`
            // self.b = self_ptr;
            // self.a = String::from("test");
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }
    // 此时的`test1`可以被随意的移动
    let mut test1 = Test::new("test1");
    // 一旦类型实现了 `!Unpin` ，那将它的值固定到栈( `stack` )上就是不安全的行为，
    // 因此在代码中我们使用了 `unsafe` 语句块来进行处理
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    // 你也可以使用 pin_utils(https://docs.rs/pin-utils/)来避免 unsafe 的使用
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());
    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    // 再去尝试移动被Pin 固定的 !Unpin 类型，是不被允许的：
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", test2.as_ref().a(), Test::b(test2.as_ref()));
}

fn study_future_executor() {
    println!("------------------study_future_executor------------------");

    // 一个简化版的 Future 特征:
    trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }

    enum Poll<T> {
        Ready(T),
        Pending,
    }
    // 们提到过 Future 需要被执行器poll(轮询)后才能运行，诺，这里 poll 就来了，
    // 通过调用该方法，可以推进 Future 的进一步执行，直到被切走为止。
    // 这里不好理解，但是你只需要知道 Future 并不能保证在一次 poll 中就被执行完，后面会详解介绍。

    // 若在当前 poll 中， Future 可以被完成，则会返回 Poll::Ready(result) ，
    // 反之则返回 Poll::Pending， 并且安排一个 wake 函数。
    // 当未来 Future 准备好进一步执行时， 该 wake 函数会被调用，
    // 然后管理该 Future 的执行器(例如上一章节中的block_on函数)会再次调用 poll 方法，
    // 此时 Future 就可以继续执行了。

    // 如果没有 wake 方法，那执行器无法知道某个 Future 是否可以继续被执行，
    // 除非执行器定期的轮询每一个 Future，确认它是否能被执行，但这种作法效率较低。
    // 而有了 wake，Future 就可以主动通知执行器，然后执行器就可以精确的执行该 Future。
    // 这种“事件通知 -> 执行”的方式要远比定期对所有 Future 进行一次全遍历来的高效。

    // 考虑一个需要从 socket 读取数据的场景：如果有数据，可以直接读取数据并返回 Poll::Ready(data)，
    // 但如果没有数据，Future 会被阻塞且不会再继续执行，此时它会注册一个 wake 函数，
    // 当 socket 数据准备好时，该函数将被调用以通知执行器：我们的 Future 已经准备好了，可以继续执行。
    // 下面的 SocketRead 结构体就是一个 Future:
    pub struct SocketRead<'a> {
        socket: &'a mut Socket,
    }

    impl SimpleFuture for SocketRead<'_> {
        type Output = Vec<u8>;

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                // socket有数据，写入buffer中并返回
                Poll::Ready(self.socket.read_buf())
            } else {
                // socket中还没数据
                //
                // 注册一个 wake 回调函数函数，当数据可用时，该函数会被调用，
                // 然后当前Future的执行器会再次调用 poll 方法，此时就可以读取到数据
                self.socket.set_readable_callback(wake);
                Poll::Pending
            }
        }
    }
    struct Socket {
        buffer: Vec<u8>,
        callback: Option<fn()>,
    }

    impl Socket {
        fn new() -> Self {
            Socket {
                buffer: vec![],
                callback: None,
            }
        }
        fn has_data_to_read(&self) -> bool {
            self.buffer.len() > 0
        }
        fn generate_data(&mut self) {
            self.buffer = vec![1, 2, 3];
            if let Some(callback) = self.callback {
                callback();
            }
        }
        fn read_buf(&self) -> Vec<u8> {
            vec![]
        }
        fn set_readable_callback(&mut self, wake: fn()) {
            self.callback = Some(wake);
        }
    }
    let mut socket = Socket::new();
    let mut socket_read = SocketRead {
        socket: &mut socket,
    };
    let data = socket_read.poll(|| println!("wake up"));
    match data {
        Poll::Ready(data) => println!("data: {:?}", data),
        Poll::Pending => println!("pending"),
    }
    socket_read.socket.generate_data();

    // 这种 Future 模型允许将多个异步操作组合在一起，同时还无需任何内存分配。
    // 不仅仅如此，如果你需要同时运行多个 Future或链式调用多个 Future ，也可以通过无内存分配的状态机实现，

    /// 一个SimpleFuture，它会并发地运行两个Future直到它们完成
    ///
    /// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
    pub struct Join<FutureA, FutureB> {
        // 结构体的每个字段都包含一个Future，可以运行直到完成.
        // 等到Future完成后，字段会被设置为 None. 这样Future完成后，就不会再被轮询
        a: Option<FutureA>,
        b: Option<FutureB>,
    }

    impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // 尝试去完成一个 Future `a`
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take(); // takes the value out of the option, leaving a None in its place.
                }
            }

            // 尝试去完成一个 Future `b`
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // 两个 Future都已完成 - 我们可以成功地返回了
                Poll::Ready(())
            } else {
                // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
                // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
                Poll::Pending
            }
        }
    }
    /// 一个SimpleFuture, 它使用顺序的方式，一个接一个地运行两个Future
    //
    // 注意: 由于本例子用于演示，因此功能简单，`AndThenFut` 会假设两个 Future 在创建时就可用了.
    // 而真实的`Andthen`允许根据第一个`Future`的输出来创建第二个`Future`，因此复杂的多。
    pub struct AndThenFut<FutureA, FutureB> {
        first: Option<FutureA>,
        second: FutureB,
    }

    impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if let Some(first) = &mut self.first {
                match first.poll(wake) {
                    // 我们已经完成了第一个 Future， 可以将它移除， 然后准备开始运行第二个
                    Poll::Ready(()) => self.first.take(),
                    // 第一个 Future 还不能完成
                    Poll::Pending => return Poll::Pending,
                };
            }

            // 运行到这里，说明第一个Future已经完成，尝试去完成第二个
            self.second.poll(wake)
        }
    }

    // 真实的 `Future` trait
    // trait Future {
    //     type Output;
    //     fn poll(
    //         // 首先值得注意的地方是，`self`的类型从`&mut self`变成了`Pin<&mut Self>`:
    //         self: Pin<&mut Self>,
    //         // 其次将`wake: fn()` 修改为 `cx: &mut Context<'_>`:
    //         cx: &mut Context<'_>,
    //     ) -> Poll<Self::Output>;
    // }

    // 首先这里多了一个 Pin ，关于它我们会在后面章节详细介绍，
    // 现在你只需要知道使用它可以创建一个无法被移动的 Future ，
    // 因为无法被移动，所以它将具有固定的内存地址，意味着我们可以存储它的指针
    // (如果内存地址可能会变动，那存储指针地址将毫无意义！)，也意味着可以实现一个
    // 自引用数据结构: struct MyFut { a: i32, ptr_to_a: *const i32 }。
    // 而对于 async/await 来说，Pin 是不可或缺的关键特性。
    // 其次，从 wake: fn() 变成了 &mut Context<'_> 。意味着 wake 函数可以携带数据了，
    // 为何要携带数据？考虑一个真实世界的场景，一个复杂应用例如 web 服务器可能有数千连接同时在线，
    // 那么同时就有数千 Future 在被同时管理着，如果不能携带数据，
    // 当一个 Future 调用 wake 后，执行器该如何知道是哪个 Future 调用了 wake ,
    // 然后进一步去 poll 对应的 Future ？没有办法！那之前的例子为啥就可以使用没有携带数据的 wake ？
    // 因为足够简单，不存在歧义性。
    // 总之，在正式场景要进行 wake ，就必须携带上数据。 而 Context 类型通过提供一个 Waker 类型的值，
    // 就可以用来唤醒特定的的任务。

    // 对于 Future 来说，第一次被 poll 时无法完成任务是很正常的。
    // 但它需要确保在未来一旦准备好时，可以通知执行器再次对其进行 poll 进而继续往下执行，
    // 该通知就是通过 Waker 类型完成的。
    // Waker 提供了一个 wake() 方法可以用于告诉执行器：相关的任务可以被唤醒了，
    // 此时执行器就可以对相应的 Future 再次进行 poll 操作。

    example_study_async_2();
}

fn example_study_async_2() {
    println!("----------------example_study_async_2------------------");
    // 下面一起来实现一个简单的定时器 `Future` , 实现使用 Waker 来唤醒任务
    // 我们重新创建一个工程来演示：使用 `cargo new --lib timer_future` 来创建一个新工程

    // 下面我们将实现一个简单的执行器，它可以同时并发运行多个 `Future` 。
    // 例子中，需要用到 `futures` 包的 `ArcWake` 特征，它可以提供一个方便的途径去构建一个 `Waker` 。
    use {
        futures::{
            future::{BoxFuture, FutureExt},
            task::{waker_ref, ArcWake},
        },
        std::{
            future::Future,
            sync::mpsc::{sync_channel, Receiver, SyncSender},
            sync::{Arc, Mutex},
            task::Context,
            time::Duration,
        },
        // 引入之前实现的定时器模块
        timer_future::TimerFuture,
    };

    // 执行器需要从一个消息通道( channel )中拉取事件，然后运行它们。
    // 当一个任务准备好后（可以继续执行），它会将自己放入消息通道中，然后等待执行器 poll 。
    /// 任务执行器，负责从通道中接收任务然后执行
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    /// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }

    /// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
    struct Task {
        /// 进行中的Future，在未来的某个时间点会被完成
        ///
        /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
        /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
        /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
        ///
        /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
        future: Mutex<Option<BoxFuture<'static, ()>>>,

        /// 可以将该任务自身放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        // 任务通道允许的最大缓冲数(任务队列的最大长度)
        // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }
    // 下面再来添加一个方法用于生成 `Future` , 然后将它放入任务通道中:
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future)),
                task_sender: self.task_sender.clone(),
            });
            self.task_sender.send(task).expect("任务队列已满");
        }
    }
    // 在执行器 poll 一个 Future 之前，首先需要调用 wake 方法进行唤醒，
    // 然后再由 Waker 负责调度该任务并将其放入任务通道中。
    // 创建 Waker 的最简单的方式就是实现 ArcWake 特征，先来为我们的任务实现 ArcWake 特征，
    // 这样它们就能被转变成 Waker 然后被唤醒:
    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
            let cloned = arc_self.clone();
            arc_self.task_sender.send(cloned).expect("任务队列已满");
        }
    }
    // 当任务实现了 `ArcWake` 特征后，它就变成了 `Waker` ，
    // 在调用 `wake()` 对其唤醒后会将任务复制一份所有权( `Arc` )，
    // 然后将其发送到任务通道中。最后我们的执行器将从通道中获取任务，然后进行 `poll` 执行：
    impl Executor {
        fn run(&self) {
            while let Ok(task) = self.ready_queue.recv() {
                println!("执行器接收到任务");
                // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    // 基于任务自身创建一个 `LocalWaker`
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    //  BoxFuture<T> 是 Pin<Box<dyn Future<Output = T> + Send + 'static>> 的类型别名
                    // 通过调用 as_mut 方法，可以将上面的类型转换成 Pin<&mut dyn Future + Send + 'static>
                    if future.as_mut().poll(context).is_pending() {
                        // Future还没执行完，因此将它放回任务中，等待下次被poll
                        // (任务会被放回任务通道中，ArcWake 的 wake_by_ref 方法会被调用)
                        println!("任务还未完成，等待任务调用wake唤醒");
                        *future_slot = Some(future);
                    }
                }
            }
            println!("执行器退出");
        }
    }

    // 恭喜！我们终于拥有了自己的执行器，下面再来写一段代码使用该执行器去运行之前的定时器 `Future` ：
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        // // 可以使用await关键字是因为我们的为TimerFuture实现了Future特征
        TimerFuture::new(Duration::new(1, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来(通道已经关闭，退出循环)
    drop(spawner);

    //  Rust 的 Future 是惰性的，可以尝试等待一下，
    //  我们会发现上面的任务并没有被执行，因为执行器还没有运行起来。
    std::thread::sleep(Duration::new(2, 0));

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停1秒，接着打印 `done!`
    executor.run();
    // Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动。
    // 其中一个推动它的方式就是在 async 函数中使用 .await 来调用另一个 async 函数，
    // 但是这个只能解决 async 内部的问题，那么这些最外层的 async 函数，谁来推动它们运行呢？
    // 答案就是我们之前多次提到的执行器 executor (例如上一章节中的block_on函数) 。

    /*
        async函数本质上就是返回了一个 Future trait 对象，执行器可以通过poll的方式来推动其运行。
        执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们直到完成。
        最开始，执行器会先 poll 一次 Future ，后面就不会主动去 poll 了，
        而是等待 Future 通过调用 wake 函数来通知它可以继续，
        它才会继续去 poll 。这种 wake 通知然后 poll 的方式会不断重复，直到 Future 完成。
    */

    // - 在 Rust 中，async 是惰性的，直到执行器 poll 它们时，才会开始执行
    // - Waker 是 Future 被执行的关键，它可以链接起 Future 任务和执行器
    // - 当资源没有准备时，会返回一个 Poll::Pending
    // - 当资源准备好时，会通过 waker.wake 发出通知
    // - 执行器会收到通知，然后调度该任务继续执行，此时由于资源已经准备好，因此任务可以顺利往前推进了
}

fn study_async_and_await() {
    println!("------------------study_async_and_await------------------");
    // 目前已经有诸多语言都通过 async 的方式提供了异步编程，例如 JavaScript ，但 Rust 在实现上有所区别:
    // - Future 在 Rust 中是惰性的，只有在被轮询(poll)时才会运行，
    //   因此丢弃一个 future 会阻止它未来再被运行, 你可以将Future理解为一个在未来某个时间点被调度执行的任务。
    // - Async 在 Rust 中使用开销是零， 意味着只有你能看到的代码(自己的代码)才有性能损耗，
    //   你看不到的(async 内部实现)都没有性能损耗，
    //   例如，你可以无需分配任何堆内存、也无需任何动态分发来使用 async ，
    //   这对于热点路径的性能有非常大的好处，正是得益于此，Rust 的异步编程性能才会这么高。
    // - Rust 没有内置异步调用所必需的运行时，但是无需担心，
    //   Rust 社区生态中已经提供了非常优异的运行时实现，例如大明星 [tokio](https://tokio.rs/)
    // - 运行时同时支持单线程和多线程，这两者拥有各自的优缺点。

    // 虽然 `async` 和多线程都可以实现并发编程，后者甚至还能通过线程池来增强并发能力，
    // 但是这两个方式并不互通，从一个方式切换成另一个需要大量的代码重构工作，
    // 因此提前为自己的项目选择适合的并发模型就变得至关重要。

    // `OS` 线程非常适合少量任务并发，因为线程的创建和上下文切换是非常昂贵的，
    // 甚至于空闲的线程都会消耗系统资源。虽说线程池可以有效的降低性能损耗，但是也无法彻底解决问题。

    // 对于长时间运行的 CPU 密集型任务，例如并行计算，使用线程将更有优势。
    // 这种密集任务往往会让所在的线程持续运行，任何不必要的线程切换都会带来性能损耗，
    // 因此高并发反而在此时成为了一种多余。同时你所创建的线程数应该等于 CPU 核心数，
    // 充分利用 CPU 的并行能力，甚至还可以将线程绑定到 CPU 核心上，进一步减少线程上下文切换。

    // 而高并发更适合 IO 密集型任务，例如 web 服务器、数据库连接等等网络服务，
    // 因为这些任务绝大部分时间都处于等待状态，如果使用多线程，那线程大量时间会处于无所事事的状态，
    // 再加上线程上下文切换的高昂代价，让多线程做 IO 密集任务变成了一件非常奢侈的事。
    // 而使用async，既可以有效的降低 CPU 和内存的负担，又可以让大量的任务并发的运行，
    // 一个任务一旦处于IO或者其他等待(阻塞)状态，就会被立刻切走并执行另一个任务，
    // 而这里的任务切换的性能开销要远远低于使用多线程时的线程上下文切换。

    // 事实上, async 底层也是基于线程实现，但是它基于线程封装了一个运行时，可以将多个任务映射到少量线程上，
    // 然后将线程切换变成了任务切换，后者仅仅是内存中的访问，因此要高效的多。
    // 不过async也有其缺点，原因是编译器会为async函数生成状态机，
    // 然后将整个运行时打包进来，这会造成我们编译出的二进制可执行文件体积显著增大。

    // - 有大量 `IO` 任务需要并发运行时，选 `async` 模型
    // - 有部分 `IO` 任务需要并发运行时，选多线程，如果想要降低线程创建和销毁的开销，可以使用线程池
    // - 有大量 `CPU` 密集任务需要并行运行时，例如并行计算，选多线程模型，且让线程数等于或者稍大于 `CPU` 核心数
    // - 无所谓时，统一选多线程

    async fn _get_two_sites_async() {
        // 创建两个不同的`future`，你可以把`future`理解为未来某个时刻会被执行的计划任务
        // 当两个`future`被同时执行后，它们将并发的去下载目标页面
        let future_one = _download_async("https://www.foo.com");
        let future_two = _download_async("https://www.bar.com");
        // 同时运行两个`future`，直至完成
        use futures::join;
        join!(future_one, future_two);
    }

    // download
    async fn _download_async(url: &str) {
        println!("download_async: {}", url);
        // sleep random time
        // use rand::Rng;
        // use std::time::Duration;
        // let mut rng = rand::thread_rng();
        // let sleep_time = rng.gen_range(0..10);
        // println!("sleep time: {}", sleep_time);
        // tokio::time::sleep(Duration::from_secs(sleep_time)).await;
        println!("download_async: done");
    }

    // 目前主流的 async 运行时几乎都使用了多线程实现，相比单线程虽然增加了并发表现，
    // 但是对于执行性能会有所损失，因为多线程实现会有同步和切换上的性能开销，
    // 若你需要极致的 顺序执行 性能，那么 async 目前并不是一个好的选择。
    // 同样的，对于延迟敏感的任务来说，任务的执行次序需要能被严格掌控，而不是交由运行时去自动调度，
    // 后者会导致不可预知的延迟。正因为此，延迟敏感的任务非常依赖于运行时或操作系统提供调度次序上的支持。
    // 以上的两个需求，目前的 async 运行时并不能很好的支持，在未来可能会有更好的支持，
    // 但在此之前，我们可以尝试用多线程解决。

    // async/.await 简单入门
    println!("----------------async/.await 简单入门----------------");
    // async/.await 是 Rust 内置的语言特性，可以让我们用同步的方式去编写异步的代码。
    // 通过 async 标记的语法块会被转换成实现了Future特征的状态机。
    // 与同步调用阻塞当前线程不同，当Future执行并遇到阻塞时，它会让出当前线程的控制权，
    // 这样其它的Future就可以在该线程中运行，这种方式完全不会导致当前线程的阻塞。

    // 需要注意，异步函数的返回值是一个 Future，
    // 若直接调用该函数，不会输出任何结果，因为 Future 还未被执行：
    async fn do_something() {
        println!("go go go !");
    }
    // do_something();
    // 编译器给提示 Future 未被使用，那么到底该如何使用？答案是使用一个执行器( executor ):
    let future = do_something(); // 返回一个Future, 不会打印任何输出
    println!("sleep 1s");
    std::thread::sleep(std::time::Duration::from_secs(1));
    use futures::executor::block_on;
    block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出

    // 如果你要在一个async fn函数中去调用另一个async fn并等待其完成后再执行后续的代码，该如何做?
    async fn hello_cat() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("hello, kitty!");
    }

    async fn hello_world() {
        // `.await`并不会阻塞当前的线程，而是异步的等待`Future`的完成，
        // 在等待的过程中，该线程还可以继续执行其它的`Future B`，最终实现了并发处理的效果。
        // 调用hello_world()的代码块不会被阻塞，但这里会等待hello_cat()执行完成后才会继续往下执行
        hello_cat().await; // 等待`hello_cat`执行完成
        println!("hello, world!");
    }

    // main调用hello_world()，使用同步的代码顺序实现了异步的执行效果，
    // 非常简单、高效，而且很好理解，未来也绝对不会有回调地狱的发生。
    let future = hello_world(); // 调用async函数不会阻塞
    println!("async fn hello_world() called");
    block_on(future);

    // 一个例子
    example_async_await();
}

fn example_async_await() {
    println!("----------------async/.await 例子----------------");
    use futures::executor::block_on;

    struct Song {
        author: String,
        name: String,
    }

    async fn learn_song() -> Song {
        Song {
            author: "曲婉婷".to_string(),
            name: String::from("《我的歌声里》"),
        }
    }

    async fn sing_song(song: Song) {
        println!(
            "给大家献上一首{}的{} ~ {}",
            song.author, song.name, "你存在我深深的脑海里~ ~"
        );
    }

    async fn dance() {
        println!("唱到情深处，身体不由自主的动了起来~ ~");
    }

    async fn learn_and_sing() {
        // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，
        // 完全可以去执行跳舞的任务
        let song = learn_song().await;

        // 唱歌必须要在学歌之后
        sing_song(song).await;
    }

    async fn async_main() {
        let f1 = learn_and_sing(); // 调用async函数不会阻塞
        let f2 = dance(); // 调用async函数不会阻塞

        // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，
        // 那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，
        // 那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
        // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，
        // 并将其交给`main`函数中的`block_on`执行器
        futures::join!(f1, f2);
    }

    block_on(async_main());
}

mod config;
fn main() {
    // 线程的使用
    study_thread();

    // 单例模式
    study_singleton();

    // 互斥锁
    study_mutex();

    // 读写锁
    study_rwlock();

    // 信号量
    study_semaphore();

    // 原子类型
    study_atomic();

    // 基于 Send 和 Sync 的线程安全
    study_send_sync();

    // 条件变量(Condition Variable)
    study_condition_variable();

    // 线程之间的通信
    study_thread_communication();

    // scoped thread
    study_scoped_thread();

    config_example();
}

fn config_example() {
    let config = config::Config::new("192.168.1.111".to_string(), 8080);
    let cnf_writer = config.read_write();
    assert_eq!(cnf_writer.read().ip, "192.168.1.111");

    let config_reader = cnf_writer.clone_reader();

    cnf_writer.write().ip = "hhhhh".to_string();
    assert_eq!(config_reader.read().ip, "hhhhh");

    let cnf_writer2 = cnf_writer.clone();
    let config_reader2 = config_reader.clone();
    assert_eq!(cnf_writer2.read().ip, config_reader2.read().ip, "hhhhh");
}

fn study_scoped_thread() {
    println!("---------------------------scoped thread---------------------------");
    // std::thread::spawn，只接受满足'static 约束的闭包，也就意味着不可以捕获局部变量的借用。
    // 这样看起来 Sync 约束就看起来稍许鸡肋——如果只有&'static T 能在线程间传递，那
    // 只需要给所有&'static T 实现Send 不就好了，何必再加个Sync 呢。
    // 后来才意识到，哦，原来做好了线程同步（当然这个是很难保证的），
    // 是可以安全地通过借用访问跨线程的局部变量的。至少Sync 在非'static 的情况下也是有意义的。
    // 当然，这句话还有个误区：
    // 那只需要给所有 &'static T 实现Send 不就好了
    // 我们其实是可以通过内部可变，拿到一个&T 就可以修改T了。
    // 如果没有线程同步机制，直接让&T 在线程间共享，也会立刻GG。
    // 这就是为啥RefCell 不满足Sync ，而Mutex 满足的原因了。
    // 另外还有一些类型满足Sync 而不满足Send 的，常见于各种guard类型。
    // 比如说MutexGuard，代表一个锁。而锁在系统层面，跨线程释放是一种ub。
    // 如果MutexGuard 满足Send ，被“安全地”传到了另外一个线程上，然后释放，GG；
    // 但锁里面的东西，确实可以通过借用在各个线程中共享
    //只要里面里面的东西肯被共享就行——这一点老版本std没有考虑到，也GG了

    // 总之Send 和Sync 这两个trait在rust并发编程中是缺一不可的。
    // 但在并发编程中，仅有这两个trait，也是远远不够的。比如，如何描述线程的“父子”关系呢——来点scoped thread。

    // std::thread::scope
    // - 可以捕获scope以外的变量；
    // - scope会阻塞地等待所有子线程结束才返回；
    // - 子线程panic会向父线程传递

    // std::thread::scope 为线程创建作用域。
    // 传递给作用域的函数会被提供一个作用域对象，通过这个对象可以生成有作用域的线程。
    // 与无作用域的线程不同，有作用域的线程可以借用非 'static 数据，
    // 因为作用域保证所有线程都将在作用域的末尾被join。
    // 在这个函数返回之前，在作用域中生成的所有尚未被手动连接的线程都会被自动join。
    fn _test1() {
        let x = 1i32;
        std::thread::scope(|s| {
            s.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                println!("local x = {}", &x);
            });
        });
    }

    use std::thread;

    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x = a[0] + a[2];
        });
        // 执行顺序不确定
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}

fn study_send_sync() {
    // 在介绍Send特征之前，再来看看Arc为何可以在多线程使用，玄机在于两者的源码实现上：
    // Rc源码片段
    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}

    // Arc源码片段
    // unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
    // unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}

    // !代表移除特征的相应实现，上面代码中Rc<T>的Send和Sync特征被特地移除了实现，
    // 在rust标准库中Rc、cell等明确标记 !Sync ，因为这些东西线程是不安全的，你不能在多线程中用直接用。
    // 而Arc<T>则相反，实现了Sync + Send特征，这也是Arc<T>可以在多线程使用的原因。
    println!("---------------------------基于 Send 和 Sync 的线程安全---------------------------");
    // `Send`和`Sync`是 Rust 安全并发的重中之重，但是实际上它们只是
    // 标记特征(marker trait，该特征未定义任何行为，因此非常适合用于标记), 来看看它们的作用：
    // - 实现`Send`的类型可以在线程间安全的传递其所有权
    // - 实现`Sync`的类型可以在线程间安全的共享(通过引用)

    // 标记实现了Send的类型表明该类型的所有权可以移交给别的线程。
    // 若实现，则可以 包括但不限于 放进传入std::thread::spawn的闭包里。
    // T: Sync 等价于&T: Send，意义上是某个数据能否被多个线程同时不可变地访问。
    // 若实现，则可以 包括但不限于 放进Arc里并将Arc复制后传给别的线程。

    // 在 Rust 中，几乎所有类型都默认实现了`Send`和`Sync`，
    // 而且由于这两个特征都是可自动派生的特征(通过`derive`派生)，
    // 意味着一个复合类型(例如结构体), 只要它内部的所有成员都实现了`Send`或者`Sync`，
    // 那么它就自动实现了`Send`或`Sync`。

    // 以下几个是常见的没有实现的(事实上不止这几个，只不过它们比较常见):
    // - 裸指针两者都没实现，因为它本身就没有任何安全保证
    // - `UnsafeCell`不是`Sync`，因此`Cell`和`RefCell`也不是
    // - `Rc`两者都没实现(因为内部的引用计数器不是线程安全的)

    // 实现Send + !Sync的有Cell<T>和RefCell<T>。原因是它的内部可变性。
    // 改变内部数据只需要不可变引用即可，而多个线程同时改变内部数据会导致数据竞争。但是把它发给别的线程倒没什么事。

    // 实现!Send + Sync的有MutexGuard<T>。这种情况比较少，原因是它不能在别的线程析构。
    // 某些平台上互斥锁必须在原线程释放，但数据是Sync的，那仅访问数据不释放锁也没什么事。
    // 或者想象一下你自己封装了一个包含裸指针的类型，你通过内部实现让此类型可以在线程间安全的引用，
    // 但不能在其他线程释放内存，这时就实现了!Send + Sync。

    // 手动实现 `Send` 和 `Sync` 是不安全的，通常并不需要手动实现 Send 和 Sync trait，
    // 实现者需要使用`unsafe`小心维护并发安全保证。

    // 使用newtype为裸指针实现`Send`
    println!("---------------------------为裸指针实现`Send`---------------------------");
    use std::thread;

    #[derive(Debug)]
    struct MyBox(*mut u8);
    unsafe impl Send for MyBox {}

    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });
    t.join().unwrap();

    // 为裸指针实现`Sync`
    println!("---------------------------为裸指针实现`Sync`---------------------------");
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct MyBox2(*const u8);
    unsafe impl Send for MyBox2 {}
    unsafe impl Sync for MyBox2 {}

    let b = &MyBox2(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();

    // 简单总结下：
    // 1. 实现`Send`的类型可以在线程间安全的传递其所有权, 实现`Sync`的类型可以在线程间安全的共享(通过引用)
    // 2. 绝大部分类型都实现了`Send`和`Sync`，常见的未实现的有：裸指针、`Cell`、`RefCell`、`Rc` 等
    // 3. 可以为自定义类型实现`Send`和`Sync`，但是需要`unsafe`代码块
    // 4. 可以为部分 Rust 中的类型实现`Send`、`Sync`，但是需要使用`newtype`，例如文中的裸指针例子
}

fn study_atomic() {
    println!("---------------------------原子类型---------------------------");

    // 内存顺序
    study_memory_order();

    example_atomic1();

    example_atomic2();
}

fn study_memory_order() {
    println!("---------------------------内存顺序---------------------------");
    // 内存顺序是指 CPU 在访问内存时的顺序，该顺序可能受以下因素的影响：
    // - 代码中的先后顺序
    // - 编译器优化导致在编译阶段发生改变(内存重排序 reordering)
    // - 运行阶段因 CPU 的缓存机制导致顺序被打乱

    // 限定内存顺序的 5 个规则
    // 在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了`Ordering::Relaxed`用于限定内存顺序了，
    // 事实上，该枚举有 5 个成员:
    // - Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
    // - Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，
    //           但是它后面的操作可能被重排到它前面
    // - Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，
    //           但是它之前的操作却有可能被重排到它后面，往往和`Release`在不同线程中联合使用
    // - AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。
    //           比如你要对一个 `atomic` 自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
    // - SeqCst  顺序一致性， `SeqCst`就像是`AcqRel`的加强版，
    //           它不管原子操作是属于读取还是写入的操作，只要某个线程有用到`SeqCst`的原子操作，
    //           线程中该`SeqCst`操作前的数据操作绝对不会被重新排在该`SeqCst`操作之后，
    //           且该`SeqCst`操作后的数据操作也绝对不会被重新排在`SeqCst`操作前。

    // 内存屏障
    study_memory_barrier();

    // SeqCst
    study_seqcst();
}

fn example_atomic2() {
    println!("---------------------------原子类型---------------------------");
    // 和`Mutex`一样，`Atomic`的值具有内部可变性**，你无需将其声明为`mut`：
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Counter {
        count: u64,
    }
    let n = Mutex::new(Counter { count: 0 });

    n.lock().unwrap().count += 1;

    let n = AtomicU64::new(0);

    n.fetch_add(0, Ordering::Relaxed);
    println!("{}", n.load(Ordering::Relaxed));
}

fn example_atomic1() {
    println!("---------------------------原子类型---------------------------");
    // 原子类型的一个常用场景，就是作为全局变量来使用:
    // 在使用原子类型提供的原子操作时，需要额外传入一个Ordering枚举的变体实体。
    // 这个Ordering枚举可不是std::cmp这个模块下用来比大小的Ordering哦！
    // 而是位于std::sync::atomic模块下的Ordering枚举。
    // 这边的Ordering枚举是用来控制原子操作时所使用的「内存顺序」(Memory Ordering)的限制，
    // 共有Relaxed、Acquire、Release、AcqRel、SeqCst五种变体。
    // 内存顺序是指CPU在访问内存时的顺序，这个顺序不单纯是程序叙述的撰写顺序，
    // 可能还会因编译器优化，在编译阶段发生改变(reordering)，也可能在运行阶段时，因CPU的缓存机制而被打乱顺序。
    // Relaxed只会进行单纯的原子操作，并不会对内存顺序进行任何限制。
    // 换句话说，它可以最大幅度地保留编译器优化的程度，
    // 不过如果想要在多个原子操作间实现跨线程的同步机制，就得采用其它的内存顺序的限制方式了。
    // 借由Acquire和Release这两个内存顺序的限制，可以构筑出一对内存屏障(Memory Barrier)，
    // 或称内存栅栏(Memory Fence)，防止编译器和CPU将屏障前(Release)和屏障后(Acquire)中的数据
    // 操作重新排在屏障围成的范围之外。
    // https://magiclen.org/rust-atomic/

    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread::{self, JoinHandle};
    const N_TIMES: u64 = 10000000;
    const N_THREADS: usize = 10;

    static R: AtomicU64 = AtomicU64::new(0);

    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let s = std::time::Instant::now(); // Instant::now()返回一个Instant实例，它代表了当前的时间点。
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // 以上代码启动了数个线程，每个线程都在疯狂对全局变量进行加 1 操作, 最后将它与`线程数 * 加1次数`进行比较。
    // 原子操作 `Atomic`实现会比`Mutex`快，实际上在复杂场景下还能更快(甚至达到 4 倍的性能差距)！
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    //use std::ops::Sub;
    println!("{:?}", std::time::Instant::now() - s); // 两个时间点之间的时间差。
}

fn study_seqcst() {
    println!("---------------------------SeqCst---------------------------");
    // SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，
    // 只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，
    // 且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。
    // 另外，Acquire、Release和AcqRel等也可以与SeqCst搭配使用，来构筑出一对内存屏障。
}

fn study_memory_barrier() {
    println!("---------------------------内存屏障---------------------------");
    //原则上， Acquire用于读取，而Release用于写入。
    // 但是由于有些原子操作同时拥有读取和写入的功能，此时就需要使用AcqRel来设置内存顺序了。
    // 在内存屏障中被写入的数据，都可以被其它线程读取到，不会有CPU缓存的问题。
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::{self, JoinHandle};

    static mut DATA: u64 = 0;
    static READY: AtomicBool = AtomicBool::new(false);

    fn reset() {
        unsafe {
            DATA = 0;
        }
        READY.store(false, Ordering::Relaxed);
    }

    fn producer() -> JoinHandle<()> {
        thread::spawn(move || {
            unsafe {
                DATA = 100; // A
            }
            READY.store(true, Ordering::Release); // B: memory fence ↑
        })
    }

    fn consumer() -> JoinHandle<()> {
        thread::spawn(move || {
            while !READY.load(Ordering::Acquire) {} // C: memory fence ↓

            assert_eq!(100, unsafe { DATA }); // D
        })
    }
    for _ in 0..3 {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}

fn study_semaphore() {
    println!("---------------------------信号量---------------------------");
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[tokio::main]
    async fn example() {
        // 创建了一个容量为 3 的信号量
        let semaphore = Arc::new(Semaphore::new(3));
        let mut join_handles = Vec::new();

        for _ in 0..5 {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            join_handles.push(tokio::spawn(async move {
                //
                // 在这里执行任务...
                //
                drop(permit);
            }));
        }

        for handle in join_handles {
            handle.await.unwrap();
        }
    }
    // 关键在于：信号量的申请和归还，使用前需要申请信号量，如果容量满了，就需要等待；
    // 使用后需要释放信号量，以便其它等待者可以继续。
    example();
}

fn study_thread_communication() {
    println!("---------------------------线程之间的通信---------------------------");
    // 消息通道
    println!("---------------------------消息通道---------------------------");
    // 与 Go 语言内置的`chan`不同，Rust 是在标准库里提供了消息通道(`channel`)的实现。
    // 一个通道应该支持多个发送者和接收者。
    // 但是，在实际使用中，我们需要使用不同的库来满足诸如：多发送者 -> 单接收者，多发送者 -> 多接收者
    // 等场景形式，此时一个标准库显然就不够了，不过别急，让我们先从标准库讲起。

    // 多发送者，单接收者
    study_multi_sender_single_receiver();

    // 同步和异步通道
    study_sync_and_async_channel();

    // 多发送者，多接收者
    study_multi_sender_multi_receiver();
}

fn study_multi_sender_multi_receiver() {
    println!("---------------------------多发送者，多接收者---------------------------");
    // 如果你需要 mpmc(多发送者，多接收者)或者需要更高的性能，可以考虑第三方库:
    // - crossbeam-channel 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了`crossbeam`主仓库中
    // - flume, 官方给出的性能数据某些场景要比 crossbeam 更好些

    // crossbeam-channel
    // https://doraemon.xlog.app/crossbeam_channelmd
    println!("---------------------------crossbeam-channel---------------------------");
    // crossbeam-channel 是一个多生产者，多消费者通道，它提供了三种通道类型：
    // - unbounded: 无界通道，可以无限制的发送消息，直到内存耗尽
    // - bounded: 有界通道，可以指定通道的缓冲区大小，当缓冲区满时，发送者会阻塞
    // - array: 固定大小的通道，可以指定通道的缓冲区大小，当缓冲区满时，发送者会阻塞
    // 通道的发送和接收操作都是非阻塞的，如果通道已满或者为空，发送者和接收者都会立刻返回一个错误。
    // std::sync::mpsc 和 crossbeam::channel， 这些通道在等待消息时会阻塞当前的线程，
    // 因此不适用于 async 编程，只适合在多线程中使用。

    use crossbeam_channel::{bounded, unbounded};
    // Create a channel of unbounded capacity.
    // 创建一个无限容量的通道。
    let (s, r) = unbounded();
    // Send a message into the channel.
    s.send("Hello, world!").unwrap();
    // Receive the message from the channel.
    println!("recv:{}", r.recv().unwrap());

    // bounded creates a channel of bounded capacity,
    // i.e. there is a limit to how many messages it can hold at a time.
    // unbounded creates a channel of unbounded capacity,
    // i.e. it can hold any number of messages at a time.
    // Both functions return a Sender and a Receiver,
    // which represent the two opposite sides of a channel.

    // Create a channel that can hold at most 5 messages at a time.
    let (s, _r) = bounded(5);

    // Can send only 5 messages without blocking.
    for i in 0..5 {
        s.send(i).unwrap();
    }

    // Another call to `send` would block because the channel is full.
    // s.send(5).unwrap();

    // A special case is zero-capacity channel, which cannot hold any messages. Instead,
    // send and receive operations must appear at the same time in order to pair up and
    // pass the message over:
    // Create a zero-capacity channel.
    let (s, r) = bounded(0);
    // Sending blocks until a receive operation appears on the other side.
    std::thread::spawn(move || s.send("Hi!").unwrap());
    println!("recv:{}", r.recv().unwrap());

    // Sharing channels
    println!("---------------------------Sharing channels---------------------------");

    example_crossbeam_channel_1();
    example_crossbeam_channel_2();
    example_crossbeam_channel_3();
}

fn example_crossbeam_channel_3() {
    println!("---------------------------example 3 stopwatch---------------------------");
    // Prints the elapsed time every 1 second and quits on Ctrl+C.

    #[cfg(windows)] // signal_hook::iterator does not work on windows
    println!("This example does not work on Windows");

    #[cfg(not(windows))]
    fn main() {
        use std::io;
        use std::thread;
        use std::time::{Duration, Instant};

        use crossbeam_channel::{Receiver, bounded, select, tick};
        use signal_hook::consts::SIGINT;
        use signal_hook::iterator::Signals;

        // Creates a channel that gets a message every time `SIGINT` is signalled.
        fn sigint_notifier() -> io::Result<Receiver<()>> {
            let (s, r) = bounded(100);
            let mut signals = Signals::new(&[SIGINT])?;

            thread::spawn(move || {
                for _ in signals.forever() {
                    if s.send(()).is_err() {
                        break;
                    }
                }
            });

            Ok(r)
        }

        // Prints the elapsed time.
        fn show(dur: Duration) {
            println!("Elapsed: {}.{:03} sec", dur.as_secs(), dur.subsec_millis());
        }

        let start = Instant::now();
        let update = tick(Duration::from_secs(1));
        let ctrl_c = sigint_notifier().unwrap();

        loop {
            select! {
                recv(update) -> _ => {
                    show(start.elapsed());
                }
                recv(ctrl_c) -> _ => {
                    println!();
                    println!("Goodbye!");
                    show(start.elapsed());
                    break;
                }
            }
        }
    }
}
fn example_crossbeam_channel_2() {
    println!("---------------------------example 2 fibonacci---------------------------");
    // An asynchronous fibonacci sequence generator.
    use crossbeam_channel::{Sender, bounded};
    use std::thread;
    // Sends the Fibonacci sequence into the channel until it becomes disconnected.
    fn fibonacci(sender: Sender<u64>) {
        let (mut x, mut y) = (0, 1);
        while sender.send(x).is_ok() {
            let tmp = x;
            x = y;
            y += tmp;
        }
    }

    let (s, r) = bounded(0);
    thread::spawn(|| fibonacci(s));

    // Print the first 20 Fibonacci numbers.
    for num in r.iter().take(20) {
        println!("{}", num);
    }
}

fn example_crossbeam_channel_1() {
    println!("---------------------------example 1 matching---------------------------");
    // func main() {
    //     people := []string{"Anna", "Bob", "Cody", "Dave", "Eva"}
    //     match := make(chan string, 1) // Make room for one unmatched send.
    //     wg := new(sync.WaitGroup)
    //     for _, name := range people {
    //         wg.Add(1)
    //         go Seek(name, match, wg)
    //     }
    //     wg.Wait()
    //     select {
    //     case name := <-match:
    //         fmt.Printf("No one received %s’s message.\n", name)
    //     default:
    //         // There was no pending send operation.
    //     }
    // }

    // // Seek either sends or receives, whichever possible, a name on the match
    // // channel and notifies the wait group when done.
    // func Seek(name string, match chan string, wg *sync.WaitGroup) {
    //     select {
    //     case peer := <-match:
    //         fmt.Printf("%s received a message from %s.\n", name, peer)
    //     case match <- name:
    //         // Wait for someone to receive my message.
    //     }
    //     wg.Done()
    // }

    use crossbeam_channel::{Receiver, Sender, bounded, select};

    let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
    let (s, r) = bounded(1); // Make room for one unmatched send.

    // Either send my name into the channel or receive someone else's, whatever happens first.
    fn seek<T>(name: T, s: Sender<T>, r: Receiver<T>)
    where
        T: std::fmt::Display,
    {
        select! {
            recv(r) -> peer => println!("{} received a message from {}.", name, peer.unwrap()),
            send(s, name) -> _ => {}, // Wait for someone to receive my message.
        }
    }

    crossbeam_utils::thread::scope(|scope| {
        for name in people {
            let (s, r) = (s.clone(), r.clone());
            scope.spawn(move |_| seek(name, s, r));
        }
    })
    .unwrap();

    // Check if there is a pending send operation.
    if let Ok(name) = r.try_recv() {
        println!("No one received {}’s message.", name);
    }
}

fn study_multi_sender_single_receiver() {
    println!("---------------------------多发送者，单接收者---------------------------");
    // 标准库提供了通道`std::sync::mpsc`，其中`mpsc`是multiple producer, single consumer的缩写，
    // 代表了该通道支持多个发送者，但是只支持唯一的接收者。
    use std::sync::mpsc;
    use std::thread;
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    // `tx`,`rx`对应发送者和接收者，它们的类型由编译器自动推导。
    let (tx, rx) = mpsc::channel(); //tx means transmitter, rx means receiver
    let mut handles = vec![];

    // 创建线程，并发送消息
    let tx1 = tx.clone();
    handles.push(thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx1.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    }));

    let tx2 = tx.clone();
    handles.push(thread::spawn(move || {
        tx2.send(2).unwrap();
    }));

    for handle in handles {
        // 等待线程结束
        handle.join().unwrap();
    }
    // hung up
    drop(tx);

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
    println!("receive {}", rx.recv().unwrap());
    // returning an error if the corresponding channel has hung up.
    // 没有消息时，recv方法会阻塞当前线程，直到接收到消息
    println!("receive err {:?}", rx.recv());

    // 除了上述`recv`方法，还可以使用`try_recv`尝试接收一次消息，
    // 该方法并不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误。
    println!("receive err {:?}", rx.try_recv()); // receive Err(Disconnected)

    // 使用通道来传输数据，一样要遵循 Rust 的所有权规则：
    // - 若值的类型实现了`Copy`特征，则直接复制一份该值，然后传输过去，例如之前的`i32`类型
    // - 若值没有实现`Copy`，则它的所有权会被转移给接收端，在发送端继续使用该值将报错。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s).unwrap();
        // println!("val is {}", s); // borrow of moved value: `s`
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn study_sync_and_async_channel() {
    println!("---------------------------同步和异步通道---------------------------");
    // 之前我们使用的都是异步通道：无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞。
    // 异步通道的缓冲上限取决于你的内存大小，不要撑爆就行。
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("thread 发送之前");
        tx.send(1).unwrap();
        println!("thread 发送之后");
    });

    println!("main 睡眠之前");
    thread::sleep(Duration::from_secs(1));
    println!("main 睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();

    // 与异步通道相反，同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞。
    // 0表示无缓冲区，若有缓冲区，则在缓冲区满时才会阻塞。
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("thread 发送之前");
        tx.send(1).unwrap();
        println!("thread 发送之后");
    });

    println!("main 睡眠之前");
    thread::sleep(Duration::from_secs(1));
    println!("main 睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn study_thread() {
    println!("---------------------------线程的使用---------------------------");
    // 创建线程
    // 线程内部的代码使用闭包来执行
    // `main` 线程一旦结束，程序就立刻结束。
    // `thread::sleep` 会让当前线程休眠指定的时间，随后其它线程会被调度运行。
    // 线程调度的方式往往取决于你使用的操作系统。总之，千万不要依赖线程的执行顺序。
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // 阻塞，直到它等待的子线程的结束
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // `move` 关键字在闭包中的使用可以让该闭包拿走环境中某个值的所有权，
    // 同样地，你可以使用 `move` 来将所有权从一个线程转移到另外一个线程。
    let v = vec![1, 2, 3];

    let handle = std::thread::spawn(move || {
        println!("Here's a vector: {:?}", v); // v 的所有权被转移到了线程内部
    });

    // 函数名取名为 `join` 含义是等待多线程结束并加入主线程。
    handle.join().unwrap();

    // 多线程的性能
    println!("---------------------------多线程的性能---------------------------");
    // 据不精确估算，创建一个线程大概需要 0.24 毫秒，
    // 随着线程的变多，这个值会变得更大，因此线程的创建耗时并不是不可忽略的。

    // 因为 CPU 的核心数限制，当任务是 CPU 密集型时，就算线程数超过了 CPU 核心数，也并不能帮你获得更好的性能，
    // 因为每个线程的任务都可以轻松让 CPU 的某个核心跑满，既然如此，让线程数等于 CPU 核心数是最好的。
    // 但是当你的任务大部分时间都处于阻塞状态时，就可以考虑增多线程数量，
    // 这样当某个线程处于阻塞状态时，会被切走，进而运行其它的线程，从而提高整体的效率。

    // 事实上，对于网络 IO 情况，一般都不再使用多线程的方式了，毕竟操作系统的线程数是有限的，
    // 意味着并发数也很容易达到上限，而且过多的线程也会导致线程上下文切换的代价过大，
    // 使用 `async/await` 的 `M:N` 并发模型，就没有这个烦恼。

    // 线程屏障(Barrier)
    println!("---------------------------线程屏障(Barrier)---------------------------");
    // 在 Rust 中，可以使用 `Barrier` 让多个线程都执行到某个点后，才继续一起往后执行：
    let mut handles = Vec::with_capacity(6);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(6));
    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(std::thread::spawn(move || {
            println!("before wait");
            b.wait(); // 等所有的线程都打印出before wait后，各个线程再继续执行。
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 线程局部变量(Thread Local Variable)
    println!("----------------------线程局部变量(Thread Local Variable)----------------------");
    // 使用 `thread_local` 宏可以初始化线程局部变量，然后在线程内部使用该变量的 `with` 方法获取变量值：
    thread_local!(static FOO: std::cell::RefCell<u32> = std::cell::RefCell::new(1));
    // `FOO` 即是我们创建的线程局部变量，每个新的线程访问它时，都会使用它的初始值作为开始，
    // 各个线程中的 `FOO` 值彼此互不干扰。
    // 注意 `FOO` 使用 `static` 声明为生命周期为 `'static` 的静态变量

    // main线程中修改为2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值。
    let t = std::thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
            println!("t: {:?}", *f.borrow());
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        println!("main: {:?}", *f.borrow());
    });

    // 结构体中使用线程局部变量
    println!("----------------------结构体中使用线程局部变量----------------------");
    struct Foo;
    impl Foo {
        thread_local! {
            static FOO: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
        }
    }
    Foo::FOO.with(|x| println!("{:?}", x));

    // 通过引用的方式使用线程局部变量
    println!("----------------------通过引用的方式使用线程局部变量----------------------");
    thread_local! {
        static FOO2:  std::cell::RefCell<usize> =  std::cell::RefCell::new(0);
    }
    struct Bar {
        foo: &'static std::thread::LocalKey<std::cell::RefCell<usize>>,
    }
    impl Bar {
        fn constructor() -> Self {
            Self { foo: &FOO2 }
        }
    }
    let bar = Bar::constructor();
    bar.foo.with(|f| println!("{:?}", f));

    // 第三方库 thread-local
    println!("----------------------第三方库 thread-local----------------------");
    // 上面可以注意到，线程中对 线程局部变量 的使用是通过借用的方式。
    // 除了标准库外，一位大神还开发了 thread-local 库，它允许每个线程持有值的独立拷贝：
    use thread_local::ThreadLocal;
    let tls = std::sync::Arc::new(ThreadLocal::new());

    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        std::thread::spawn(move || {
            // 将计数器加1
            let cell = tls2.get_or(|| std::cell::Cell::new(0));
            cell.set(cell.get() + 1);
        })
        .join()
        .unwrap();
    }

    // 该库不仅仅使用了值的拷贝，而且还能自动把多个拷贝汇总到一个迭代器中，最后进行求和，非常好用。
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = std::sync::Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());

    // 和为5
    println!("total: {}", total);
}

fn study_rwlock() {
    println!("----------------------读写锁----------------------");
    let lock = std::sync::RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会panic，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    } // 写锁在此处被drop
}

fn study_singleton() {
    println!("----------------------只被调用一次的函数----------------------");
    use std::sync::Once;
    use std::thread;

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });
    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // 代码运行的结果取决于哪个线程先调用 `INIT.call_once`
    println!("VAL: {}", unsafe { VAL });
}

fn study_mutex() {
    println!("----------------------互斥锁----------------------");
    // Mutex<T> 可以支持修改内部数据，当结合 Arc<T> 一起使用时，可以实现多线程的内部可变性。
    // Mutex当全局变量使用时，不需要Arc。
    use std::sync::{Arc, Mutex};
    use std::thread;
    let lock = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let lock = lock.clone();
        // RAII （Resource Acquisition Is Initialization）,也称为“资源获取就是初始化”，
        // 是C++语言的一种管理资源、避免泄漏的惯用法。
        let handle = thread::spawn(move || {
            {
                // Mutex<T> 是一个智能指针，准确的说是 lock() 返回一个智能指针MutexGuard<T>。
                // 获取锁并返回一个RAII guard，
                // 当 guard 离开作用域时，它会自动释放锁。
                let mut num = lock.lock().unwrap();
                *num += 1;
            }
            // 锁在这里离开作用域，自动释放
            println!("thread {} finished", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *lock.lock().unwrap());
}

fn study_condition_variable() {
    println!("----------------------条件变量(Condition Variable)----------------------");
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    // thread::sleep(std::time::Duration::from_secs(2));
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("waiting");
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");

    // example
    example_condition_variable();
}

fn example_condition_variable() {
    println!("----------------------条件变量(Condition Variable)----------------------");
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut m = { *cflag.lock().unwrap() };
        let mut counter = 0;

        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }

            {
                m = false;
                *cflag.lock().unwrap() = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    // 通过主线程来触发子线程实现交替打印输出
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}

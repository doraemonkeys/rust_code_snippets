#[tokio::main]
async fn main() {
    study_hello_tokio().await;

    // Tokio 中的 I/O 操作
    study_tokio_io().await;
}

async fn study_tokio_io() {
    println!("--------------------tokio_io-------------------");
    // Tokio 中的 I/O 操作和 std 在使用方式上几无区别，最大的区别就是前者是异步的，
    // 例如 Tokio 的读写特征分别是 AsyncRead 和 AsyncWrite:
    // - 有部分类型按照自己的所需实现了它们: TcpStream，File，Stdout
    // - 还有数据结构也实现了它们：Vec<u8>、&[u8]，这样就可以直接使用这些数据结构作为读写器( reader / writer)

    // AsyncRead 和 AsyncWrite
    println!("--------------------AsyncRead 和 AsyncWrite-------------------");
    // 这两个特征为字节流的异步读写提供了便利，通常我们会使用 AsyncReadExt 和 AsyncWriteExt 提供的工具方法，
    // 这些方法都使用 async 声明，且需要通过 .await 进行调用。

    // AsyncReadExt::read 是一个异步方法可以将数据读入缓冲区( buffer )中，然后返回读取的字节数。
    async fn _test1() -> tokio::io::Result<()> {
        use tokio::io::AsyncReadExt;
        let mut f = tokio::fs::File::open("foo.txt").await?;
        let mut buffer = [0; 10];

        // 由于 buffer 的长度限制，当次的 `read` 调用最多可以从文件中读取 10 个字节的数据
        let n = f.read(&mut buffer[..]).await?;

        println!("The bytes: {:?}", &buffer[..n]);
        Ok(())
    }

    // AsyncReadExt::read_to_end 方法会从字节流中读取所有的字节，直到遇到 EOF ：
    async fn _test2() -> tokio::io::Result<()> {
        use tokio::io::AsyncReadExt;
        let mut f = tokio::fs::File::open("foo.txt").await?;
        let mut buffer = Vec::new();

        // 读取整个文件的内容
        f.read_to_end(&mut buffer).await?;
        Ok(())
    }

    // AsyncWriteExt::write 异步方法会尝试将缓冲区的内容写入到写入器( writer )中，同时返回写入的字节数:
    async fn _test3() -> tokio::io::Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create("foo.txt").await?;

        let n = file.write(b"some bytes").await?;

        println!("Wrote the first {} bytes of 'some bytes'.", n);
        Ok(())
    }

    // AsyncWriteExt::write_all` 将缓冲区的内容全部写入到写入器中：
    async fn _test4() -> tokio::io::Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create("foo.txt").await?;

        file.write_all(b"some bytes").await?;
        Ok(())
    }

    // 另外，和标准库一样， tokio::io 模块包含了多个实用的函数或 API，可以用于处理标准输入/输出/错误等:
    // 例如，tokio::io::copy 异步的将读取器( reader )中的内容拷贝到写入器( writer )中。
    async fn _test5() -> tokio::io::Result<()> {
        let mut reader: &[u8] = b"hello";
        let mut writer: Vec<u8> = vec![];

        tokio::io::copy(&mut reader, &mut writer).await?;
        Ok(())
    }
}

async fn study_hello_tokio() {
    println!("--------------------hello_tokio-------------------");
    // 一个 Tokio 任务是一个异步的绿色线程，它们通过 tokio::spawn 进行创建，
    // 该函数会返回一个 JoinHandle 类型的句柄，调用者可以使用该句柄跟创建的任务进行交互。
    // spawn 函数的参数是一个 async 语句块，该语句块甚至可以返回一个值，
    // 然后调用者可以通过 JoinHandle 句柄获取该值:
    let handle = tokio::spawn(async { 10086 });

    let out = handle.await.unwrap();
    println!("GOT {}", out);

    // 任务是调度器管理的执行单元。spawn生成的任务会首先提交给调度器，然后由它负责调度执行。
    // 需要注意的是，执行任务的线程未必是创建任务的线程，任务完全有可能运行在另一个不同的线程上，
    // 而且任务在生成后，它还可能会在线程间被移动。

    // 任务在 Tokio 中远比看上去要更轻量，例如创建一个任务仅仅需要一次 64 字节大小的内存分配。
    // 因此应用程序在生成任务上，完全不应该有任何心理负担，
    // 除非你在一台没那么好的机器上疯狂生成了几百万个任务。。。

    // 当使用 Tokio 创建一个任务时，该任务类型的生命周期必须是 'static。
    // 意味着，在任务中不能使用外部数据的引用:
    let v = vec![1, 2, 3];

    // 默认情况下，变量并不是通过 move 的方式转移进 async 语句块的，
    // v 变量的所有权依然属于 main 函数，因为任务内部的 println! 是通过借用的
    // 方式使用了 v，但是这种借用并不能满足 'static 生命周期的要求。
    // tokio::spawn 生成的任务必须实现 Send 特征，因为当这些任务
    // 在 .await 执行过程中发生阻塞时，Tokio 调度器会将任务在线程间移动。
    tokio::task::spawn(async move {
        // 通过 move 关键字，将 v 变量的所有权转移进 async 语句块
        println!("Here's a vec: {:?}", v);
    });

    println!("--------------------hello_tokio-------------------");
    tokio::spawn(async {
        // 语句块的使用强制了 rc 会在 .await 被调用前就被释放，
        // 因此 rc 并不会影响 .await的安全性
        {
            let rc = std::rc::Rc::new("hello"); //rc未实现Send
            println!("{}", rc);
        }
        // 很多错误都是因为 .await 引起的，其实你只要记住，在 .await 执行期间，
        // 任务可能会在线程间转移，所以必须实现 Send 特征，这样才能保证安全性。
        async fn yield_now() {}
        // rc 的作用范围已经失效，因此当任务让出所有权给当前线程时，它无需作为状态被保存起来
        yield_now().await;
    });

    // 下面的代码不工作
    use std::sync::{Mutex, MutexGuard};
    async fn _increment_and_do_stuff1(mutex: &Mutex<i32>) {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
        drop(lock); // 释放锁，编译器在这里不够聪明，目前它只能根据作用域的范围来判断

        // 其实这样是可以的(引用类型在最后一次使用后，作用域就会结束)
        // *mutex.lock().unwrap() += 1;
        _do_something_async().await;
    }
    async fn _do_something_async() {}
    // 如果你要 spawn 一个任务来执行_increment_and_do_stuff函数的话，会报错:
    // 编译器在这里不够聪明，目前它只能根据作用域的范围来判断，drop 虽然释放了锁，
    // 但是锁的作用域依然会持续到函数的结束，未来也许编译器会改进，但是现在至少还是不行的。
    // tokio::spawn(async {
    //     let mutex = Mutex::new(0);
    //     _increment_and_do_stuff1(&mutex).await;
    // });

    // 重构代码：在 .await 期间不持有锁
    println!("--------------------refactor-------------------");
    // 之前的代码其实也是为了在 `.await` 期间不持有锁，但是我们还有更好的实现方式，
    // 例如，你可以把 `Mutex` 放入一个结构体中，并且只在该结构体的非异步方法中使用该锁:
    // 其实本质就是引用类型在最后一次使用后，作用域就会结束。
    struct CanIncrement {
        mutex: Mutex<i32>,
    }
    impl CanIncrement {
        // 该方法不是 `async`
        fn increment(&self) {
            let mut lock = self.mutex.lock().unwrap();
            *lock += 1;
        }
    }

    async fn increment_and_do_stuff2(can_incr: &CanIncrement) {
        can_incr.increment();
        // 引用类型在最后一次使用后，作用域就会结束
        _do_something_async().await;
    }
    // 现在，我们可以在 `.await` 期间不持有锁了:
    tokio::spawn(async {
        let can_incr = CanIncrement {
            mutex: Mutex::new(0),
        };
        increment_and_do_stuff2(&can_incr).await;
    });

    // 使用 Tokio 提供的异步锁
    println!("--------------------tokio::sync::Mutex-------------------");
    // Tokio 提供的锁最大的优点就是：它可以在 `.await` 执行期间被持有，而且不会有任何问题。
    // 但是代价就是，这种异步锁的性能开销会更高，同时也会更加容易造成死锁。
    // 因此如果可以，使用之前的两种方法来解决会更好。

    // 下面的代码会编译
    // 但是就这个例子而言，之前的方式会更好
    // 注意，这里使用的是 Tokio 提供的锁
    async fn _increment_and_do_stuff(mutex: &tokio::sync::Mutex<i32>) {
        let mut lock = mutex.lock().await;
        *lock += 1;

        _do_something_async().await;
    } // 锁在这里被释放

    // tokio死锁的例子
    println!("--------------------tokio死锁的例子-------------------");
    // 由于 Tokio 的异步锁可以在 `.await` 执行期间被持有，因此在某些情况下，
    // 你可能会不小心的在同一个任务中多次获取锁，这样就会造成死锁。
    // 例如:
    async fn _deadlock_example(mutex: &tokio::sync::Mutex<i32>) {
        let mut lock = mutex.lock().await;
        *lock += 1;

        // 这里会发生死锁，因为在 `.await` 执行期间，锁已经被持有了。
        // 这种情况一般发生的很隐蔽，因为rust编译器不会报错，运行时也不会Panic，
        // 发生死锁的线程就这样一直被阻塞。
        let mut lock2 = mutex.lock().await;
        *lock2 += 1;
    }

    // eample:将锁进行分片
    study_tokio_split_lock();
}

fn study_tokio_split_lock() {
    println!("--------------------split_lock-------------------");
    // 锁如果在多个 .await 过程中持有，应该使用 Tokio 提供的锁，
    // 原因是 .await的过程中锁可能在线程间转移，若使用标准库的同步锁存在死锁的可能性，
    // 例如某个任务刚获取完锁，还没使用完就因为 .await 让出了当前线程的所有权，
    // 结果下个任务又去获取了锁，造成死锁。
    // - 锁竞争不多的情况下，使用 std::sync::Mutex
    // - 锁竞争多，可以考虑使用三方库提供的性能更高的锁，例如 parking_lot::Mutex

    // 当一个锁竞争触发后，当前正在执行任务(请求锁)的线程会被阻塞，并等待锁被前一个使用者释放。
    // 这里的关键就是：锁竞争不仅仅会导致当前的任务被阻塞，还会导致执行任务的线程被阻塞，
    // 因此该线程准备执行的其它任务也会因此被阻塞！
    // 默认情况下，Tokio 调度器使用了多线程模式，此时如果有大量的任务都需要访问同一个锁，
    // 那么锁竞争将变得激烈起来。当然，你也可以使用 current_thread 运行时设置，
    // 在该设置下会使用一个单线程的调度器(执行器)，所有的任务都会创建并执行在当前线程上，因此不再会有锁竞争。

    // current_thread 是一个轻量级、单线程的运行时，当任务数不多或连接数不多时是一个很好的选择。
    // 例如你想在一个异步客户端库的基础上提供给用户同步的 API 访问时，该模式就很适用

    // 当同步锁的竞争变成一个问题时，使用 Tokio 提供的异步锁几乎并不能帮你解决问题，此时可以考虑如下选项：
    // - 创建专门的任务并使用消息传递的方式来管理状态
    // - 将锁进行分片
    fn _split_lock() {
        use std::collections::HashMap;
        use std::sync::Arc;
        use std::sync::Mutex;
        // 在我们的例子中，由于每一个 key 都是独立的，因此对锁进行分片将成为一个不错的选择:
        type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

        fn new_sharded_db(num_shards: usize) -> ShardedDb {
            let mut db = Vec::with_capacity(num_shards);
            for _ in 0..num_shards {
                db.push(Mutex::new(HashMap::new()));
            }
            Arc::new(db)
        }
        // 在这里，我们创建了 N 个不同的存储实例，每个实例都会存储不同的分片数据，
        // 例如我们有a-i共 9 个不同的 key, 可以将存储分成 3 个实例，
        // 那么第一个实例可以存储 a-c，第二个d-f，以此类推。
        // 在这种情况下，访问 b 时，只需要锁住第一个实例，
        // 此时二、三实例依然可以正常访问，因此锁被成功的分片了。
        // 在分片后，使用给定的 key 找到对应的值就变成了两个步骤：
        // 首先，使用 key 通过特定的算法寻找到对应的分片，然后再使用该 key 从分片中查询到值:
        let db = new_sharded_db(3);
        let key = "hello";
        fn hash(_key: &str) -> usize {
            // ...
            0
        }
        let mut shard = db[hash(key) % db.len()].lock().unwrap();
        let value = Vec::from("world");
        shard.insert(key.to_string(), value);
        // 一致性哈希
        // 这里我们使用 hash 算法来进行分片，但是该算法有个缺陷：分片的数量不能变，
        // 一旦变了后，那之前落入分片 1 的key很可能将落入到其它分片中，最终全部乱掉。
        // 此时你可以考虑 dashmap，它提供了更复杂、更精妙的支持分片的hash map。
    }
}

use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
    workers: Vec<Option<Worker>>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                if message.is_err() {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
                let job = message.unwrap();
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}

// - ThreadPool 拥有不错的文档注释，甚至包含了可能 panic 的情况，
//   通过 cargo doc --open 可以访问文档注释
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // 学过多线程一章后，大家应该知道 `thread::spawn` 虽然是生成线程最好的方式，
        // 但是它会立即执行传入的任务，然而，在我们的使用场景中，创建线程和执行任务明显是要分离的，
        // 因此标准库看起来不再适合。
        // 可以考虑创建一个 `Worker` 结构体，作为 `ThreadPool` 和任务线程联系的桥梁，
        // 它的任务是获得将要执行的代码，然后在具体的线程中去执行。
        // 想象一个场景：一个餐馆，`Worker` 等待顾客的点餐，然后将具体的点餐信息传递给厨房，感觉类似服务员？
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Some(Worker::new(id, Arc::clone(&receiver))));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if self.sender.is_none() {
            return;
        }
        let res = self.sender.as_ref().unwrap().send(job);
        if res.is_err() {
            println!("Error: {:?}", res);
        }
    }
}

// 当线程池被 drop 时，需要等待所有的子线程完成它们的工作，然后再退出
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            // 对于 Option 类型，可以使用 take 方法拿走内部值的所有权，
            // 同时留下一个 None 在风中孤独凌乱。
            if let Some(worker) = worker.take() {
                // 虽然调用了 join ，但是目标线程依然不会停止，原因在于它们在无限的 loop 循环等待，
                // 需要借用 channel 的 drop 机制：释放 sender发送端后，receiver 接收端会收到报错，
                // 然后再退出即可。
                println!("Shutting down worker {}", worker.id);
                worker.thread.join().unwrap();
            }
        }
        println!("All workers are shutdown.");
    }
}

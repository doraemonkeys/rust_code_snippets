use std::sync::{Arc, Barrier, Mutex};

// Barrier 可以用 wait 来控制 n 个线程的同步，数量需要提前指明。
// 当调用 wait 时，如果不是 n 个都完成，就会一直阻塞当前线程，直到第 n 个 wait 调用，才能进行后续操作。
// 这种机制就像在多个线程中插入了一道屏障，当所有线程都执行到这里时，才能解除屏障继续向后执行。
// 这样实现在线程数量大的时候是会有比较明显的性能开销的，底层是使用 condvar+mutex 来实现的。

fn main() {
    let numthreads = 10;
    let my_mutex = Arc::new(Mutex::new(0));

    // We use a barrier to ensure the readout happens after all writing
    let barrier = Arc::new(Barrier::new(numthreads + 1));

    for i in 0..numthreads {
        let my_barrier = barrier.clone();
        let my_lock = my_mutex.clone();
        std::thread::spawn(move || {
            let mut guard = my_lock.lock().unwrap();
            *guard += 1;

            // Release the lock to prevent a deadlock
            drop(guard);
            println!("thread {} is ready", i);
            // Blocks the current thread until all threads have rendezvoused here.
            my_barrier.wait();
            println!("thread {} is done", i)
        });
    }

    // A barrier will block `n`-1 threads which call [`wait()`] and then wake
    // up all threads at once when the `n`th thread calls [`wait()`].
    barrier.wait();

    let answer = { *my_mutex.lock().unwrap() };
    assert_eq!(answer, numthreads);
}

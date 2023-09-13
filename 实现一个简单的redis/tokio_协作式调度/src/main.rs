use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

use rand::Rng;
use tokio::time::Instant;

async fn worker(i: i32) {
    println!("current thread: {}", thread::current().name().unwrap());
    let mut count = 0;
    loop {
        let start = Instant::now();
        let mut num: i64 = 0;
        loop {
            num += rand::thread_rng().gen_range(1..=100);
            let elapsed = start.elapsed();
            if elapsed.as_millis() > 200 {
                break;
            }
        }
        println!("worker {}: {}", i, num);
        count += 1;
        if count > 20 {
            println!("worker {} exit", i);
            break;
        }
    }
}

fn main() -> io::Result<()> {
    // 默认情况下，Tokio 启动的工作线程数和 CPU 核数相等，也可以自定义。
    let runtime = Builder::new_multi_thread().worker_threads(1).build()?;

    runtime.spawn(worker(1));
    // tokio 是协作式调度，所以如果想要让 worker 2 有机会执行，就需要让 worker 1 主动让出 CPU。
    runtime.spawn(worker(2));

    println!("current thread: {}", thread::current().name().unwrap());
    let mut count = 0;
    loop {
        let start = Instant::now();
        let mut num: i64 = 0;
        loop {
            num += rand::thread_rng().gen_range(1..=100);
            let elapsed = start.elapsed();
            if elapsed.as_millis() > 200 {
                break;
            }
        }
        println!("worker main: {}", num);
        count += 1;
        if count > 20 {
            println!("worker main exit");
            break;
        }
    }

    println!("current thread: {}", thread::current().name().unwrap());
    // thread::sleep(Duration::from_secs(4444));
    // runtime.shutdown_timeout(Duration::from_secs(4));

    println!("exit main");
    Ok(())
}

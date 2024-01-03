fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let j1 = rt.spawn(async {
        other_worker().await;
    });

    println!("main: other_worker spawned");

    rt.spawn(async {
        http_task().await;
    });
    println!("main: http_task spawned");

    rt.block_on(j1).unwrap();
}

async fn other_worker() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("otherWorker: {}", counter);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

async fn http_task() {
    let mut counter = 0;
    while counter < 1000000000 {
        counter += 1;
        if counter % 50000000 == 0 {
            println!("httpTask: {}", counter);
        }
    }
    println!("httpTask: try to get http://www.baidu.com");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("httpTask: get success,begin to parse html");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("httpTask: done");
}

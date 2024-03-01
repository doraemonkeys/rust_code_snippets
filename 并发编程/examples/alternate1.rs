use std::sync::{mpsc, Mutex};

fn worker(sender: mpsc::SyncSender<i32>, receiver: &Mutex<mpsc::Receiver<i32>>, id: i32) {
    loop {
        let i = receiver.lock().unwrap().recv().unwrap();
        println!("worker {}: {}", id, i);
        sender.send(i + 1).unwrap();
    }
}

// 两个线程/协程交替打印数字
fn main() {
    let (tx, rx) = mpsc::sync_channel(0);
    let rx = Mutex::new(rx);
    std::thread::scope(|s| {
        s.spawn(|| worker(tx.clone(), &rx, 1));
        tx.send(0).unwrap();
        worker(tx.clone(), &rx, 2);
    });
}

use std::sync::mpsc;

fn worker(sender: mpsc::SyncSender<i32>, receiver: UnsafeRef<mpsc::Receiver<i32>>, id: i32) {
    loop {
        let i = receiver.recv().unwrap();
        println!("worker {}: {}", id, i);
        sender.send(i + 1).unwrap();
    }
}

// unsafe实现，去除receiver加的锁
fn main() {
    let (tx, rx) = mpsc::sync_channel(0);
    let rx = UnsafeRef::new(&rx);
    std::thread::scope(|s| {
        {
            let tx = tx.clone();
            let rx = rx.clone();
            s.spawn(|| worker(tx, rx, 1));
        }
        tx.send(0).unwrap();
        worker(tx, rx, 2);
    });
}

struct UnsafeRef<T> {
    inner: *const T,
}

impl<T> UnsafeRef<T> {
    fn new(inner: *const T) -> Self {
        Self { inner }
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}
impl<T> std::ops::Deref for UnsafeRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

unsafe impl<T> std::marker::Sync for UnsafeRef<T> {}
unsafe impl<T> std::marker::Send for UnsafeRef<T> {}

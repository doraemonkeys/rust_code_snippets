use std::future::Future;
use std::mem::take;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(task1());
    // tokio::spawn(task2()).await.unwrap();
    tokio::spawn(tokio::task::unconstrained(task2()))
        .await
        .unwrap();
    // let _fut = tokio::task::unconstrained(task2());
    // task1().await;
    // fut.await;
}

async fn task1() {
    loop {
        println!("task1");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

async fn task2() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    loop {
        let _ = tx.send(());
        // This will always be ready. If coop was in effect, this code would be forced to yield
        // periodically. However, if left unconstrained, then this code will never yield.
        rx.recv().await;

        // let poll_always_ok = PollAlwaysOk::new();
        // poll_always_ok.await;
        // tokio::task::yield_now().await; // 手动yield 插入 yield point

        // println!("task2");
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
}

struct PollAlwaysOk(bool);

impl PollAlwaysOk {
    fn new() -> Self {
        Self(true)
    }
}

impl Future for PollAlwaysOk {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            self.0 = false;
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

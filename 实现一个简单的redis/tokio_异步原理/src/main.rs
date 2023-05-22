use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// 下面来一起实现个五脏俱全的 `Future`，它将：
// 1. 等待某个特定时间点的到来 2. 在标准输出打印文本 3. 生成一个字符串
struct Delay {
    when: Instant,
}

// 为我们的 Delay 类型实现 Future 特征
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            // 时间到了，Future 可以结束
            println!("Hello world");
            // Future 执行结束并返回 "done" 字符串
            Poll::Ready("done")
        } else {
            // 目前先忽略下面这行代码
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    // 运行并等待 Future 的完成
    let out = future.await;

    // 判断 Future 返回的字符串是否是 "done"
    assert_eq!(out, "done");
}

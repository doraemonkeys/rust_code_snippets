use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// 实现 `Future` 定时器，之前提到: 新建线程在睡眠结束后会需要将状态同步给定时器 `Future` ，
// 由于是多线程环境，我们需要使用 `Arc<Mutex<T>>` 来作为一个共享状态，用于在新线程和 `Future` 定时器间共享。
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("TimerFuture poll");
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("TimerFuture poll completed, return Poll::Ready");
            Poll::Ready(())
        } else {
            // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            //
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            println!("TimerFuture poll not completed, return Poll::Pending");
            Poll::Pending
        }
    }
}

// 代码很简单，只要新线程设置了 `shared_state.completed = true` ，那任务就能顺利结束。
// 如果没有设置，会为当前的任务克隆一份 `Waker` ，这样新线程就可以使用它来唤醒当前的任务。

// 最后，再来创建一个 API 用于构建定时器和启动计时线程:
impl TimerFuture {
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        println!("TimerFuture new");
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

// 至此，一个简单的定时器 Future 就已创建成功，
// 那么该如何使用它呢？我们需要创建一个执行器，才能让程序动起来。

// Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动。
// 其中一个推动它的方式就是在 async 函数中使用 .await 来调用另一个 async 函数，
// 但是这个只能解决 async 内部的问题，那么这些最外层的 async 函数，谁来推动它们运行呢？
// 答案就是我们之前多次提到的执行器 executor (例如上一章节中的block_on函数) 。

// 执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们直到完成。
// 最开始，执行器会先 poll 一次 Future ，后面就不会主动去 poll 了，
// 而是等待 Future 通过调用 wake 函数来通知它可以继续，
// 它才会继续去 poll 。这种 wake 通知然后 poll 的方式会不断重复，直到 Future 完成。

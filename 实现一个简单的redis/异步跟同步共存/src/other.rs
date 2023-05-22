use tokio::runtime::Builder;
use tokio::sync::mpsc;

pub struct Task {
    pub name: String,
    // 一些信息用于描述该任务
}

async fn handle_task(task: Task) {
    println!("Got task {}", task.name);
}

#[derive(Clone)]
pub struct TaskSpawner {
    spawn: mpsc::Sender<Task>,
}

impl TaskSpawner {
    pub fn new() -> TaskSpawner {
        // 创建一个消息通道用于通信
        let (send, mut recv) = mpsc::channel(16);

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    tokio::spawn(handle_task(task));
                }

                // 一旦所有的发送端超出作用域被 drop 后，`.recv()` 方法会返回 None，同时 while 循环会退出，然后线程结束
            });
        });

        TaskSpawner { spawn: send }
    }

    pub fn spawn_task(&self, task: Task) {
        match self.spawn.blocking_send(task) {
            Ok(()) => println!("spawn_task Ok"),
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}

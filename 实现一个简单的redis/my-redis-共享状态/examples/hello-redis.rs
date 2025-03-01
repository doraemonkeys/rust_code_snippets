use mini_redis::{Result, client};

// 在代码中，使用了一个与众不同的 main 函数 : async fn main ，
// 而且是用 #[tokio::main] 属性进行了标记。异步 main 函数有以下意义：

// .await 只能在 async 函数中使用，如果是以前的 fn main，
// 那它内部是无法直接使用 async 函数的！这个会极大的限制了我们的使用场景
// 异步运行时本身需要初始化

// `#[tokio::main]` 宏在将 `async fn main` 隐式的转换为 `fn main` 的同时
// 还对整个异步运行时进行了初始化。例如以下代码:
/*

#[tokio::main]
async fn main() {
    println!("hello");
}

将被转换成:
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}

*/

#[tokio::main]
async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    // >$ mini-redis-server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"
    client.set("hello", "world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    async fn say_to_world() -> String {
        String::from("world")
    }

    // 此处的函数调用是惰性的，并不会执行 say_to_world() 函数体中的代码
    let op = say_to_world();

    // 首先打印出 "hello"
    println!("hello");

    // 使用 .await 让 say_to_world 开始运行起来
    // async fn 到底返回什么？它实际上返回的是一个实现了 Future 特征的匿名类型:
    // impl Future<Output = String>。
    println!("{}", op.await);

    Ok(())
}

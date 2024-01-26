include!(concat!(env!("OUT_DIR"), "/hello.rs"));
// env! 用于在编译时展开为指定的环境变量的值。
// concat! 用于将字符串字面量连接起来成为一个新的&str。
// include!将文件内容插入到这里。

fn main() {
    println!("{}", message());
    env!("OUT_DIR");
}

// 这里才是体现真正技术的地方，我们联合使用 rustc 定义的 include! 以及 concat! 和 env! 宏，
// 将生成的代码文件( hello.rs ) 纳入到我们项目的编译流程中。

// 例子虽然很简单，但是它清晰地告诉了我们该如何生成代码文件以及将这些代码文件纳入到编译中来，
// 大家以后有需要只要回头看看即可。

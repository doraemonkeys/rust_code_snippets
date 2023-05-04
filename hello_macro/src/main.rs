use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
    // 简单明了的代码总是令人愉快，为了让代码运行起来，还需要定义下过程宏。
    // 目前只能在单独的包中定义过程宏，尽管未来这种限制会被取消，但是现在我们还得遵循这个规则。
    // 由于过程宏所在的包跟我们的项目紧密相连，因此将它放在项目之中。

    // 问题又来了，该如何在项目的 `src/main.rs` 中引用 `hello_macro_derive` 包的内容？
    // 方法有两种，第一种是将 `hello_macro_derive` 发布到 `crates.io` 或 `GitHub` 中，
    // 就像我们引用的其它依赖一样；
    // 另一种就是使用相对路径引入的本地化方式，修改 `hello_macro/Cargo.toml` 文件添加以下内容:

    // 学习过程更好的办法是通过展开宏来阅读和调试自己写的宏，
    // 这里需要用到一个 cargo-expand 的工具，可以通过下面的命令安装:
    // cargo install cargo-expand

    // 观察展开后的代码
    // rustup override set nightly
    // cargo expand --bin hello_macro
}

// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
// #[macro_use]
// extern crate std;
// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;
// struct Sunfei;
// impl HelloMacro for Sunfei {
//     fn hello_macro() {
//         {
//             ::std::io::_print(format_args!("Hello, Macro! My name is {0}!\n", "Sunfei"));
//         };
//     }
// }
// struct Sunface;
// impl HelloMacro for Sunface {
//     fn hello_macro() {
//         {
//             ::std::io::_print(
//                 format_args!("Hello, Macro! My name is {0}!\n", "Sunface"),
//             );
//         };
//     }
// }
// fn main() {
//     Sunfei::hello_macro();
//     Sunface::hello_macro();
// }

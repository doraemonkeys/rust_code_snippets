extern crate cc;

// build.rs

/*
在项目中使用基于 C 或 C++ 的本地库，而这种使用场景恰恰是构建脚本非常擅长的。

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rerun-if-changed=src/hello.c");
}

*/

// 首先，构建脚本将我们的 C 文件通过 gcc 编译成目标文件，
// 然后使用 ar 将该文件转换成一个静态库，最后告诉 Cargo 我们的输出内容在 out_dir 中，
// 编译器要在这里搜索相应的静态库，最终通过 -l static-hello 标志将我们的项目跟 libhello.a 进行静态链接。

// 但是这种硬编码的解决方式有几个问题:
// gcc 命令的跨平台性是受限的，例如 Windows 下就难以使用它，
// 甚至于有些 Unix 系统也没有 gcc 命令，同样，ar 也有这个问题
// 这些命令往往不会考虑交叉编译的问题，如果我们要为 Android 平台进行交叉编译，
// 那么 gcc 很可能无法输出一个 ARM 的可执行文件
// 但是别怕，构建依赖 [build-dependencies] 解君忧：社区中已经有现成的解决方案，
// 可以让这种任务得到更容易的解决。例如文章开头提到的 cc 包。

// 简单来说，cc 包将构建脚本使用 C 的需求进行了抽象:
// cc 会针对不同的平台调用合适的编译器：windows 下调用 MSVC, MinGW 下调用 gcc， Unix 平台调用 cc 等
// 在编译时会考虑到平台因素，例如将合适的标志传给正在使用的编译器
// 其它环境变量，例如 OPT_LEVEL、DEBUG 等会自动帮我们处理
// 标准输出和 OUT_DIR 的位置也会被 cc 所处理

fn main() {
    // 编译成静态库
    cc::Build::new().file("src/double.c").compile("libdouble.a");
    cc::Build::new().file("src/third.c").compile("libthird.a");
}

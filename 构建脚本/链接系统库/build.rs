fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

// 代码很清晰，也很简洁，这里就不再过多介绍，
// 运行 cargo build --vv 来看看部分结果( 系统中需要已经安装 libz 库)：

// [libz-sys 0.1.0] cargo:rustc-link-search=native=/usr/lib
// [libz-sys 0.1.0] cargo:rustc-link-lib=z
// [libz-sys 0.1.0] cargo:rerun-if-changed=build.rs
// 非常棒，pkg-config 帮助我们找到了目标库，并且还告知了 Cargo 所有需要的信息！

// 实际使用中，我们需要做的比上面的代码更多，
// 例如 libz-sys 包会先检查环境变量 LIBZ_SYS_STATIC 或者 static feature，
// 然后基于源码去构建 libz，而不是直接去使用系统库。

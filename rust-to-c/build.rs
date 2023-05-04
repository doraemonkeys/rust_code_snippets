extern crate cc;

fn main() {
    // 编译成静态库
    cc::Build::new().file("src/double.c").compile("libdouble.a");
    cc::Build::new().file("src/third.c").compile("libthird.a");
}

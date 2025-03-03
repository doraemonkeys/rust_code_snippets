// rustc默认编译产生rust自用的rlib格式库，要让rustc产生动态链接库或者静态链接库，需要显式指定。

// 方法1: 在文件中指定。
// 在文件头加上#![crate_type = “foo”],
// 其中foo的可选类型有bin, lib, rlib, dylib, staticlib，
// 默认(将由rustc自己决定), rlib格式，动态链接库，静态链接库。
// 方法2: 编译时给rustc 传–crate-type参数，参数内容同上。
// 方法3: 使用cargo，指定crate-type = [“foo”], foo可选类型同1。

// #[no_mangle] 的作用是由于rust支持重载，所以函数名会被编译器进行混淆，
// 就像c++一样，加上这个就可以不修改函数名。

#![crate_type = "staticlib"] // 指定rustc编译成什么库类型，这里指定为静态库类型。

#[unsafe(no_mangle)]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}

#[unsafe(no_mangle)]
pub extern "C" fn third_input(input: i32) -> i32 {
    input * 3
}

// cargo build --release

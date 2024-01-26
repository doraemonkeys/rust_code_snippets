// 构建脚本可以通过发出 rustc-cfg 指令来开启编译时的条件检查。
// 在本例中，一起来看看 openssl 包是如何支持多版本的 OpenSSL 库的。

// openssl-sys 包对 OpenSSL 库进行了构建和链接，支持多个不同的实现(例如 LibreSSL )和多个不同的版本。
// 它也使用了 links 配置，这样就可以给其它构建脚本传递所需的信息。
// 例如 version_number ，包含了检测到的 OpenSSL 库的版本号信息。
// openssl-sys 自己的构建脚本中有类似于如下的代码:
// println!("cargo:version_number={:x}", openssl_version);
// 该指令将 version_number 的信息通过环境变量 DEP_OPENSSL_VERSION_NUMBER 的方式
// 传递给直接使用 openssl-sys 的项目。
// 例如 openssl 包提供了更高级的抽象接口，并且它使用了 openssl-sys 作为依赖。
// openssl 的构建脚本会通过环境变量读取 openssl-sys 提供的版本号的信息，然后使用该版本号来生成一些 cfg:

/*
if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
    let version = u64::from_str_radix(&version, 16).unwrap();

    if version >= 0x1_00_01_00_0 {
        println!("cargo:rustc-cfg=ossl101");
    }
    if version >= 0x1_00_02_00_0 {
        println!("cargo:rustc-cfg=ossl102");
    }
    if version >= 0x1_01_00_00_0 {
        println!("cargo:rustc-cfg=ossl110");
    }
    if version >= 0x1_01_00_07_0 {
        println!("cargo:rustc-cfg=ossl110g");
    }
    if version >= 0x1_01_01_00_0 {
        println!("cargo:rustc-cfg=ossl111");
    }
}

*/

// 这些 cfg 可以跟 cfg 属性 或 cfg 宏一起使用以实现条件编译。
// 例如，在 OpenSSL 1.1 中引入了 SHA3 的支持，
// 那么我们就可以指定只有当版本号为 1.1 时，才包含并编译相关的代码:

/*
#[cfg(ossl111)]
pub fn sha3_224() -> MessageDigest {
    unsafe { MessageDigest(ffi::EVP_sha3_224()) }
}
*/

// 当然，大家在使用时一定要小心，因为这可能会导致生成的二进制文件进一步依赖当前的构建环境。
// 例如，当二进制可执行文件需要在另一个操作系统中分发运行时，那它依赖的信息对于该操作系统可能是不存在的！

fn main() {}

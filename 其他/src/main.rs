mod error_handle;
mod format;
fn main() {
    // 错误处理
    study_error_handling();

    // 错误处理2
    error_handle::study_error_handling2();

    // 格式化输出
    format::study_format_output();
}

fn study_error_handling() {
    // panic! 宏
    println!("----------------- panic! 宏 -----------------");
    // panic! 宏，当调用执行该宏时，程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序。
    //panic!("crash and burn");

    // backtrace 栈展开
    println!("----------------- backtrace 栈展开 -----------------");
    //     在使用时加上一个环境变量可以获取更详细的栈展开信息：
    // - Linux/macOS 等 UNIX 系统： `RUST_BACKTRACE=1 cargo run`
    // - Windows 系统（PowerShell）： `$env:RUST_BACKTRACE=1 ; cargo run`
    // 当出现 panic! 时，程序提供了两种方式来处理终止流程：栈展开 和 直接终止。
    // 其中，默认的方式就是 `栈展开`，这意味着 Rust 会回溯栈上数据和函数调用，因此也意味着更多的善后工作，
    // 好处是可以给出充分的报错信息和栈调用信息，便于事后的问题复盘。
    // `直接终止`，顾名思义，不清理数据就直接退出程序，善后工作交与操作系统来负责。

    // Result 枚举
    study_result_enum();

    // 传播错误
    study_propagate_error();

    // 引入外部库
    study_use_external_library();
}

fn study_use_external_library() {
    println!("----------------- 引入本地库 -----------------");
    // Cargo.toml 中添加依赖
    sayhello_lib::say_hello("哆啦A梦");
    println!("3 + 4 = {}", sayhello_lib::add(3, 4));
}

fn study_propagate_error() {
    println!("----------------- 传播错误 -----------------");
    // 实际应用中，大概率会把错误层层上传然后交给调用链的上游函数进行处理，错误传播将极为常见。
    let file = read_username_from_file("unknown.txt");
    if let Err(e) = file {
        println!("read file error: {:?}", e);
    }
    // 更简洁的写法
    println!("----------------- 更简洁的写法 -----------------");
    let file = read_username_from_file2("hello.txt");
    if let Err(e) = file {
        println!("read file error: {:?}", e);
        return;
    }
    println!("file's content: {}", file.unwrap());
    // 更更简洁的写法
    println!("----------------- 更更简洁的写法 -----------------");
    let file = read_username_from_file3("hello.txt");
    if let Err(e) = file {
        println!("read file error: {:?}", e);
        return;
    }
    println!("file's content: {}", file.unwrap());
    // 更更更简洁的写法
    println!("----------------- 更更更简洁的写法 -----------------");
    let file = read_username_from_file4();
    if let Err(e) = file {
        println!("read file error: {:?}", e);
        return;
    }
    println!("file's content: {}", file.unwrap());

    // ? 用于 Option 的返回
    println!("----------------- ? 用于 Option 的返回 -----------------");
    let arr = [1, 2, 3];
    let first = first(&arr);
    println!("first: {:?}", first);
}

fn first(arr: &[i32]) -> Option<&i32> {
    // 如果 `get` 的结果是 `None`，则直接返回 `None`，如果是 `Some(&i32)`，则把里面的值赋给 `v`。
    // `?` 操作符需要一个变量来承载正确的值，因此这里需要使用 `let` 语句来声明一个变量。
    let v = arr.get(0)?;
    Some(v)
}

fn read_username_from_file4() -> Result<String, std::io::Error> {
    // 从文件读取数据到字符串中，是比较常见的操作，因此 Rust 标准库为我们提供了 `fs::read_to_string` 函数，
    // 该函数内部会打开一个文件、创建 `String`、读取文件内容最后写入字符串并返回。
    std::fs::read_to_string("hello.txt")
}

/// 更更简洁的写法
fn read_username_from_file3(filename: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    use std::io::Read;
    // 瞧见没？ `?` 还能实现链式调用，`File::open` 遇到错误就返回，
    // 没有错误就将 `Ok` 中的值取出来用于下一个方法调用，简直太精妙了
    std::fs::File::open(filename)?.read_to_string(&mut s)?;

    Ok(s)
}

/// 更简洁的写法
fn read_username_from_file2(filename: &str) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(filename)?;
    let mut s = String::new();
    use std::io::Read;
    // 其实 ? 就是一个宏，它的作用跟上一个函数里面的 match 几乎一模一样：
    // 如果结果是 Err(E)，返回该错误，否则返回 Ok(T) 中的 T，这里赋值给了result。
    // `?` 操作符需要一个变量来承载正确的值，因此这里需要使用 `let` 语句来声明一个变量。
    let n = f.read_to_string(&mut s)?;
    println!("read {} bytes", n);
    Ok(s)
    // 虽然 `?` 和 `match` 功能一致，但是事实上 `?` 会更胜一筹。何解？
    // 一个设计良好的系统中，肯定有自定义的错误特征，错误之间很可能会存在上下级关系，
    // 例如标准库中的 `std::io::Error `和 `std::error::Error`，
    // 前者是 IO 相关的错误结构体，后者是一个最最通用的标准错误特征，同时前者实现了后者，
    // 因此 `std::io::Error` 可以转换为 `std:error::Error`。
    // ?` 的更胜一筹就很好理解了，它可以自动进行类型提升（转换）：
    // 例如 `File::open` 报错时返回的错误是 `std::io::Error` 类型，
    // 但是 `open_file` 函数返回的错误类型是 `std::error::Error` 的特征对象，
    // 可以看到一个错误类型通过 `?` 返回后，变成了另一个错误类型
}

/// 下面函数的代码有点Go风格，Rust中有更简洁的写法
fn read_username_from_file(filename: &str) -> Result<String, std::io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = std::fs::File::open(filename);

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    use std::io::Read;
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}

fn study_result_enum() {
    println!("----------------- Result 枚举 -----------------");
    // 当没有错误发生时，函数返回一个用 `Result` 类型包裹的值 `Ok(T)`，当错误时，返回一个 `Err(E)`。
    // 对于 `Result` 返回我们有很多处理方法，最简单粗暴的就是 `unwrap` 和 `expect`，这两个函数非常类似，
    // 如果有错误，这两个函数都会 会直接 `panic!`，而 `expect` 还会附带自定义的错误信息。
    // 如果需要快速地搭建代码，错误处理会拖慢编码的速度，也不是特别有必要，
    // 因此通过 `unwrap`、`expect` 等方法来处理是最快的。
    let mut f = std::fs::File::open("hello.txt").unwrap();
    let mut file: String = String::new();
    std::io::Read::read_to_string(&mut f, &mut file).expect("read file error");
    println!("file: {}", file);

    // unwrap_or, unwrap_or_else, unwrap_or_default
    // unwrap_or_else 接受一个闭包，当有错误时，会将错误传递给闭包，闭包的返回值会作为 `unwrap_or_else` 的返回值。
    let num = "42".parse::<i32>().unwrap_or_else(|err| {
        println!("parse error: {}", err);
        0
    });
    println!("num: {}", num);
    // unwrap_or 在有错误时，会返回一个默认值，这个默认值可以是任意类型，因为它的类型是 `Option<T>` 中的 `T`。
    let num = "42e".parse::<i32>().unwrap_or(10);
    println!("num: {}", num);
    // unwrap_or_default 会返回 `T` 的默认值，这个 `T` 必须实现了 `Default` 特征。
    let num = "42e".parse::<i32>().unwrap_or_default();
    println!("num: {}", num);

    // map, map_err ,map_or, map_or_else
    // map 接受一个闭包，当 `Result` 为 `Ok(T)` 时，会将 `T` 传递给闭包，闭包的返回值会作为 `map` 的返回值，
    // 也就是 `Result<U, E>`，这个 `U` 可以是任意类型，因为它是 `map` 的返回值。
    let str = "1,2,3,4,5";
    let nums: Vec<_> = str.split(',').map(|s| s.parse::<i32>()).collect();
    println!("num: {:?}", nums);
    // map_err 接受一个闭包，当 `Result` 为 `Err(E)` 时，会将 `E` 传递给闭包，闭包的返回值会作为 `map_err` 的返回值，
    // 也就是 `Result<T, F>`，这个 `F` 可以是任意类型，因为它是 `map_err` 的返回值。
    let num = "42e".parse::<i32>().map_err(|err| {
        println!("parse error: {}", err);
        0
    });
    println!("num: {:?}", num);
    // map_or 接受一个默认值，当 `Result` 为 `Ok(T)` 时，会将 `T` 作为 `map_or` 的返回值，
    // 当 `Result` 为 `Err(E)` 时，会将默认值作为 `map_or` 的返回值。
    let num = "42".parse::<i32>().map_or(0, |n| n + 1);
    println!("num: {}", num);
    // map_or_else 接受一个闭包，当 `Result` 为 `Ok(T)` 时，会将 `T` 传递给闭包，闭包的返回值会作为 `map_or_else` 的返回值，
    // 当 `Result` 为 `Err(E)` 时，会将默认值作为 `map_or_else` 的返回值。
    let num = "42e".parse::<i32>().map_or_else(
        |err| {
            println!("parse error: {}", err);
            10
        },
        |n| n + 1,
    );
    println!("num: {}", num);

    println!("-----------------  -----------------");

    let filename = "hello2.txt";
    let f = std::fs::File::open(filename);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create(filename) {
                Ok(fc) => {
                    println!("create file: {}", filename);
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("file: {:?}", f);
    // close file
    std::mem::drop(f);
}

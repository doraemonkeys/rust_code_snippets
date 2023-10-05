use anyhow::Context;

pub fn study_error_handling2() {
    println!("-----------------错误处理2-----------------");

    // 组合器
    study_combinator();

    // 自定义错误类型
    study_custom_error_type();
}

fn study_custom_error_type() {
    println!("-----------------自定义错误类型-----------------");
    // 为了帮助我们更好的定义错误，Rust 在标准库中提供了一些可复用的特征，例如 `std::error::Error` 特征：
    // pub trait Error: Debug + Display {
    //     fn source(&self) -> Option<&(Error + 'static)> { ... }
    // }
    // 当自定义类型实现该特征后，该类型就可以作为 `Err` 来使用了。
    // 实际上，自定义错误类型只需要实现 `Debug` 和 `Display` 特征即可，`source` 方法是可选的，
    // 而 `Debug` 特征往往也无需手动实现，可以直接通过 `derive` 来派生

    // AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
    // 为 AppError 自动派生 Debug 特征
    #[derive(Debug)]
    struct AppError;

    // 为 AppError 实现 std::fmt::Display 特征
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "An Error Occurred, Please Try Again!") // user-facing output
        }
    }
    // 一个示例函数用于产生 AppError 错误
    fn produce_error() -> Result<(), AppError> {
        Err(AppError)
    }
    match produce_error() {
        Err(e) => eprintln!("{}", e), // An Error Occurred, Please Try Again!
        _ => println!("No error"),
    }
    eprintln!("{:?}", produce_error()); // Err(AppError)

    // 更详尽的错误
    example_more_detail_error();

    // 错误转换 `From` 特征
    study_error_conversion();

    // 归一化不同的错误类型
    study_normalization();

    // 自定义错误类型
    study_custom_error_type2();

    // 简化错误处理
    study_simplify_error_handling();
}

fn study_simplify_error_handling() {
    println!("-----------------简化错误处理-----------------");
    // thiserror
    example_thiserror();

    // anyhow
    example_anyhow();
}

fn example_anyhow() {
    println!("-----------------anyhow-----------------");
    //anyhow 和 thiserror 是同一个作者开发的，这里是作者关于 anyhow 和 thiserror 的原话：
    // 如果你想要设计自己的错误类型，同时给调用者提供具体的信息时，就使用 thiserror，
    // 例如当你在开发一个三方库代码时。如果你只想要简单，就使用 anyhow，例如在自己的应用服务中。
    use std::fs::read_to_string;

    fn main() -> anyhow::Result<()> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> anyhow::Result<String> {
        let key = "MARKDOWN";
        // 为错误添加上下文(Debug 输出)
        let file = std::env::var(key).with_context(|| format!("Failed to get env var {}", key))?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    if let Err(e) = main() {
        eprintln!("{}", e); // Failed to get env var MARKDOWN
        eprintln!("{:?}", e);
        // Failed to get env var MARKDOWN
        // Caused by:
        //     environment variable not found
    }
    // 关于如何选用 thiserror 和 anyhow 只需要遵循一个原则即可：是否关注自定义错误消息，
    // anyhow 可以更方便的向上传递错误，但传递的过程中会丢失原本错误的类型，只会留下错误信息。
}
fn example_thiserror() {
    println!("-----------------thiserror-----------------");
    use std::fs::read_to_string;

    fn main() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }
    // 只要为struct或者每个成员提供​​#[error("...")]​​​，那么就会实现​​Display​​，具体语法如下
    // ​​#[error("{var}")]​​ ⟶ ​​write!("{}", self.var)​​
    // ​​#[error("{0}")]​​ ⟶ ​​write!("{}", self.0)​​
    // ​​#[error("{var:?}")]​​ ⟶ ​​write!("{:?}", self.var)​​
    // ​​#[error("{0:?}")]​​ ⟶ ​​write!("{:?}", self.0)​​
    #[derive(thiserror::Error, Debug)]
    enum MyError {
        #[error("Environment variable not found")] // 为错误类型添加描述 (Display trait)
        EnvironmentVariableNotFound(#[from] std::env::VarError), // from 用于转换错误类型
        #[error(transparent)] // 无描述？
        IOError(#[from] std::io::Error),
    }
    // 如上所示，只要简单写写注释，就可以实现错误处理了，惊不惊喜？
    if let Err(e) = main() {
        eprintln!("{}", e); // Environment variable not found
        eprintln!("{:?}", e); // EnvironmentVariableNotFound
    }
}

fn study_custom_error_type2() {
    println!("-----------------自定义错误类型2-----------------");
    // 与特征对象相比，自定义错误类型麻烦归麻烦，但是它非常灵活，因此也不具有上面的类似限制:
    use std::fs::read_to_string;

    #[derive(Debug)]
    enum MyError {
        EnvironmentVariableNotFound,
        IOError(std::io::Error),
    }

    impl From<std::env::VarError> for MyError {
        fn from(_: std::env::VarError) -> Self {
            Self::EnvironmentVariableNotFound
        }
    }

    impl From<std::io::Error> for MyError {
        fn from(value: std::io::Error) -> Self {
            Self::IOError(value)
        }
    }

    impl std::error::Error for MyError {}

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
                MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
            }
        }
    }
    fn main() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }
    if let Err(e) = main() {
        eprintln!("{}", e); // Environment variable not found
        eprintln!("{:?}", e); // EnvironmentVariableNotFound
    }
    // 灵活归灵活，啰嗦也是真啰嗦，好在 Rust 的社区为我们提供了 `thiserror` 解决方案，
    // 下面一起来看看该如何简化 Rust 中的错误处理。
}

fn study_normalization() {
    println!("-----------------归一化不同的错误类型-----------------");
    // 在实际项目中，我们往往会为不同的错误定义不同的类型，这样做非常好，
    // 但是如果你要在一个函数中返回不同的错误呢？
    use std::fs::read_to_string;

    fn render() -> Result<String, Box<dyn std::error::Error>> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }
    let html = render();
    println!("{:?}", html); // Err(NotPresent)

    // 这个方法很简单，在绝大多数场景中，性能也非常够用，但是有一个问题：
    // `Result` 实际上不会限制错误的类型，也就是一个类型就算不实现 `Error` 特征，
    // 它依然可以在 `Result<T, E>` 中作为 `E` 来使用，此时这种特征对象的解决方案就无能为力了。
}

fn study_error_conversion() {
    println!("-----------------错误转换 From 特征-----------------");
    // 标准库、三方库、本地库，各有各的精彩，各也有各的错误。那么问题就来了，
    // 我们该如何将其它的错误类型转换成自定义的错误类型？总不能神鬼牛魔，同台共舞吧。。
    // 好在 Rust 为我们提供了 `std::convert::From` 特征:
    // 大家都使用过 `String::from` 函数吧？它可以通过 `&str` 来创建一个 `String`，
    // 其实该函数就是 `From` 特征提供的。
    use std::fs::File;
    use std::io;

    #[derive(Debug)]
    struct AppError {
        _kind: String,    // 错误类型
        _message: String, // 错误信息
    }

    // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
    // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError {
                _kind: String::from("io"),
                _message: error.to_string(),
            }
        }
    }

    fn main() -> Result<(), AppError> {
        // 通过 ?宏 自动进行类型转换，将 io::Error 转换成 AppError,
        // 本质上是调用了 AppError::from(error)
        let _file = File::open("nonexistent_file.txt")?;

        Ok(())
    }
    if let Err(e) = main() {
        eprintln!("{:?}", e); // AppError { kind: "io", message: "No such file or directory (os error 2)" }
    }
}

fn example_more_detail_error() {
    println!("-----------------更详尽的错误-----------------");

    struct AppError {
        code: usize,
        message: String,
    }

    // 根据错误码显示不同的错误信息
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let err_msg = match self.code {
                404 => "Sorry, Can not find the Page!",
                _ => "Sorry, something is wrong! Please Try Again!",
            };

            write!(f, "{}", err_msg)
        }
    }

    impl std::fmt::Debug for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "AppError {{ code: {}, message: {} }}",
                self.code, self.message
            )
        }
    }

    fn produce_error() -> Result<(), AppError> {
        Err(AppError {
            code: 404,
            message: String::from("Page not found"),
        })
    }
    match produce_error() {
        Err(e) => eprintln!("{}", e), // Sorry, Can not find the Page!
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

    eprintln!("{:#?}", produce_error());
    // Err(
    //     AppError { code: 404, message: Page not found }
    // )
}
fn study_combinator() {
    println!("-----------------组合器-----------------");
    // 将对象组合成树形结构以表示“部分整体”的层次结构。
    // 组合模式使得用户对单个对象和组合对象的使用具有一致性。–GoF <<设计模式>>
    // 与组合器模式有所不同，在 Rust 中，组合器更多的是用于对返回结果的类型进行变换：
    // 例如使用 `ok_or` 将一个 `Option` 类型转换成 `Result` 类型。
    let s = Some(10);
    let n: Option<u32> = None;
    assert_eq!(s.ok_or("error"), Ok(10));
    assert_eq!(n.ok_or("error"), Err("error"));

    // or() 和 and()
    study_or_and();

    // or_else() 和 and_then()
    study_or_else_and_then();

    // filter
    study_filter();

    // map() 和 map_err()
    study_map_map_err();

    // map_or() 和 map_or_else()
    study_map_or_map_or_else();
}

fn study_map_or_map_or_else() {
    println!("----------------- map_or() 和 map_or_else() -----------------");
    // `map_or` 在 `map` 的基础上提供了一个默认值:
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);

    // map_or_else 与 map_or 类似，但是它是通过一个闭包来提供默认值:
    let s = Some(10);
    let n: Option<i8> = None;

    let fn_closure = |v: i8| v + 2;
    let fn_default = || 1;

    assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

    let o = Ok(10);
    let e = Err(5);
    let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值

    assert_eq!(o.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);

    // `ok_or_else` 接收一个闭包作为 `Err` 参数,
    // Transforms the Option<T> into a Result<T, E>,
    // mapping [Some(v)] to [Ok(v)] and None to [Err(err())].
    let x = Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));

    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)
}

fn study_map_map_err() {
    println!("----------------- map() 和 map_err() -----------------");
    // `map` 可以将 `Some<T1>` 或 `Ok` 中的值映射为另一个：Some<T1> map = Some<T2>
    // 而 `map_err` 则可以将 `Err` 中的值映射为另一个：
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2

    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2

    //但是如果你想要将 `Err` 中的值进行改变， `map` 就无能为力了，此时我们需要用 `map_err`：
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
}

fn study_filter() {
    println!("----------------- filter -----------------");
    // `filter` 用于对 `Option` 进行过滤：
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None
}

fn study_or_else_and_then() {
    println!("----------------- or_else() 和 and_then() -----------------");
    // 它们跟 `or()` 和 `and()` 类似，唯一的区别在于，它们的第二个表达式是一个闭包。
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2

    // and_then with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    // 如果o1 是 `Ok`，则 `and_then` 会调用 `fn_ok`，并返回 `fn_ok` 的返回值。
    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}

fn study_or_and() {
    println!("----------------- or() 和 and() -----------------");
    // 跟布尔关系的与/或很像，这两个方法会对两个表达式做逻辑组合，最终返回 `Option` / `Result`。
    // - `or()`，表达式按照顺序求值，若任何一个表达式的结果是 `Some` 或 `Ok`，则该值会立刻返回
    // - `and()`，若两个表达式的结果都是 `Some` 或 `Ok`，则第二个表达式中的值被返回。
    //    若任何一个的结果是 `None` 或 `Err` ，则立刻返回。
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(s1.or(n), s1); // Some or None = Some
    assert_eq!(n.or(s1), s1); // None or Some = Some
    assert_eq!(n.or(n), n); // None1 or None2 = None2

    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
    assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
    assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

    assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
    assert_eq!(s1.and(n), n); // Some and None = None
    assert_eq!(n.and(s1), n); // None and Some = None
    assert_eq!(n.and(n), n); // None1 and None2 = None1

    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1

    // 除了 `or` 和 `and` 之外，Rust 还为我们提供了 `xor` ，
    // 但是它只能应用在 `Option` 上，其实想想也是这个理，如果能应用在 `Result` 上，
    // 那你又该如何对一个值和错误进行异或操作？
    // `xor()`，若两个表达式的结果一个是 `Some`，一个是 `None`，则返回 `Some` 中的值。
    //  若两个表达式的结果都是 `Some` 或 `None`，则返回 `None`。
    assert_eq!(s1.xor(s2), n); // Some1 xor Some2 = None
    assert_eq!(s1.xor(n), s1); // Some xor None = Some
    assert_eq!(n.xor(s1), s1); // None xor Some = Some
    assert_eq!(n.xor(n), n); // None1 xor None2 = None
}

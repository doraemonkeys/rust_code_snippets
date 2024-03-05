fn main() {
    // 在其它语言中，我们用 `var a = "hello world"` 的方式给 `a` 赋值，
    // 也就是把等式右边的 `"hello world"` 字符串赋值给变量 `a` ，
    // 而在 Rust 中，我们这样写： `let a = "hello world"` ，同时给这个过程起了另一个名字：变量绑定。

    // 为何不用赋值而用绑定呢（其实你也可以称之为赋值，但是绑定的含义更清晰准确）？
    // 这里就涉及 Rust 最核心的原则——所有权。

    // 变量
    study_variable_binding();
    // Rust 的数值上可以使用方法
    // 基本数据类型
    study_basic_data_types();
    // 有理数和复数
    study_rational_and_complex();
    // 字符、布尔、单元类型
    study_char_bool_unit();
    // 类型转换
    study_type_conversion();
    // 全局变量
    study_global_variable();
}

fn study_global_variable() {
    println!("-----------------全局变量-----------------");
    // 全局变量的生命周期肯定是`'static`，但是不代表它需要用`static`来声明，
    // 例如常量、字符串字面值等无需使用`static`进行声明，原因是它们已经被打包到二进制可执行文件中。

    // 静态常量
    println!("-----------------静态常量-----------------");
    // 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，
    // 所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址。
    const MAX_ID: usize = usize::MAX / 2; // 适合用作静态配置
    println!("用户ID允许的最大值是{}", MAX_ID);

    // 静态变量
    println!("-----------------静态变量-----------------");
    static mut REQUEST_RECV: usize = 0;
    // 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址；
    // 存储在静态变量中的值必须要实现 Sync trait

    // const 和 static 的区别
    fn foo() -> (i32, i32) {
        static X1: AtomicUsize = AtomicUsize::new(1);
        const X2: AtomicUsize = AtomicUsize::new(99);
        (
            X1.fetch_add(1, Ordering::SeqCst) as i32,
            X2.fetch_add(1, Ordering::SeqCst) as i32,
        )
    }
    println!("first call foo: {:?}", foo()); // (1, 99)
    println!("second call foo: {:?}", foo()); // (2, 99)

    // 原子类型
    println!("-----------------原子类型-----------------");
    // 想要全局计数器、状态控制等功能，又想要线程安全的实现，原子类型是非常好的办法。
    use std::sync::atomic::{AtomicUsize, Ordering};
    static REQUEST_RECV_ATOMIC: AtomicUsize = AtomicUsize::new(0);

    // lazy_static是社区提供的非常强大的宏，用于懒初始化静态变量，之前的静态变量都是在编译期初始化的，
    // 因此无法使用函数调用进行赋值，而 lazy_static 允许我们在运行期初始化静态变量！
    println!("-----------------lazy_static-----------------");
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    // 为什么需要运行初始化？以下的静态初始化有一个致命的问题：
    // 无法用函数进行静态初始化，例如你如果想声明一个全局的Mutex锁
    // static NAMES1: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));

    lazy_static! {
        // lazy_static 宏，匹配的是 static ref ，所以定义的静态变量都是不可变引用。
        // lazy_static 定义后直到使用该变量，才进行初始化，非常 lazy static 。
        static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
    }
    // OnceCell
    println!("-----------------OnceCell-----------------");
    #[allow(unused_imports)]
    use std::cell::OnceCell;
    // OnceCell 是用于单线程环境下的懒加载数据结构。
    // 它可以用来存储某个值，并在需要时进行初始化，但是只能在单线程环境下使用。

    // OnceLock
    println!("-----------------OnceLock-----------------");
    // OnceCell 和它的线程安全对应类型 OnceLock两个新的类型在1.70稳定下来，用于共享数据的一次性初始化。

    use std::sync::OnceLock;
    static DATA: OnceLock<&str> = OnceLock::new();

    fn init_example() {
        let winner = std::thread::scope(|s| {
            s.spawn(|| DATA.set("thread"));

            std::thread::yield_now(); // give them a chance...

            // Many threads may call get_or_init concurrently with different
            // initializing functions, but it is guaranteed that only one function will be executed.
            DATA.get_or_init(|| "main")
        });

        println!("{winner} wins!");
    }

    init_example();

    // Box::leak
    println!("-----------------Box::leak-----------------");
    // 我们提到了Box::leak可以用于全局变量，例如用作运行期初始化的全局动态配置，
    // 先来看看如果不使用lazy_static也不使用Box::leak，会发生什么：
    #[derive(Debug)]
    struct Config {
        _a: String,
        _b: String,
    }
    static mut CONFIG: Option<&mut Config> = None;
    fn test_no_lazy() {
        let c = Box::new(Config {
            _a: "A".to_string(),
            _b: "B".to_string(),
        });

        unsafe {
            // 报错，Rust 的借用和生命周期规则限制了我们做到这一点，
            // 因为试图将一个局部生命周期的变量赋值给全局生命周期的CONFIG，这明显是不安全的。
            // CONFIG = Some(&mut Config {
            //     a: "A".to_string(),
            //     b: "B".to_string(),
            // });

            // 好在`Rust`为我们提供了`Box::leak`方法，它可以将一个变量从内存中泄漏(听上去怪怪的，竟然做主动内存泄漏)，
            // 然后将其变为`'static`生命周期，最终该变量将和程序活得一样久，因此可以赋值给全局静态变量`CONFIG`。
            // 将`c`从内存中泄漏，变成`'static`生命周期
            CONFIG = Some(Box::leak(c));
            println!("{:?}", CONFIG);
        }
    }
    test_no_lazy();
    unsafe {
        REQUEST_RECV += 1;
        // Rust 要求必须使用unsafe语句块才能访问和修改static变量，因为这种使用方式往往并不安全，
        // 其实编译器是对的，当在多线程中同时去修改时，会不可避免的遇到脏数据
        assert_eq!(REQUEST_RECV, 1);
    }

    for _ in 0..100 {
        REQUEST_RECV_ATOMIC.fetch_add(1, Ordering::Relaxed);
    }
    println!("当前用户请求数{:?}", REQUEST_RECV_ATOMIC);

    //lazy_static直到运行到main中的第一行代码时，才进行初始化，非常lazy static
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Tom");
    println!("{}", v);

    // 从函数中返回全局变量
    println!("-----------------从函数中返回全局变量-----------------");
    fn init() -> Option<&'static mut Config> {
        let c = Box::new(Config {
            _a: "A".to_string(),
            _b: "B".to_string(),
        });

        Some(Box::leak(c)) // 将`c`从内存中泄漏，变成`'static`生命周期
    }
    unsafe {
        CONFIG = init();
        println!("{:?}", CONFIG)
    }

    // 标准库中的 OnceCell
    println!("-----------------OnceCell-----------------");
    // 在 Rust 标准库中提供 lazy::OnceCell 和 lazy::SyncOnceCell 两种 Cell，
    // 前者用于单线程，后者用于多线程，它们用来存储堆上的信息，并且具有最多只能赋值一次的特性。
    // 如实现一个多线程的日志组件 Logger：
    example_logger();
}

fn example_logger() {
    println!("-----------------example_logger-----------------");
    // #![feature(once_cell)]
    // 目前 `OnceCell` 和 `SyncOnceCell` API 暂未稳定，需启用特性 `#![feature(once_cell)]`。
}

fn study_type_conversion() {
    // Rust 不提供原生类型之间的隐式类型转换（coercion），但可以使用 as 关键字进行显 式类型转换（casting）。
    println!("-----------------类型转换-----------------");
    let decimal = 65.4321_f32;

    // 错误！不提供隐式转换
    //let integer: u8 = decimal;

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // int8 的范围是 -128..=127
    let num = 128;
    println!("{} as a i8 is : {}", num, num as i8); // -128

    // MaxInt8
    let num = -1i8;
    let max_int8 = ((num as u8) >> 1) as i8;
    println!("max_int8 = {}", max_int8);
    let max_int8_2 = std::i8::MAX;
    assert_eq!(max_int8, max_int8_2);

    // try_from 和 try_into
    // 从 Rust 1.34 开始，标准库提供了 `TryFrom` 和 `TryInto` 两个 trait，
    // 它们可以在转换失败时返回错误，而不是 panic。
    // TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
    println!("-----------------try_from 和 try_into-----------------");
    let num = 128;
    let num: Result<i8, _> = num.try_into();
    if let Ok(num) = num {
        // 如果num是Ok类型，就将num赋值给num
        println!("num = {}", num);
    } else {
        // 这里128超出了i8的范围，所以会打印出错误信息
        println!("num = {:?}", num);
    }
    // try_from
    let num = 128;
    let num: Result<i8, _> = i8::try_from(num);
    if let Ok(num) = num {
        // 如果num是Ok类型，就将num赋值给num
        println!("num = {}", num);
    } else {
        // 这里128超出了i8的范围，所以会打印出错误信息
        println!("num = {:?}", num);
    }
    // 解析字符串
    println!("-----------------解析字符串-----------------");
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};
    // 如果要转换到用户定义类型，只要手动实现 FromStr trait 就行。
    // 例如，我们可以将字符串转换为 Point 结构体：
    println!("-----------------解析字符串到用户定义类型-----------------");
    let point = "1,2".parse::<Point>();
    let point = match point {
        Ok(point) => point,
        Err(e) => {
            println!("Error: {:?}", e);
            Point { _x: 0, _y: 0 }
        }
    };
    println!("point = {:?}", point);

    // 内存地址转换为指针
    println!("-----------------内存地址转换为指针-----------------");
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    println!("values = {:?}", values); // [1, 3]
}

#[derive(Debug)]
struct Point {
    _x: i32,
    _y: i32,
}
#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl std::str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(',') {
            None => Err(ParsePointError),
            Some(index) => {
                let (x, y) = s.split_at(index);
                let x = x.parse::<i32>();
                if matches!(x, Err(_)) {
                    return Err(ParsePointError);
                }
                let y = y[1..].parse::<i32>();
                if matches!(y, Err(_)) {
                    return Err(ParsePointError);
                }
                Ok(Point {
                    _x: x.unwrap(),
                    _y: y.unwrap(),
                })
            }
        }
    }
}

fn study_char_bool_unit() {
    // Rust 中的字符类型是 `char`(本质上是一个 `u32` 类型)，它占用 4 个字节，即 32 位。
    // Rust 的字符不仅仅是 `ASCII`，所有的 `Unicode` 值都可以作为 Rust 字符，
    // 包括单个的中文、日文、韩文、emoji 表情符号等等
    println!("-----------------字符-----------------");
    let a = 'a';
    let b: char = 'b';
    let c = '中';
    let d = '🤣';
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    println!("字符'中'的字节长度：{}", c.len_utf8());
    println!("字符'🤣'的字节长度：{}", d.len_utf8());
    println!("字符'{}'占用了{}个字节内存", d, std::mem::size_of_val(&c));

    // Rust 中的布尔类型是 `bool`，它占用 1 个字节。
    // Rust 中的布尔类型只有两个值：`true` 和 `false`。
    println!("-----------------布尔-----------------");
    let a = true;
    let b: bool = false;
    println!("a = {}, b = {}", a, b);

    // Rust 中的单元类型是 ()，它占用 0 个字节，唯一的值也是 ()。
    // main 函数就返回这个单元类型 ()，你不能说 `main` 函数无返回值，
    // 没有返回值的函数在 Rust 中是有单独的定义的：`发散函数( diverge function )`，顾名思义，无法收敛的函数。
    // 你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。
    // 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，并且完全不占用任何内存。
    println!("-----------------单元-----------------");
    let a = ();
    let b: () = ();
    println!("a = {:?}, b = {:?}", a, b);
    println!("单元类型()占用了{}个字节内存", std::mem::size_of_val(&a));
}

fn study_rational_and_complex() {
    // Rust 的标准库相比其它语言，准入门槛较高，因此有理数和复数并未包含在标准库中
    // Rust 有一个叫做 `num` 的第三方库，它提供了有理数和复数的支持，可以通过 `cargo add num` 安装。
    println!("-----------------有理数和复数-----------------");
    // re 代表实部，im 代表虚部
    let a = num::complex::Complex { re: 2.1, im: -1.2 };
    let b = num::complex::Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

fn study_basic_data_types() {
    println!("-----------------整型-----------------");
    // Rust 中的整型有：u8、i8、u16、i16、u32、i32、u64、i64、u128、i128、usize、isize。
    // `isize` 和 `usize` 类型取决于程序运行的计算机 CPU 类型：
    // isize是指针大小的有符号整数类型，而usize是指针大小的无符号整数类型。
    // 若 CPU 是 32 位的，则这两个类型是 32 位的，同理，若 CPU 是 64 位，那么它们则是 64 位。
    // Rust 整型默认使用 `i32`，例如 `let i = 1`，那 `i` 就是 `i32` 类型，
    // 因此你可以首选它，同时该类型也往往是性能最好的。
    let a: i32 = 98_222;
    let b: i64 = 0xff;
    let c: isize = -0o77;
    let d: u32 = 0b1111_0000;
    // 字节赋值，仅限于u8
    let e: u8 = b'A';
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
    // 整型溢出
    println!("-----------------整型溢出-----------------");
    // 当在 debug 模式编译时，Rust 会在整型溢出时 panic。
    // 当在 release 模式编译时，Rust不检测溢出。相反，当检测到整型溢出时，
    // Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理。
    // 如果要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
    // 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    // 使用 checked_* 方法时发生溢出，则返回 None 值
    // 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // 使用 saturating_* 方法使值达到最小值或最大值
    let aa: u8 = 255;
    let bb: u8 = aa.wrapping_add(1);
    println!("aa = {}, bb = {}", aa, bb); //aa = 255, bb = 0
                                          // 浮点陷阱
    println!("-----------------浮点陷阱-----------------");
    study_float_traps();
    //NaN
    println!("-----------------NaN-----------------");
    // 对于数学上未定义的结果，例如对负数取平方根 `-42.1.sqrt()` ，会产生一个特殊的结果：
    // Rust 的浮点数类型使用 `NaN` (not a number)来处理这些情况。
    // 所有跟 `NaN` 交互的操作，都会返回一个 `NaN`，而且 `NaN` 不能用来比较。
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    // 无穷大
    let inf = 1.0_f32 / 0.0_f32; // inf = Infinity
    println!("inf = {}", inf);

    //类型推断
    println!("-----------------类型推断-----------------");
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
    println!("twenty_two = {}", twenty_two);
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    // 序列
    println!("-----------------序列-----------------");
    // Rust 提供了一个非常简洁的方式，用来生成连续的数值，例如 `1..5`，生成从 1 到 4 的连续数字，不包含 5。
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();
    // `1..=5`，生成从 1 到 5 的连续数字，包含 5。
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // 将 `13.14` 取整：`13.14_f32.round()`
    println!("-----------------取整-----------------");
    // round() 方法返回一个最接近的整数，四舍五入
    println!("13.14_f32.round() = {}", 13.14_f32.round());
    // floor() 方法返回一个最接近的整数，向下取整
    println!("13.99_f32.floor() = {}", 13.99_f32.floor());
}

fn study_float_traps() {
    // 应当避免在浮点数上测试相等性，因为二进制精度问题，
    // 导致了 0.1 + 0.2 并不严格等于 0.3，它们可能在小数点 N 位后存在误差。
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // to_bits() 返回了浮点数的二进制表示，并将其转换为整数类型。
    // {:x} 表示以十六进制的形式输出。

    // 对 f32 类型做加法时，0.1 + 0.2 的结果是 3e99999a，0.3 也是 3e99999a，
    // 因此 f32 下的 0.1 + 0.2 == 0.3 通过测试
    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    // 但是到了 f64 类型时，结果就不一样了，因为 f64 精度高很多，因此在小数点非常后面发生了一点微小的变化，
    // 0.1 + 0.2 以 4 结尾，但是 0.3 以3结尾，这个细微区别导致 f64 下的测试失败了
    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    println!("abc.0 + abc.1 == abc.2: {}", abc.0 + abc.1 == abc.2);
    println!("xyz.0 + xyz.1 == xyz.2: {}", xyz.0 + xyz.1 == xyz.2);
}

fn study_variable_binding() {
    // Rust中变量默认不可变，如果需要可变，需要使用 mut 关键字。
    // mut 表示可变的，也就是说，我们可以改变这个变量的值。
    println!("-----------------变量绑定-----------------");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 如果你创建了一个变量却不在任何地方使用它，Rust 通常会给你一个警告。
    // 使用下划线开头忽略未使用的变量。
    let _y = 6;

    // let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构，
    // 即从一个相对复杂的变量中，匹配出该变量的一部分内容。
    println!("-----------------解构式绑定-----------------");
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    // assert_eq! 是一个宏，它的作用是比较两个值是否相等，如果不相等，就会 panic。
    assert_eq!(a, b);

    // 解构式赋值
    println!("-----------------解构式赋值-----------------");
    study_variable_binding_destructuring_assignment();

    // 变量与常量
    // 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
    // 常量不允许使用 mut 关键字。
    println!("-----------------常量-----------------");
    // Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    // 变量遮蔽
    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的变量。
    println!("-----------------变量遮蔽-----------------");
    study_variable_binding_shadowing();
}

struct Struct {
    e: i32,
}

fn study_variable_binding_destructuring_assignment() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 与其他语言中的占位符类似，它表示忽略这个值。
    // .. 省略其余的值
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!("[a, b, c, d, e] = {:?}", [a, b, c, d, e]);
}

fn study_variable_binding_shadowing() {
    let x = 5;
    // 在函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        // x = 12
        println!("The value of x in the inner scope is: {}", x);
    }

    // x = 6
    println!("The value of x is: {}", x);

    // 假设有一个程序要统计一个空格字符串的空格数量
    let spaces = "   ";
    // 虽然 spaces.len()返回一个 usize 类型的值，但这种结构是允许的.
    // 以变量遮蔽可以帮我们节省些脑细胞，不用去想如 `spaces_str` 和 `spaces_num` 此类的变量名；
    // 相反我们可以重复使用更简单的 `spaces` 变量名。
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
}

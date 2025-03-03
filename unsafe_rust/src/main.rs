fn main() {
    println!("---------------------unsafe---------------------");
    // 使用 `unsafe` 非常简单，只需要将对应的代码块标记下即可:
    let num = 5;

    let r1 = &num as *const i32;

    unsafe {
        // r1` 是一个裸指针(raw pointer)，由于它具有破坏 Rust 内存安全的潜力，
        // 因此只能在 `unsafe` 代码块中使用，如果你去掉 `unsafe {}`，编译器会立刻报错。
        println!("r1 is: {}", *r1);
    }
    // `unsafe` 能赋予我们 5 种超能力，这些能力在安全的 Rust 代码中是无法获取的：
    // - 解引用裸指针，就如上例所示
    // - 调用一个 `unsafe` 或外部的函数
    // - 访问或修改一个可变的静态变量
    // - 实现一个 `unsafe` 特征
    // - 访问 `union` 中的字段

    // 作为使用者，你的水平决定了 unsafe 到底有多不安全，因此你需要在 unsafe 中小心谨慎地去访问内存。
    // 即使做到小心谨慎，依然会有出错的可能性，但是 unsafe 语句块决定了：
    // 就算内存访问出错了，你也能立刻意识到，错误是在 unsafe 代码块中，而不花大量时间像无头苍蝇一样去寻找问题所在。
    // 正因为此，写代码时要尽量控制好 unsafe 的边界大小，越小的 unsafe 越会让我们在未来感谢自己当初的选择。
    // 除了控制边界大小，另一个很常用的方式就是在 unsafe 代码块外包裹一层 safe 的 API，
    // 例如一个函数声明为 safe 的，然后在其内部有一块儿是 unsafe 代码。

    // 解引用裸指针
    study_deref_raw_pointer();

    // 调用 unsafe 函数或方法
    study_call_unsafe_function();

    // FFI
    study_ffi();

    // 访问 union 中的字段
    study_union();
}

fn study_union() {
    println!("---------------------study_union---------------------");
    // 截止目前，我们还没有介绍过 `union` ，原因很简单，它主要用于跟 `C` 代码进行交互。
    // 访问 union 的字段是不安全的，因为 Rust 无法保证当前存储在 union 实例中的数据类型。
    // union 的所有字段都共享同一个存储空间，意味着往 union 的某个字段写入值，
    // 会导致其它字段的值会被覆盖。

    // #[repr(C)]这是最重要的一种 repr。它的目的很简单，就是和 C 保持一致。
    // 数据的顺序、大小、对齐方式都和你在 C 或 C++ 中见到的一摸一样。
    // 所有你需要通过 FFI 交互的类型都应该有 repr(C)，因为 C 是程序设计领域的世界语。
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: i8,
    }
    let mut u = MyUnion { f1: 0 };
    unsafe {
        println!("u.f1 = {}", u.f1);
        println!("u.f2 = {}", u.f2);

        u.f2 = -6;
        println!("u.f1 = {}", u.f1);
        println!("u.f2 = {}", u.f2);
    }
}

fn study_ffi() {
    println!("---------------------study_ffi---------------------");
    // `FFI`（Foreign Function Interface）可以用来与其它语言进行交互，
    // 但是并不是所有语言都这么称呼，例如 Java 称之为 `JNI（Java Native Interface）`。
    // Rust 这门语言依然很年轻，一些生态是缺失的，我们在写一些不是那么大众的项目时，
    // 可能会同时遇到没有相应的 Rust 库可用的尴尬境况，此时通过 `FFI` 去调用 C 语言的库就成了相当棒的选择。

    // 下面的例子演示了如何调用 C 标准库中的 `abs` 函数：
    // 要调用外部 C 函数（标准库、系统调用等），可以使用 libc crate，它包含了 C 标准库中的类型别名和函数定义。
    // 当然也可以自己声明外部 C 函数，而 Rust 默认会链接 libc 和 libm：
    unsafe extern "C" {
        // "C" 表示这些外部函数遵循 C 语言 ABI，ABI 规定了在汇编层如何调用这些函数。
        // 在 extern "C" 代码块中，我们列出了想要调用的外部函数的签名。
        // 其中 "C" 定义了外部函数所使用的应用二进制接口ABI (Application Binary Interface)：
        // ABI 定义了如何在汇编层面来调用该函数。在所有 ABI 中，C 语言的是最常见的。
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn study_call_unsafe_function() {
    println!("---------------------study_call_unsafe_function---------------------");
    // 裸指针这么危险，为何还要使用？除了之前提到的性能等原因，
    // 还有一个重要用途就是跟 `C` 语言的代码进行交互( FFI )。

    // unsafe 函数从外表上来看跟普通函数并无区别，唯一的区别就是它需要使用 unsafe fn 来进行定义。
    // 强制调用者加上 unsafe 语句块，就可以让他清晰的认识到，正在调用一个不安全的函数，需要小心看看文档。
    // unsafe 无需俄罗斯套娃，在 unsafe 函数体中使用 unsafe 语句块是多余的行为。
    unsafe fn dangerous() {
        println!("dangerous");
    }
    unsafe {
        dangerous();
    }

    // 用安全抽象包裹 unsafe 代码
    // 想象一下这个场景：需要将一个数组分成两个切片，且每一个切片都要求是可变的。
    // 类似需求在安全 Rust 中是很难实现的，因为要对同一个数组做两个可变借用：
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        // as_mut_ptr 会返回指向 slice 首地址的裸指针 *mut i32
        let ptr = slice.as_mut_ptr();

        // 这段代码我们怎么保证 unsafe 中使用的裸指针 ptr 和 ptr.add(mid) 是合法的呢？
        // 秘诀就在于 assert!(mid <= len); ，通过这个断言，
        // 我们保证了裸指针一定指向了 slice 切片中的某个元素，而不是一个莫名其妙的内存地址。
        assert!(mid <= len);

        use std::slice;
        unsafe {
            (
                // ptr.add(mid) 可以获取第二个切片的初始地址，由于切片中的元素是 i32 类型，
                // 每个元素都占用了 4 个字节的内存大小，因此我们不能简单的用 ptr + mid 来作为初始地址，
                // 而应该使用 ptr + 4 * mid，但是这种使用方式并不安全，因此 .add 方法是最佳选择
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    // 对于 Rust 的借用检查器来说，它无法理解我们是分别借用了同一个切片的两个不同部分，
    // 但事实上，这种行为是没任何问题的，毕竟两个借用没有任何重叠之处。
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    println!("a: {:?}, b: {:?}", a, b);
}

fn study_deref_raw_pointer() {
    println!("---------------------study_deref_raw_pointer---------------------");
    // 裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性，
    // 裸指针长这样: *const T 和 *mut T，它们分别代表了不可变和可变。
    // 在裸指针 *const T 中，这里的 * 只是类型名称的一部分，并没有解引用的含义。
    let num = 5;
    // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为
    let r1 = &num as *const i32;
    unsafe {
        println!("r1 is: {}", *r1);
    }

    // 基于内存地址创建裸指针
    // 这种行为是相当危险的。试图使用任意的内存地址往往是一种未定义的行为(undefined behavior)，
    // 因为该内存地址有可能存在值，也有可能没有
    let address = 0x0usize;
    let _r = address as *const i32;

    // 获取字符串的内存地址和长度
    fn get_memory_location() -> (usize, usize) {
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }

    // 在指定的内存地址读取字符串
    use std::{slice::from_raw_parts, str::from_utf8_unchecked};
    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);\

    // 基于智能指针创建裸指针
    println!("---------------------基于智能指针创建裸指针---------------------");
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);
    unsafe {
        println!("b: {}", *b);
        println!("c: {}", *c);
    }
}

pub fn study_rust_type() {
    println!("--------------------深入Rust类型--------------------");
    study_newtype();

    // 类型别名(Type Alias)
    study_type_alias();

    // Sized 和不定长类型 DST
    study_sized_and_dst();

    // Any
    study_any();
}

fn study_any() {
    println!("--------------------Any--------------------");
    // This module implements the Any trait, which enables dynamic typing
    // of any 'static type through runtime reflection
    // Any Trait，它允许 'static 类型通过运行时反射，实现动态类型。
    // 所谓 Runtime Reflection，就是在运行的时候，可以判断和操作一个对象、变量等的信息，
    // 不需要在编译期知道对象的信息。

    // TODO: 未完待续
}

fn study_sized_and_dst() {
    println!("--------------------Sized 和不定长类型 DST--------------------");
    // 如果从编译器何时能获知类型大小的角度出发，可以分成两类:
    // - 定长类型( sized )，这些类型的大小在编译时是已知的
    // - 不定长类型( unsized )，与定长类型相反，它的大小只有到了程序运行时才能动态获知，这种类型又被称之为 DST
    // 之前学过的几乎所有类型，都是固定大小的类型，包括集合 `Vec`、`String` 和 `HashMap` 等，
    // 而动态大小类型刚好与之相反：编译器无法在编译期得知该类型值的大小，只有到了程序运行时，才能动态获知。
    // 对于动态类型，我们使用 `DST`(dynamically sized types)或者 `unsized` 类型来称呼它。

    // 上述的这些集合虽然底层数据可动态变化，感觉像是动态大小的类型。
    // 但是实际上，这些底层数据只是保存在堆上，在栈中还存有一个引用类型，
    // 该引用包含了集合的内存地址、元素数目、分配空间信息，通过这些信息，
    // 编译器对于该集合的实际大小了若指掌，最最重要的是：
    // 栈上的引用类型是固定大小的，因此它们依然是固定大小的类型。

    // 常见的动态大小类型：
    // - 特征对象(列如 Box<dyn Trait>、&dyn Trait)，只能通过引用或 `Box` 的方式来使用特征对象，直接使用将报错！
    //   函数能直接传递特征对象，是因为编译期做了类型推导，本质上是泛型的语法糖，
    //   而如果要返回多种不同类型的特征对象，就需要使用 `Box<dyn Trait>`或 引用。
    //   trait不是类型，dyn trait是?sized类型，只能用指针(引用)间接使用它。
    // - 切片(列如 str、[i32] , 其实str、[i32]是切片，&str、&[i32]是切片的引用，
    //   由于切片在编译时不能确定大小导致报错，所以一般使用切片的引用)
    // error
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // Rust 需要明确地知道一个特定类型的值占据了多少内存空间，同时该类型的所有值都必须使用相同大小的内存。
    // 如果 Rust 允许我们使用这种动态类型，那么这两个 `str` 值就需要占用同样大小的内存，这显然是不现实的:
    // `s1` 占用了 12 字节，`s2` 占用了 15 字节，总不至于为了满足同样的内存大小，用空白字符去填补字符串吧？
    // 所以，我们只有一条路走，那就是给它们一个固定大小的类型：`&str`。那么为何字符串切片 `&str` 就是固定大小呢？
    // 因为它的引用存储在栈上，具有固定大小(类似指针)，同时它指向的数据存储在堆中，也是已知的大小，
    // 再加上 `&str` 引用中包含有堆上数据内存地址、长度等信息，因此最终可以得出字符串切片是固定大小类型的结论。
    // 与 `&str` 类似，`String` 字符串也是固定大小的类型。

    // 试图创建动态大小的数组, error
    // fn my_function(n: usize) {
    //     let array = [123; n];
    // }

    // 总结：只能间接使用的 DST
    println!("--------------------只能间接使用的 DST--------------------");
    // 为泛型类型参数加上 ?Sized 的约束，表示该类型可以是具有固定大小的类型，也可以是不具有固定大小的类型。
    fn print_size<T: ?Sized>(x: &T) {
        println!("Size of type T: {}", std::mem::size_of_val(x));
    }
    let x: u32 = 42;
    print_size(&x); // 输出 "Size of type T: 4"
    let b: bool = true;
    print_size(&b); // 输出 "Size of type T: 1"
    let bb = &b;
    print_size(&bb); // 输出 "Size of type T: 8"
    print_size(bb); // 输出 "Size of type T: 1"

    let x: Box<dyn std::any::Any> = Box::new(42);
    print_size(&x);
    // Any 不具有固定大小(模拟动态类型的特征，必须具有static生命周期)，
    // 需要使用 ?Sized 约束来表示它可以是具有固定大小的类型，也可以是不具有固定大小的类型。
    // 没有?Sized约束的话，下面这行代码会报错。
    print_size(&*x); // 先解Box引用再取地址

    // Sized 特征
    println!("--------------------Sized 特征--------------------");
    // 既然动态类型的问题这么大，那么在使用泛型时，Rust 如何保证我们的泛型参数是固定大小的类型呢？
    // 奥秘在于编译器自动帮我们加上了 `Sized` 特征约束：`T: Sized`，这就是说，泛型参数 `T` 必须是固定大小的类型。
    // 你能想到的几乎所有类型都实现了 `Sized` 特征，除了上面那个坑坑的 `str`，哦，还有特征。
    let x: i8 = 5;
    let y = &x;
    println!("x = {}, y = {}", x, y);
    println!(
        "size of x = {}, size of y = {}",
        std::mem::size_of_val(&x), // 1 byte
        std::mem::size_of_val(&y)  // 8 byte
    );

    // 使用 `Box` 可以将一个动态大小的特征变成一个具有固定大小的特征对象，
    // 能否故技重施，将 `str` 封装成一个固定大小类型？
    // 答案是不可以！
    // the size for values of type `str` cannot be known at compilation time.
    // let s1: Box<str> = Box::new("Hello there!" as str);
    // `Box<str>` 使用了一个引用来指向 `str`，引用本身就是一个固定大小的类型，但是
    //  Box` 中有该 `str` 的长度信息吗？显然是 `No`。
    // 那为什么特征就可以变成特征对象？其实这个还蛮复杂的，简单来说，
    // 对于特征对象，编译器无需知道它具体是什么类型，只要知道它能调用哪几个方法即可，
    // 因此编译器帮我们实现了剩下的一切。

    // 主动转换成 `str` 的方式不可行，但是可以让编译器来帮我们完成，只要告诉它我们需要的类型即可。
    let s1: Box<str> = "Hello there!".into();
    // 相当于下面的代码
    let s2: Box<str> = Box::from("How's it going?");
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

fn study_type_alias() {
    println!("--------------------类型别名(Type Alias)--------------------");
    // 除了使用 `newtype`，我们还可以使用一个更传统的方式来创建新类型：类型别名。
    //  类型别名并不是一个独立的全新的类型，而是某一个类型的别名。
    type Meters = u32;
    let x: u32 = 5;
    let y: Meters = 5;
    println!("x + y = {}", x + y);
}

fn study_newtype() {
    println!("--------------------newtype--------------------");
    // newtype
    // 何为 newtype？简单来说，就是使用元组结构体的方式将已有的类型包裹起来：`struct Meters(u32);`，
    // 那么此处 `Meters` 就是一个 `newtype`。
    // 自定义类型可以让我们给出更有意义和可读性的类型名，
    // 例如与其使用 `u32` 作为距离的单位类型，我们可以使用 `Meters`，它的可读性要好得多

    // 为外部类型实现外部特征
    println!("--------------------为外部类型实现外部特征(见其他文件)--------------------");

    // 更好的可读性及类型异化
    println!("--------------------更好的可读性及类型异化--------------------");

    struct Meters(u32);
    impl std::fmt::Display for Meters {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "目标地点距离你{}米", self.0)
        }
    }

    impl std::ops::Add for Meters {
        type Output = Self;

        fn add(self, other: Meters) -> Self {
            Self(self.0 + other.0)
        }
    }

    fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
        d1 + d2
    }
    let d = calculate_distance(Meters(10), Meters(20));
    println!("{}", d);

    // 隐藏内部类型的细节
    println!("--------------------隐藏内部类型的细节--------------------");
    // 众所周知，Rust 的类型有很多自定义的方法，假如我们把某个类型传给了用户，
    // 但是又不想用户调用这些方法，就可以使用 `newtype`。
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    println!("n = {}", n);
    // 下面的代码将报错，因为`Meters`类型上没有`pow`方法
    // assert_eq!(n.pow(2), 4);
    // 不过需要偷偷告诉你的是，这种方式实际上是掩耳盗铃，
    // 因为用户依然可以通过 `n.0.pow(2)` 的方式来调用内部类型的方法 :)
}

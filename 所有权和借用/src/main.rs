mod study;

// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者。
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
fn main() {
    println!("--------------------初识所有权---------------------");
    // hello分配在堆上,s1获得这个String的所有权
    let s1 = String::from("hello");
    // s2获得s1指向的String的所有权，s1将不能再访问这个String
    let s2 = s1;
    // println!("{}, world!", s1); // error: value borrowed here after move
    println!("{}, world!", s2);

    test_func1();
    println!("--------------------深拷贝---------------------");
    // Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝。
    // 如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 浅拷贝
    // 浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
    println!("--------------------浅拷贝---------------------");
    // 代码背后的逻辑很简单, 将 `5` 绑定到变量 `x`；接着拷贝 `x` 的值赋给 `y`，最终 `x` 和 `y` 都等于 `5`，
    // 因为整数是 Rust 基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，
    // 都被存在栈中，完全无需在堆上分配内存。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // 但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，
    // 不过依然实现了类似深拷贝的效果 —— 没有报所有权的错误。
    // 原因是像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。
    // 换句话说，这里没有深浅拷贝的区别，因此这里调用 clone`并不会与通常的浅拷贝有什么不同，
    // 我们可以不用管它（可以理解成在栈上做了深拷贝）。
    // Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。
    // 如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。
    // 一个通用的规则：任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。
    // 不可变引用 &T 可以 Copy ，但是注意: 可变引用 &mut T 是不可以 Copy的。
    let str1 = "hello";
    let str2 = str1;
    println!("str1 = {}, str2 = {}", str1, str2);

    // 再识所有权
    study_ownership();
    // 同样的，函数返回值也有所有权
    study_return_ownership();
    // 引用和借用
    study::study_reference_and_borrowing();
}

fn study_return_ownership() {
    println!("--------------------函数返回值所有权---------------------");
    let s1 = gives_ownership(); // gives_ownership 将返回值 移给 s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello2"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到  takes_and_gives_back 中
    println!("s3 = {}", s3);

    // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃。
}

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello1"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

fn study_ownership() {
    println!("--------------------再识所有权---------------------");
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    //println!("s = {}", s); // error: value borrowed here after move

    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x

    // 函数结束后 x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn test_func1() {
    println!("--------------------引用---------------------");
    // 这段代码和之前的 `String` 有一个本质上的区别：
    // 在 `String` 的例子中 `s1` 持有了通过`String::from("hello")` 创建的值的所有权，
    // 而这个例子中，`x` 只是引用了存储在二进制中的字符串 `"hello, world"`，并没有持有所有权。
    // 因此 `let y = x` 中，仅仅是对该引用进行了拷贝，此时 `y` 和 `x` 都引用了同一个字符串。
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
    // &str是不可变引用。
}

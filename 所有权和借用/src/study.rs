// 总的来说，借用规则如下：
// - 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
// - 引用必须总是有效的

pub fn study_reference_and_borrowing() {
    println!("--------------------引用(借用)---------------------");
    // 如果仅仅支持通过转移所有权的方式获取一个值，那会让程序变得复杂。
    // Rust 能否像其它编程语言一样，使用某个变量的指针或者引用呢？答案是可以。
    let x = 5;
    // & 符号即是引用，它们允许你使用值，但是不获取所有权
    let y = &x; // y 是 x 的引用
    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用 *y 来解出引用所指向的值（也就是解引用）

    // 不可变引用
    study_immutable_reference();
    // 可变引用
    study_mutable_reference();
    // 可变引用与不可变引用不能同时存在
    study_mutable_and_immutable_reference();
    // 悬垂引用(Dangling References)
    study_dangling_references();
}

fn study_dangling_references() {
    println!("--------------------悬垂引用(Dangling References)---------------------");
    // 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，
    // 其指向的内存可能不存在任何值或已被其它变量重新使用。

    // 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：
    // 当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用

    // 让我们尝试创建一个悬垂引用，Rust 会抛出一个编译时错误。
    // let reference_to_nothing = dangle();

    // 其中一个很好的解决方法是直接返回 String。 最终 String 的 所有权被转移给外面的调用者。
    let s = no_dangle();
    println!("s = {}", s);
}
/*
// 让我们尝试创建一个悬垂引用，Rust 会抛出一个编译时错误
fn dangle() -> &String {
    // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn study_mutable_and_immutable_reference() {
    println!("--------------------可变引用与不可变引用不能同时存在---------------------");
    // 可变引用与不可变引用不能同时存在
    let mut _s = String::from("hello");
    let r1 = &_s; // 没问题
    let r2 = &_s; // 没问题
                  // 无法借用可变 _s 因为它已经被借用了不可变
                  // let r3 = &mut _s; // 大问题

    println!("{}, {}", r1, r2);

    // 引用的作用域从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，
    // 变量的作用域从创建持续到某一个花括号 `}`。
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
    // 新编译器中，r3作用域在这里结束

    // 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL) ，
    // 专门用于找到某个引用在作用域(`}`)结束前就不再被使用的代码位置。
}

fn study_mutable_reference() {
    println!("--------------------可变引用---------------------");
    let mut s = String::from("hello");

    change(&mut s);
    println!("s = {}", s);
    // 可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用。
    // 这种限制的好处就是使 Rust 在编译期就避免数据竞争。
    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    r1.push_str(", it's me");
    println!("s = {}", s);

    // 很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域。
    {
        let r2 = &mut s;
        r2.push_str("🤣");
        // r2 在这里离开了作用域，所以我们完全可以创建一个新的引用
    }
    let r1 = &mut s;
    println!("s = {}", r1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn study_immutable_reference() {
    // 若存在不可变引用，即使原始变量拥有所有权，也不能再创建可变引用(即引用的值不会被修改)
    println!("--------------------不可变引用---------------------");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // 正如变量默认不可变一样，引用指向的值默认也是不可变的
    // s.push_str(", world!"); //cannot borrow `*s` as mutable, as it is behind a `&` reference `s` is a `&` reference,

    s.len()
    // &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。
    // 因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃。
}

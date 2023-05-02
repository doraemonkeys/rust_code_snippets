pub fn study_closure() {
    println!("------------------闭包------------------");
    // Rust 中的闭包（closure），也叫做 lambda 表达式或者 lambda，是一类能够捕获周围作用域中变量的函数。
    // 声明时使用 `||` 替代 `()` 将输入参数括起来。
    // 函数体定界符`{}`对于单个表达式是可选的，其他情况必须加上。

    // 通过闭包和函数分别实现自增。
    // 译注：下面这行是使用函数的实现
    fn function(i: i32) -> i32 {
        i + 1
    }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    // 译注：将闭包绑定到引用的说法可能不准。
    // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // `closure_xxx` 变量解引用。

    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i)); //2
    println!("closure_annotated: {}", closure_annotated(i)); //2
    println!("closure_annotated: {}", closure_annotated(i)); //2
    println!("closure_inferred: {}", closure_inferred(i)); //2

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one()); // 1

    // 捕获
    study_closure_capture();

    // 作为参数传递闭包
    study_closure_as_parameter();

    // 作为返回值返回闭包
    study_closure_as_return_value();

    // 闭包的生命周期
    study_closure_lifetime();
}

fn study_closure_lifetime() {
    println!("------------------闭包的生命周期------------------");
    fn _fn_elision(x: &i32) -> &i32 {
        x
    }
    // let closure_slision = |x: &i32| -> &i32 { x };
    // 两个一模一样功能的函数，一个正常编译，一个却报错，
    // 错误原因是编译器无法推测返回的引用和传入的引用谁活得更久！
    // 对于函数的生命周期而言，它的消除规则之所以能生效是因为它的生命周期完全体现在签名的引用类型上，在函数体中无需任何体现。
    // 可是闭包，并没有函数那么简单，它的生命周期分散在参数和闭包函数体中(主要是它没有确切的返回值签名)。
    // 编译器就必须深入到闭包函数体中，去分析和推测生命周期，复杂度因此极具提升。

    // 用 `Fn` 特征解决闭包生命周期。
    let closure_slision = func(|x: &i32| -> &i32 { x });
    let x = 5;
    println!("closure_slision: {}", closure_slision(&x));
    // 虽然可以解决，但还是建议大家遇到后，还是老老实实用正常的函数，不要秀闭包了。
}

fn func<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn study_closure_as_return_value() {
    println!("------------------作为返回值返回闭包------------------");
    // 匿名的闭包的类型是未知的，所以只有使用`impl Trait`才能返回一个闭包。
    // 除此之外，还必须使用 `move` 关键字，它表明所有的捕获都是通过值进行的。
    // 这是必须的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32
where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32,
{
    f(3)
}

fn study_closure_as_parameter() {
    println!("------------------作为参数传递闭包------------------");

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        std::mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

fn study_closure_capture() {
    println!("------------------闭包捕获------------------");

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print(); //`color`: green

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;
    print(); // `color`: green

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    println!("------------------使用可变借用调用闭包------------------");
    inc(); // `count`: 1

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc(); // `count`: 2

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    println!("------------------不可复制类型------------------");
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。
    // 不可复制类型则必须移动（move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    // 此时 consume闭包 类型也变成了 FnOnce。
    let consume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable); // 试一试：将此行注释，consume闭包类型将变成Fn(对movable的引用)
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}

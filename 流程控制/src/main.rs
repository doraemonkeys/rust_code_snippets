#[warn(unreachable_code)]
fn main() {
    // 语句和表达式
    study_statement_and_expression(4, 5);
    // 函数
    study_function(1, 2);
    // if表达式
    study_if_expression(1, 2);
    // 循环
    study_loop();
    // 迭代器
    study_iterator();
}

fn study_iterator() {
    println!("------------------迭代器------------------");
    let arr = [1, 2, 3];
    // arr不是一个迭代器，我们却可以在for in中使用它，这是因为Rust为数组实现了IntoIterator trait。
    // Rust 通过 `for` 语法糖，自动把实现了该特征的数组类型转换为迭代器(into_iter)，最终让我们可以直接对一个数组进行迭代
    for v in arr {
        println!("{}", v);
    }
    // 迭代器之所以成为迭代器，就是因为实现了 `Iterator` 特征，
    // 要实现该特征，最主要的就是实现其中的 `next` 方法
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    println!("------------------迭代器------------------");

    // 由于 `Vec` 动态数组实现了 `IntoIterator` 特征，因此可以通过 `into_iter` 将其转换为迭代器，
    // 那如果本身就是一个迭代器，该怎么办？实际上，迭代器自身也实现了 `IntoIterator`。
    // 最终你完全可以写出这样的奇怪代码。
    let values = vec![1, 2, 3];
    for v in values.into_iter().into_iter().into_iter() {
        println!("{}", v)
    }

    println!("------------------消费者与适配器------------------");
    // 只要迭代器上的某个方法 `A` 在其内部调用了 `next` 方法，那么 `A` 就被称为消费性适配器：
    // 因为 `next` 方法会消耗掉迭代器上的元素，所以方法 `A` 的调用也会消耗掉迭代器上的元素。
    // 既然消费者适配器是消费掉迭代器，然后返回一个值。那么迭代器适配器，顾名思义，
    // 会返回一个新的迭代器(实现了迭代器特征的类型)，这是实现链式方法调用的关键：`v.iter().map().filter()...`。
    // 与消费者适配器不同，迭代器适配器是惰性的，意味着你需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值。

    // 这里的 `map` 方法是一个迭代器适配器，它是惰性的，不产生任何行为，因此我们还需要一个消费者适配器进行收尾。
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // 上面代码中，使用了 `collect` 方法，该方法就是一个消费者适配器，
    // 使用它可以将一个迭代器中的元素收集到指定类型中。
    // 为何 `collect` 在消费时要指定类型？是因为该方法其实很强大，可以收集成多种不同的集合类型，
    // `Vec<T>` 仅仅是其中之一，因此我们必须显式的告诉编译器我们想要收集成的集合类型。
    // `map` 会对迭代器中的每一个值进行一系列操作，然后把该值转换成另外一个新值，
    // 该操作是通过闭包 `|x| x + 1` 来完成。
    println!("v2 = {:?}", v2);

    // 使用 `collect` 收集成 `HashMap` 集合：
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: std::collections::HashMap<_, _> = names.into_iter().zip(ages).collect();
    // zip 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，
    // 形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器，
    // 在此处就是形如 `[(name1, age1), (name2, age2)]` 的迭代器。
    println!("{:?}", folks);

    // 实现 Iterator 特征
    study_iterator_impl();
}

fn study_iterator_impl() {
    println!("------------------实现 Iterator 特征------------------");
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    // 可以看出，实现自己的迭代器非常简单，但是 `Iterator` 特征中，不仅仅是只有 `next` 一个方法，
    // 那为什么我们只需要实现它呢？因为其它方法都具有默认实现，
    // 所以无需像 `next` 这样手动去实现，而且这些默认实现的方法其实都是基于 `next` 方法实现的。
    let counter = Counter::new();
    for i in counter {
        println!("{}", i);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // 将 `[1, 2, 3, 4, 5]` 和 `[2, 3, 4, 5]` 的迭代器合并后，新的迭代器形如 `[(1, 2),(2, 3),(3, 4),(4, 5)]`
    // `map` 是将迭代器中的值经过映射后，转换成新的值[2, 6, 12, 20]，
    // `filter` 对迭代器中的元素进行过滤，若闭包返回 `true` 则保留元素[6, 12]，反之剔除
    // 而 `sum` 是消费者适配器，对迭代器中的所有元素求和，最终返回一个 `u32` 值 `18`。
    println!("sum = {}", sum);

    // enumerate
    println!("------------------enumerate------------------");
    // `v.iter()` 创建迭代器，其次 调用 `Iterator` 特征上的方法 `enumerate`，
    // 该方法产生一个新的迭代器(实现了迭代器特征的类型)，其中每个元素均是元组 `(索引，值)`。
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第{}个值是{}", i, v)
    }

    // 迭代器的性能
    println!("------------------迭代器的性能------------------");
    // 迭代器的性能是非常高效的，因为它是零成本抽象的一部分，这意味着它不会引入运行时开销。
    // 迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，意味着抽象并不会引入运行时开销，
    // 这与 `Bjarne Stroustrup`（C++ 的设计和实现者）在 `Foundations of C++（2012）` 中
    // 所定义的 零开销（zero-overhead）如出一辙。
}

fn study_loop() {
    println!("------------------循环------------------");
    let a = [10, 20, 30, 40, 50];
    // for 循环可以使用 iter 方法来获取集合的迭代器。
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for 循环可以使用 enumerate 方法来同时获取元素的索引和值。
    for (index, element) in a.iter().enumerate() {
        println!("index = {}, the value is: {}", index, element);
    }

    // for 循环也可以使用 Range 类型来迭代数字，Range 类型是一个左闭右开的区间。
    for number in (1..4).rev() {
        // rev()方法用于反转迭代器
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // for item in collection 等价于 for item in IntoIterator::into_iter(collection) --- 会转移所有权
    // for item in &collection 等价于 for item in collection.iter() --- 不可变引用
    // for item in &mut collection 等价于 for item in collection.iter_mut() --- 可变引用
    let mut v = vec![1, 2, 3, 4, 5]; // vec! 宏可以创建一个新的 vector
    for i in &mut v {
        *i += 50; // 解引用
    }
    println!("v = {:?}", v);
    // 1.通过索引下标去访问集合 与 2.直接循环集合中的元素
    // 1.通过索引下标去访问集合
    println!("------------------通过索引下标去访问集合------------------");
    // collection[index] 索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗。
    // 这里collection的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        print!("{} ", item);
    }
    println!();
    // 2.直接循环集合中的元素
    // 直接迭代的方式就不会有边界检查的性能损耗。
    // 对于基础类型或者实现了 Copy trait 的类型，直接迭代会Copy一份数据，
    // 对于其他类型，直接迭代会移动所有权，这样就不会存在脏数据的问题。
    println!("------------------直接循环集合中的元素------------------");
    let mut collection = [5, 6, 7, 8, 9];
    for item in collection {
        if item == 5 {
            // 可以看到即使这里修改了collection[1]的值，但是在循环中的item的值并没有改变。
            collection[1] = 10;
        }
        print!("{} ", item);
    }
    println!();
    println!("collection = {:?}", collection);

    // while循环
    println!("------------------while循环------------------");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // loop循环
    println!("------------------loop循环------------------");
    let mut number = 3;
    loop {
        println!("{}!", number);
        number -= 1;
        if number == 0 {
            break;
        }
    }
    // loop 循环可以使用标签来指定循环的返回点，这样就可以在循环中使用 break 来返回到指定的标签处。
    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            // 这里使用 break 'outer 来返回到 outer 标签处。
            break 'outer;
        }
        // println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    // loop 也是表达式，可以通过 break 来返回值。
    let result = loop {
        number += 1;
        if number == 10 {
            break number * 2;
        }
    };
    println!("The result is {}", result);
}

fn study_if_expression(a: i32, b: i32) {
    println!("------------------if表达式------------------");
    // if 语句块是表达式，所谓表达式，就是可以返回值的语句块。
    // 用 if 来赋值时，要保证每个分支返回的类型一样。
    let c = if a > b { a } else { b };
    println!("c = {}", c);
}

// Rust规范中函数名和变量名使用蛇形命名法(snake case)。
fn study_function(a: i32, b: i32) -> i64 {
    println!("------------------函数------------------");
    let r = myadd(a, b);
    println!("r = {}", r);

    //永不返回的发散函数 !
    // _diverges();

    return 99;
}

//永不返回的发散函数 !
fn _diverges() -> ! {
    // panic` 的返回值是 `!`，代表它决不会返回任何值。
    panic!("This function never returns!");
}

fn myadd(a: i32, b: i32) -> i32 {
    println!("a + b = {}", a + b);
    a + b
}

// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值。
// 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值。
// 表达式会进行求值，然后返回一个值。例如 `5 + 6`，在求值后，返回值 `11`，因此它就是一条表达式。
// 表达式可以成为语句的一部分，例如 `let y = 6` 中，`6` 就是一个表达式，它在求值后返回一个值 `6`
fn study_statement_and_expression(x: i32, y: i32) -> i32 {
    println!("------------------语句和表达式------------------");
    let x = x + 1; // 语句
    let y = y + 5; // 语句

    // 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，
    // 用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式。
    // 表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句。
    let m = {
        let n = 3;
        n + 1
    };
    println!("The value of y is: {}", m);

    // 表达式如果不返回任何值，会隐式地返回一个 ()单元类型。
    assert_eq!(ret_unit_type(), ());

    // 由于 `let` 是语句，因此不能将 `let` 语句赋值给其它值，如下形式是错误的：
    // let b = (let a = 8);
    x + y // 表达式
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 { "odd" } else { "even" };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}

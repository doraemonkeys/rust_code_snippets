pub fn study_generic() {
    study_generic2();

    // 幽灵数据
    study_phantom_data();
}

fn study_phantom_data() {
    println!("------------------幽灵数据------------------");
    // 在处理不安全代码时，我们经常会遇到这样的情况：
    // 类型或生命周期在逻辑上与结构相关，但实际上并不是字段的一部分。
    // 这种情况最常发生在生命周期上。例如，&'a [T]的Iter（大约）定义如下：
    // struct Iter<'a, T: 'a> { // parameter `'a` is never used
    //     ptr: *const T,
    //     end: *const T,
    // }
    // 由于'a在结构体中是未使用的，所以它是无约束的。
    // 在结构定义中，不受约束的生命周期和类型是禁止的，
    //  因此我们必须在主体中以某种方式引用这些类型，正确地做到这一点对于正确的变异性和丢弃检查是必要的。

    // 我们使用PhantomData来做这个，它是一个特殊的标记类型。PhantomData不消耗空间，
    // 但为了静态分析的目的，模拟了一个给定类型的字段。这被认为比明确告诉类型系统你想要的变量类型更不容易出错，
    // 同时也提供了其他有用的东西，例如 auto traits 和 drop check 需要的信息。
    // Iter 逻辑上包含一堆&'a T，所以这正是我们告诉PhantomData要模拟的。
    use std::marker;
    struct _Iter<'a, T: 'a> {
        ptr: *const T,
        end: *const T,
        _marker: marker::PhantomData<&'a T>,
    }
    // 就是这样，生命周期将被限定，而你的迭代器将在'a和T上进行协变。所有的东西都是有效的。
}

fn study_generic2() {
    println!("------------------泛型------------------");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("number_list: {:?}", number_list);
    println!("The largest number is {}", result);
    let result = largest2(&number_list);
    println!("The largest number is {}", result);

    let a = 1;
    let b = 2;
    let c = my_add(&a, &b);
    println!("{} + {} = {}", a, b, c);
    // let str1 = String::from("hello");
    // let str2 = String::from("world");
    // let str3 = my_add(&str1, &str2);

    // 泛型约束可以后置
    println!("------------------泛型约束可以后置------------------");
    // 当特征约束变得很多时，函数的签名将变得很复杂，
    // 这时可以将特征约束放到 `where` 子句中，这样就可以将函数签名变得简单。
    let a = 1;
    let b = 2;
    let c = my_add2(&a, &b);
    println!("{} + {} = {}", a, b, c);

    // 结构体中使用泛型
    println!("------------------结构体中使用泛型------------------");
    let p1 = Point { _x: 1, _y: 2 };
    let p2 = Point { _x: 1.0, _y: 2.0 };
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    // 枚举中使用泛型
    println!("------------------枚举中使用泛型------------------");
    // Option<T> 是一个枚举，它有两个成员：Some 和 None。
    // Result<T, E> 是一个枚举，它有两个成员：Ok 和 Err。
    study_result();
    // 方法中使用泛型
    let p = Point { _x: 1, _y: 2 };
    println!("p.x: {}", p.x());
    // 为具体的泛型类型实现方法
    println!("------------------为具体的泛型类型实现方法------------------");
    let p = Point { _x: 1.0, _y: 2.0 };
    println!("p.distance_from_origin: {}", p.distance_from_origin());
    // const 泛型，也就是针对值的泛型
    println!("------------------const 泛型，也就是针对值的泛型------------------");
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
    // 调用方法需要引入特征
    let a: i32 = 10;
    let b: u16 = 100;
    // try_into() 是一个用于将一个类型转换为另一种类型的方法，
    // TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 。
    let b_ = b.try_into().unwrap();
    // 如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中，
    // 我们在上面用到了 try_into 方法，因此需要引入对应的特征。
    // 这里没有引入 std::convert::TryInto trait，是因为std::prelude 中已经引入了该特征。
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

// 可以使用 const 泛型来表示数组的长度。
// N 这个泛型参数，它是一个基于值的泛型参数！因为它用来替代的是数组的长度。
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// Result的使用
fn study_result() {
    println!("------------------Result的使用------------------");
    let f = std::fs::File::open("hello.txt");
    if let Ok(file) = f {
        println!("file: {:?}", file);
    } else {
        println!("open file failed, error: {:?}", f.err());
    }
}

#[derive(Debug)]
struct Point<T> {
    _x: T,
    _y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self._x
    }
}

// 为具体的泛型类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

// 泛型函数
// PartialOrd: 用于比较大小的trait
// Copy: 用于复制的trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for item in list.iter() {
        if item > &largest {
            largest = *item;
        }
    }
    largest
}

// 另一种 `largest` 的实现方式是返回在 `list` 中 `T` 值的引用。
// 此时，我们不需要 `Copy` trait。
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn my_add2<T>(x: &T, y: &T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    *x + *y
}

// 不是所有 T 类型都能进行相加操作，因此我们需要用 std::ops::Add<Output = T> 对 T 进行限制。
fn my_add<T: std::ops::Add<Output = T> + Copy>(x: &T, y: &T) -> T {
    *x + *y
}

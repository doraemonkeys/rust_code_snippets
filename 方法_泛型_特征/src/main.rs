mod closure;
mod generic;
mod inheritance;
mod operator_overloading;
mod polymorphism;
mod trait_study;
mod rust_type;
fn main() {
    // 方法
    study_method();
    // 泛型
    generic::study_generic();
    // 特征 trait
    trait_study::study_trait();
    // 闭包
    closure::study_closure();
    // 远算符重载
    operator_overloading::study_example();
    // 继承
    inheritance::study_inheritance();
    // 多态的例子
    polymorphism::study_polymorphism();
    //深入Rust类型
    rust_type::study_rust_type();
}

// 私有结构体，只能在当前模块中使用
struct Circle {
    _x: f64,
    _y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字。
    // 这种定义在 impl 中且没有 self 的函数被称之为关联函数：
    // 因为它没有 self，不能用 f.read() 的形式调用，因此它是一个函数而不是方法，
    // 它又在 impl 中，与结构体紧密关联，因此称为关联函数。
    // 这种方法往往用于初始化当前结构体的实例。
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            _x: x,
            _y: y,
            radius: radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    // 在 Rust 中，允许方法名跟结构体的字段名相同：
    fn radius(&self) -> f64 {
        self.radius
    }
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Self 指代的就是被实现方法的结构体 `Rectangle`。(注意大小写)
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
    pub fn height(&self) -> u32 {
        return self.height;
    }
}

// Rust 允许我们为一个结构体定义多个 `impl` 块，目的是提供更多的灵活性和代码组织性。
impl Rectangle {
    pub fn area(&self) -> u32 {
        return self.width * self.height;
    }
    // 带有多个参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        match self {
            Message::_Quit => println!("The Quit variant has no data to destructure."),
            Message::_Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::_ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }
}

fn study_method() {
    println!("------------------方法------------------");
    // Rust 中有一个约定俗成的规则，使用 `new` 来作为构造器的名称，
    // 但出于设计上的考虑，Rust 特地没有用 `new` 作为关键字。
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area: {}", c.area());
    // 在 Rust 中，允许方法名跟结构体的字段名相同：
    // Rust中的结构体默认是私有的，所以外部会无法访问结构体的属性,但对于和 mod 自身同层级的是可见的。
    println!("radius: {}", c.radius());
    println!("radius: {}", c.radius);
    let r = Rectangle::new(10, 20);
    println!("width: {}", r.width());
    println!("height: {}", r.height());
    println!("area: {}", r.area());

    // 带有多个参数的方法
    println!("------------------带有多个参数的方法------------------");
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // 为枚举实现方法
    println!("------------------为枚举实现方法------------------");
    let m = Message::Write(String::from("hello"));
    m.call();

    // 方法调用的点操作符
    println!("------------------方法调用的点操作符------------------");
    let array: std::rc::Rc<Box<[_; 3]>> = std::rc::Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0];
    // array 数组的底层数据隐藏在了重重封锁之后，
    // 那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁，准确的访问到数组中的第一个元素？
    // 1. 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，
    // 当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
    // 2. 接着，编译器检查 Rc<Box<[T; 3]>> 是否有实现 Index 特征，结果是否，
    // 不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
    // 3. 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
    // 4. 此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]>
    // 都没有实现 Index 特征，所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
    // 5. [T; 3] 以及它的各种引用都没有实现 Index 索引
    // (是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，
    // 它也不能再进行解引用，因此编译器只能祭出最后的大杀器：将定长转为不定长，
    // 因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，
    // 因此最终我们可以通过 index 方法访问到对应的元素。
    println!("first_entry: {}", first_entry);
}

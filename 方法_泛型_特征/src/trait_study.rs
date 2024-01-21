use std::fmt::Display;

pub fn study_trait() {
    println!("--------------------特征 trait--------------------");
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Twitter好用".to_string(),
    };
    let tweet = Tweet {
        username: "tom".to_string(),
        content: "Rust真好用".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    println!("{}", tweet.summarize());
    // 使用特征作为函数参数
    println!("----------------使用特征作为函数参数----------------");
    notify(&post);
    // 使用特征作为返回值
    println!("----------------使用特征作为返回值----------------");
    let summary = returns_summarizable();
    println!("{}", summary.summarize());
    // 使用特征作为返回值时，只允许函数多个分支返回同一类型的特征对象。
    // 如果需要返回不同类型的特征对象，可以使用特征对象。
    let summary = returns_summarizable2(true);
    println!("{}", summary.summarize());

    // 特征约束
    println!("----------------特征约束----------------");
    // `impl Trait` 这种语法非常好理解，但是实际上它只是一个语法糖。
    // 完整书写形式如下函数：
    notify2(&weibo);
    // 使用特征约束有条件地实现方法或特征
    println!("----------------使用特征约束有条件地实现方法或特征----------------");
    // `cmp_display` 方法，并不是所有的 `Pair<T>` 结构体对象都可以拥有，
    // 只有 `T` 同时实现了 `Display + PartialOrd` 的 `Pair<T>` 才可以拥有此方法。
    // 当Pair<T> 实现了 CmpDisplay trait 时，自动实现了 Foo trait。
    // 例如，标准库为任何实现了 `Display` 特征的类型实现了 `ToString` 特征：
    let pair = Pair::new(1, 2);
    pair.cmp_display();
    pair.foo();
    println!("pair.to_string: {}", pair.to_string());
    let num = 2;
    let num_str = num.to_string();
    println!("num_str: {}", num_str);

    //  特征对象
    study_trait_object();

    // 关联类型
    study_associated_type();

    // 调用同名方法
    study_same_name_method();

    // 特征定义中的特征约束
    study_trait_constraint_in_trait_def();

    // 在外部类型上实现外部特征(newtype)，以此绕过孤儿规则，这种方式称为特征适配器。
    study_trait_adapter();

    // 父 trait
    study_super_trait();

    // Eq 和 PartialEq
    study_eq_and_partial_eq();

    // AsRef 提高代码的复用性
    fn _example_as_ref<T: AsRef<[u8]>>(data: T) {
        let _data: &[u8] = data.as_ref();
        // do something
    }
}

fn study_eq_and_partial_eq() {
    println!("----------------Eq 和 PartialEq----------------");
    // 比如，+ 号需要实现 std::ops::Add 特征，
    // 而本文的主角 Eq 和 PartialEq 正是 == 和 != 所需的特征，
    // 那么问题来了，这两个特征有何区别？

    // 如果我们的类型只在部分情况下具有相等性，那你就只能实现 PartialEq，
    // 否则可以实现 PartialEq 然后再默认实现 Eq。
    enum BookFormat {
        _Paperback,
        _Hardback,
        _Ebook,
    }
    struct Book {
        isbn: i32,
        _format: BookFormat,
    }
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }
    impl Eq for Book {}
    // 这里就只实现了 PartialEq，并没有实现 Eq，而是直接使用了默认实现 impl Eq for Book {}

    // 部分相等性
    println!("----------------部分相等性----------------");
    // 在 HashMap 章节提到过 HashMap 的 key 要求实现 Eq 特征，也就是要能完全相等，
    // 而浮点数由于没有实现 Eq ，因此不能用于 HashMap 的 key。
    // 当时由于一些知识点还没有介绍，因此就没有进一步展开，
    // 那么让我们考虑浮点数既然没有实现 Eq 为何还能进行比较呢？
    let f1 = 3.14;
    let f2 = 3.14;

    if f1 == f2 {
        println!("f1 == f2");
    }
    fn _is_eq<T: Eq>(_f: T) -> bool {
        true
    }
    fn is_partial_eq<T: PartialEq>(_f: T) -> bool {
        true
    }
    // _is_eq(f1); // 编译错误 the trait Eq is not implemented for {float}
    println!("is_partial_eq(f1): {}", is_partial_eq(f1));

    // 我们成功找到了一个类型实现了 PartialEq 但没有实现 Eq，那就通过它来看看何为部分相等性。
    // 其实答案很简单，浮点数有一个特殊的值 NaN，它是无法进行相等性比较的:
    let f1 = f32::NAN;
    let f2 = f32::NAN;

    if f1 == f2 {
        println!("NaN 竟然可以比较，这很不数学啊！")
    } else {
        // 输出结果
        println!("果然，虽然两个都是 NaN ，但是它们其实并不相等")
    }

    // 偏序集合 的两个元素x和y可以处于四个相互排斥的关联中任何一个：
    // 要么x < y，要么x = y，要么x > y，要么x和y是“不可比较”的（前三个都不是）。
    // 全序集合 是上面规则排除第四种可能的集合：即所有元素对都是可比较的。

    // 既然浮点数有一个值不可以比较相等性，那它自然只能实现 PartialEq 而不能实现 Eq 了，
    // 以此类推，如果我们的类型也有这种特殊要求，那也应该这么作。

    // 事实上，还有一对与 `Eq/PartialEq` 非常类似的特征，它们可以用于 `<`、`<=`、`>` 和 `>=` 比较运算符。
    // Ord 意味着一个类型的所有值都可以进行排序，而 PartialOrd 则不然。
}

fn study_super_trait() {
    println!("----------------父 trait----------------");
    // Rust 没有“继承”，但是您可以将一个 trait 定义为另一个 trait 的超集（即父 trait）。

    trait Person {
        fn name(&self) -> String;
    }

    // Person 是 Student 的父 trait。
    // 实现 Student 需要你也 impl 了 Person。
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    // CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
    // 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
    struct CduCsStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }
    impl Person for CduCsStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl Student for CduCsStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    impl Programmer for CduCsStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    impl CompSciStudent for CduCsStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    let student = CduCsStudent {
        name: "doraemon".to_string(),
        university: "CDU".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "doraemonkeys".to_string(),
    };
    println!("{}", comp_sci_student_greeting(&student));
}

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn study_trait_adapter() {
    println!("----------------在外部类型上实现外部特征(newtype)----------------");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    // 我们这里用Wrapper来包装了Vec<String>，然后实现了Display特征，这样就可以打印出来了。
    // 但是这样之后，任何数组上的方法，你都无法直接调用，需要先用 `self.0` 取出数组，然后再进行调用。
    // Rust 提供了一个特征叫 Deref，实现该特征后，可以自动做一层类似类型转换的操作，
    // 可以将 `Wrapper` 变成 `Vec<String>` 来使用(详情见智能指针)。
    println!("w[0] = {}", w.0[0]);
}

trait OutlinePrint: std::fmt::Display {
    // 只有实现了 Display 特征的类型才能实现 OutlinePrint 特征。
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// 使用默认实现
impl<T> OutlinePrint for Pair<T> where T: std::fmt::Display {}

fn study_trait_constraint_in_trait_def() {
    println!("----------------特征定义中的特征约束----------------");
    let p = Pair::new(1, 2);
    p.outline_print();
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn study_same_name_method() {
    println!("----------------调用不同特征的同名方法----------------");
    // 不同特征拥有同名的方法是很正常的事情，你没有任何办法阻止这一点；
    let person = Human;
    // 为了能够调用两个特征的方法，需要使用显式调用的语法：
    Pilot::fly(&person);
    Wizard::fly(&person);
    // 当调用 `Human` 实例的 `fly` 时，编译器默认调用该类型中定义的方法：
    person.fly();

    // 完全限定语法
    // 这个时候问题又来了，如果方法没有 `self` 参数呢？此时，就需要使用完全限定语法。
    // 完全限定语法是调用函数最为明确的方式：
    println!("----------------完全限定语法----------------");
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);

    // 没有 `self` 的例子
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

fn study_associated_type() {
    println!("----------------关联类型(trait中声明)----------------");
    // 关联类型是在特征定义的语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型。
    let mut counter = Counter { count: 0 };
    while let Some(i) = counter.next() {
        print!("{} ", i);
    }
    println!();
}

// 这是标准库中的迭代器特征 `Iterator`，它有一个 `Item` 关联类型，用于替代遍历的值的类型。
pub trait Iterator2 {
    type Item;

    // 之前提到过，`Self` 用来指代当前调用者的具体类型，
    // 那么 `Self::Item` 就用来指代该类型实现中定义的 `Item` 类型
    fn next(&mut self) -> Option<Self::Item>;

    // 为何不用泛型?
    // 答案其实很简单，为了代码的可读性，当你使用了泛型后，你需要在所有地方都写 `Iterator<Item>`，
    // 而使用了关联类型，你只需要写 `Iterator`，当类型定义复杂时，这种写法可以极大的增加可读性
}

pub trait Iterator3<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    count: u32,
}

impl Iterator2 for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) -> String {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw()
}

fn draw2(x: &dyn Draw) -> String {
    // `draw1` 函数的参数是 `Box<dyn Draw>` 形式的特征对象，该特征对象是通过 `Box::new(x)` 的方式创建的。
    // draw2的参数是 `&dyn Draw` 形式的特征对象，该特征对象是通过 `&x` 的方式创建的
    x.draw()
}
pub struct Screen {
    // 存储了一个动态数组，里面元素的类型是 `Draw` 特征对象：`Box<dyn Draw>`，
    // 任何实现了 `Draw` 特征的类型，都可以存放其中。
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("screen run: {}", component.draw());
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        // 绘制按钮的代码
        format!(
            "Button: width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        )
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        // 绘制SelectBox的代码
        format!(
            "SelectBox: width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        )
    }
}

fn study_trait_object() {
    println!("----------------特征对象----------------");
    // 现在在做一款游戏，需要将多个对象渲染在屏幕上，这些对象属于不同的类型，存储在列表中，
    // 渲染的时候，需要循环该列表并顺序渲染每个对象，在 Rust 中该怎么实现？
    // 只要组件实现了 `Draw` 特征，就可以调用 `draw` 方法来进行渲染。
    // Rust 引入了一个概念 —— 特征对象。

    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    println!("draw1 {:?}", draw1(Box::new(x)));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    println!("draw1 {:?}", draw1(Box::new(y)));
    println!("draw2 {:?}", draw2(&x));
    println!("draw2 {:?}", draw2(&y));

    println!("----------------特征对象的动态分发(鸭子类型)----------------");
    // 泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，
    // 这种方式是静态分发(static dispatch)， 因为是在编译期完成的，对于运行期性能完全没有任何影响。
    // 与静态分发相对应的是动态分发(dynamic dispatch)，在这种情况下，直到运行时，才能确定需要调用什么方法。
    // 之前代码中的关键字 dyn 正是在强调这一“动态”的特点。
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 特征对象的限制
    // 不是所有特征都能拥有特征对象，只有对象安全的特征才行。
    // 当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    // - 方法的返回类型不能是 `Self`
    // - 方法没有任何泛型参数
    // 用Go来想，就是接口中的方法返回了实现接口的具体类型，这是不可能的，因为接口是一个抽象类型，不知道具体的实现类型。
    // 对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。
    // 而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。
    // 标准库中的 `Clone` 特征就不符合对象安全的要求，因为它的其中一个方法，返回了 `Self` 类型。
    println!("----------------特征对象的限制----------------");
}

struct Pair<T> {
    first: T,
    second: T,
}
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

pub trait CmpDisplay {
    fn cmp_display(&self);
}

// 为 Pair<T> 实现 Display trait
impl<T> Display for Pair<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

impl<T> CmpDisplay for Pair<T>
where
    T: std::fmt::Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is x = {}", self.first);
        } else {
            println!("The largest member is y = {}", self.second);
        }
    }
}

pub trait Foo {
    fn foo(&self);
}

impl<T> Foo for T
where
    T: CmpDisplay,
{
    fn foo(&self) {
        println!("foo");
        self.cmp_display();
    }
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不会卡"),
    }
}

fn returns_summarizable2(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Post {
            title: String::from("Penguins win the Stanley Cup Championship!"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Weibo {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        })
    }
}

// 我们现在有文章 `Post` 和微博 `Weibo` 两种内容载体，而我们想对相应的内容进行总结，
// 总结这个行为就是共享的，因此可以用特征来定义。
// 特征类似于接口，但是有一些不同：
// 1. 特征中的方法有默认实现，而接口中的方法没有默认实现。
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Summary 的 默认实现")
    }
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

// 实现trait 方法1
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博：{}", self.username, self.content)
    }
}
// 实现trait 方法2
impl Summary for Weibo {
    fn summarize(&self) -> String {
        // 调用自身的方法
        self.summarize()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}
// 实现trait 方法3(使用默认实现)
impl Summary for Tweet {}

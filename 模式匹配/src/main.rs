fn main() {
    study_match();
    // 综合的例子
    study_example();
    // if let
    study_if_let();
    // while let
    study_while_let();
    // matches!宏
    study_matches();
    // 变量覆盖
    study_variable_cover();
    // 匹配Option
    study_option();
    // 更多模式匹配
    study_more_pattern();
    // @绑定
    study_at_binding();
}

fn study_at_binding() {
    println!("---------------------@绑定---------------------");
    // `@`（读作 at）运算符允许为一个字段绑定另外一个变量,格式为：`field @ pattern`
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // 在某个范围内，希望能将其值绑定到 `id_variable` 变量中以便此分支中相关的代码可以使用它。
    // 我们可以将 `id_variable` 命名为 `id`，与字段同名，不过出于示例的目的这里选择了不同的名称。
    // 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 `@` 来绑定到一个新的变量上，实现想要的功能。
    match msg {
        Message::Hello {
            // 通过在 `3..=7` 之前指定 `id_variable @`，
            // 我们捕获了任何匹配此范围的值并同时将该值绑定到变量 `id_variable` 上。
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    // @前绑定后解构
    println!("---------------------@前绑定后解构---------------------");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("p: {:?}", p);

    let point = Point { x: 10, y: 5 };
    // 若模式匹配成功，则将 `point` 绑定到 `pp` 变量上
    if let pp @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, pp);
    } else {
        println!("x was not 10 :( ");
    }
    // @新特性
    println!("---------------------@新特性---------------------");
    match 1 {
        num @ (1 | 2) => {
            println!("num is {}", num);
        }
        _ => {}
    }
}

fn study_more_pattern() {
    println!("---------------------更多模式匹配---------------------");
    // 其实let语句也是一种模式匹配，变量名也是一种模式，只不过它比较朴素很不起眼罢了。
    let x: i32 = 5; // 若5匹配i32类型，则x为5
    println!("x is {}", x);
    // 将一个元组与模式进行匹配，把 `1, 2, 3` 分别绑定到 `x, y, z` 上。
    // 模式匹配要求两边的类型必须相同
    let (x, y, z) = (1, 2, 3);
    println!("x is {}, y is {}, z is {}", x, y, z);
    // 解构结构体
    println!("---------------------解构结构体---------------------");
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    // 这个例子展示了模式中的变量名不必与结构体中的字段名一致。
    // 不过通常希望变量名与字段名一致以便于理解变量来自于哪些字段。
    let Point { x: a, y: b } = p;
    println!("a is {}, b is {}", a, b);

    // 因为变量名匹配字段名是常见的，同时因为 let Point { x: x, y: y } = p;`中 `x` 和 `y` 重复了，
    // 所以对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称。
    let Point { x, y } = p;
    println!("x is {}, y is {}", x, y);

    // 也可以使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量。
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    println!("---------------------解构枚举---------------------");
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::_Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::_Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::_Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    // 解构嵌套的结构体和枚举
    study_destructuring_nested_structures_and_enums();
    // 解构数组
    println!("---------------------解构数组---------------------");
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    println!("x is {}, y is {}", x, y);

    let arr: &[u16] = &[114, 514];
    println!("arr is {:?}", arr); //arr is [114, 514]

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    // 匹配守卫提供的额外条件
    // 匹配守卫（match guard）是一个位于 `match` 分支模式之后的额外 `if` 条件，
    // 它能为分支模式提供更进一步的匹配条件。
    println!("---------------------匹配守卫提供的额外条件---------------------");
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    assert!(matches!(num, Some(x) if x < 5));
}

fn study_destructuring_nested_structures_and_enums() {
    println!("---------------------解构嵌套的结构体和枚举---------------------");
    enum Color {
        _Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::_Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => {
            println!("other");
        }
    }
}

fn study_while_let() {
    println!("---------------------while let---------------------");
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut stack2 = stack.clone();

    // stack.pop从数组尾部弹出元素直到返回None(Some(top)不匹配)
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 常规写法
    while stack2.len() > 0 {
        let top = stack2.pop();
        println!("{}", top.unwrap());
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn study_option() {
    println!("---------------------匹配Option---------------------");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five is {:?}, six is {:?}, none is {:?}", five, six, none);
}

enum Direction {
    _East,
    _West,
    _North,
    South,
}

#[derive(Debug)]
enum Coin {
    Penny,
    _Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents2(coin: &Coin2) -> u8 {
    match coin {
        Coin2::_Penny => 1,
        Coin2::_Nickel => 5,
        Coin2::_Dime => 10,
        // 匹配命名变量
        Coin2::Quarter(state) => {
            // 阿拉斯加州标记的 25 分硬币
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum IpAddr {
    _Ipv4,
    Ipv6,
}

#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    _Penny,
    _Nickel,
    _Dime,
    // `Coin::Quarter` 成员还存放了一个值：
    // 美国的某个州（因为在 1999 年到 2008 年间，美国在 25 美分(Quarter)硬币的背后为 50 个州印刷了不同的标记
    Quarter(UsState), // 25美分硬币(美国某个州)
}

fn study_match() {
    println!("---------------------match 匹配---------------------");
    let dire = Direction::South;
    // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同.
    // match 的匹配分支必须是完备的，也就是说必须覆盖所有的情况，否则编译器会报错。
    match dire {
        Direction::_East => println!("East"),
        // multiple
        Direction::_North | Direction::South => {
            println!("South or North");
        }
        // default
        _ => println!("West"),
    };

    let penny_coin = Coin::Penny;
    let quarter_coin = Coin::Quarter;
    let mut value = value_in_cents(&penny_coin);
    println!("{:?} coin value is {}", penny_coin, value);
    value = value_in_cents(&quarter_coin);
    println!("{:?} coin value is {}", Coin::Dime, value);

    // `match` 本身也是一个表达式，因此可以用它来赋值：
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::_Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("ip_str is {}", ip_str);

    // 模式匹配的另外一个重要功能是从模式中取出绑定的值
    let coin = Coin2::Quarter(UsState::Alaska);
    value = value_in_cents2(&coin);
    println!("coin value is {}", value);
    //match 不仅可以匹配枚举类型，还可以匹配一些字面值。
    // 与枚举类型比较的本质也是在比较它们的值。
    println!("---------------------match 匹配基本类型---------------------");
    let foo = 'f';
    match foo {
        // .. 表示一个包含左边但不包含右边的半开区间
        // ..= 表示一个包含左边和右边的闭区间
        'A'..='Z' | 'a'..='z' => println!("letter"),
        _ => println!("not letter"),
    }
    let foo = 99;
    match foo {
        1..=10 => println!("1-10"),
        11..=20 => println!("11-20"),
        _ => println!("other"),
    }
}

fn study_variable_cover() {
    println!("---------------------变量覆盖---------------------");
    // 无论是 `match` 还是 `if let`，他们都可以在模式匹配时覆盖掉老的值，绑定新的值:
    let age = Some(30);
    println!("在匹配前，age是{:?}", age); //Some(30),类型是Option<i32>
    if let Some(age) = age {
        println!("匹配出来的age是{}", age); //30,类型是i32
    }
    println!("在匹配后，age是{:?}", age); //Some(30),类型是Option<i32>
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn study_matches() {
    println!("---------------------matches!宏---------------------");
    // Rust 标准库中提供了一个非常实用的宏：`matches!`，
    // 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 `true` or `false`。
    let coin = Coin2::Quarter(UsState::Alaska);
    let value = matches!(coin, Coin2::Quarter(_));
    println!("coin value is {}", value);

    // 例如，有一个动态数组，里面存有以下枚举：
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // 现在如果想对 `v` 进行过滤，只保留类型是 `MyEnum::Foo` 的元素
    let v2: Vec<MyEnum> = v.into_iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("v2 is {:?}", v2);
    // 更多的例子
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    // Some(x)是pattern匹配模式
    // if x > 2 表示guard的匹配守卫(match guard)
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2)); // Some(x) if x > 2 表示 Some(x) 并且 x > 2
    assert!(matches!(bar, Some(x) if x > 2 && x < 5));
    assert!(matches!(bar, Some(4)));
}

fn study_if_let() {
    println!("---------------------if let匹配---------------------");
    // 在某些场景下，我们其实只关心某一个值是否存在，此时 match 就显得过于啰嗦。
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 完全可以用 `if let` 的方式来实现：
    if let Some(3) = some_u8_value {
        //让 Some(3) 绑定到 some_u8_value 上,如果匹配成功，就执行代码块
        println!("three");
    }
    if let Some(x) = some_u8_value {
        //如果是 Some(x)，就把 some_u8_value 的值赋值给 x
        println!("x is {}", x);
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn study_example() {
    println!("---------------------综合的例子---------------------");
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

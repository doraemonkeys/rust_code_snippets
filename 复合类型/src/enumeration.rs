#[derive(Debug)]
enum PokerSuit {
    _Clubs,    // 0
    _Spades,   // 1
    _Diamonds, // 2
    _Hearts,   // 3
}
pub fn study_enumeration() {
    println!("-----------------枚举-----------------");
    let heart = PokerSuit::_Hearts;
    let diamond = PokerSuit::_Diamonds;
    print_suit(heart);
    print_suit(diamond);
    // 将数据信息关联到枚举成员上
    println!("-----------------将数据信息关联到枚举成员上-----------------");
    let c1 = PokerCard::_Spades(5); //花色为黑桃，点数为5
    let c2 = PokerCard::_Diamonds(13); //花色为方块，点数为13
    println!("{:?}", c1);
    println!("{:?}", c2);
    // 任何类型的数据都可以放入枚举成员中
    println!("-----------------任何类型的数据都可以放入枚举成员中-----------------");
    let m1 = Message::_Quit;
    let m2 = Message::_Move { _x: 1, _y: 2 };
    println!("{:?}", m1);
    println!("{:?}", m2);
    // Option 枚举用于处理空值
    println!("-----------------Option 枚举用于处理空值-----------------");
    // `Option<T>` 枚举是如此有用以至于它被包含在了 prelude 之中，
    // prelude 属于 Rust 标准库，Rust 会将最常用的类型、函数等提前引入其中，省得我。们再手动引入，
    // 你不需要将其显式引入作用域。另外，它的成员 `Some` 和 `None` 也是如此
    let some_number = Some(5);
    let some_string = Some("a string");
    // 如果使用 `None` 而不是 `Some`，需要告诉 Rust `Option<T>` 是什么类型的，
    // 因为编译器只通过 `None` 值无法推断出 `Some` 成员保存的值的类型。
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
    // 当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值，
    // 我们可以放心使用而无需做空值检查。只有当使用 `Option<i8>`（或者任何用到的类型）的时候
    // 才需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。

    // 用 `match` 表达式处理枚举
    println!("-----------------用 `match` 表达式处理枚举-----------------");
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other"),
    }
    // 用 `if let` 表达式处理枚举
    println!("-----------------用 `if let` 表达式处理枚举-----------------");
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

#[derive(Debug)]
enum PokerCard {
    _Clubs(u8),
    _Spades(u8),
    _Diamonds(u8),
    _Hearts(u8),
}

#[derive(Debug)]
enum Message {
    // Rust 会按照枚举中占用内存最大的那个成员进行内存对齐，这意味着可能造成内存上的浪费。
    _Quit,
    _Move { _x: i32, _y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}
/*
   - `Quit` 没有任何关联数据
   - `Move` 包含一个匿名结构体
   - `Write` 包含一个 `String` 字符串
   - `ChangeColor` 包含三个 `i32`

   如果用结构体的方式来定义这些消息：
   struct QuitMessage; // 单元结构体
   struct MoveMessage {
       x: i32,
       y: i32,
   }
   struct WriteMessage(String); // 元组结构体
   struct ChangeColorMessage(i32, i32, i32); // 元组结构体

   由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，
   例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，就可以接收不同的消息，
   但是用结构体，该函数无法接受 4 个不同的结构体作为参数。
*/

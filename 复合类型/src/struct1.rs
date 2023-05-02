struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// implement `std::fmt::Display` for `User`
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}

fn build_user(email: String, username: String) -> User {
    // 简化结构体初始化
    // 当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn study_struct() {
    println!("-----------------结构体-----------------");
    println!("-----------------定义结构体-----------------");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1);
    // 简化结构体初始化
    let user2 = build_user(
        "someone@example.com".to_string(),
        "someusername123".to_string(),
    );
    println!("{}", user2);
    // 根据已有的结构体实例，创建新的结构体实例
    println!("-----------------根据已有的结构体实例，创建新的结构体实例-----------------");
    // 从 user1 实例中创建一个新的 user3 实例，只是将 email 字段的值改为了另一个值
    let user3 = User {
        email: String::from("xxxeample.com"),
        ..user1
    };
    println!("{}", user3);
    // 注意在上面的`username` 字段发生了所有权转移，作为结果，`user1.username` 不再有效
    //println!("{}", user1); // error: value borrowed here after move
    //虽然user1.username不再有效，但是其他字段还是有效的
    println!("user1.email = {}", user1.email);

    // 元组结构体(Tuple Struct)
    println!("-----------------元组结构体(Tuple Struct)-----------------");
    // 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体。
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // 单元结构体(Unit-like Struct)
    println!("-----------------单元结构体(Unit-like Struct)-----------------");
    // 还记得之前讲过的基本没啥用的单元类型吧？单元结构体就跟它很像，没有任何字段和属性，但是好在，它还挺有用。
    // 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 `单元结构体`。
    let subject = AlwaysEqual;
    println!("{}", subject);

    // 使用 `#[derive(Debug)\]` 来打印结构体的信息(使用 derive 派生实现)
    println!("-----------------使用 `#[derive(Debug)\\]` 来打印结构体的信息-----------------");
    let rect1 = Rectangle {
        _width: 30,
        _height: 50,
    };
    println!("rect1 is {:?}", rect1);
    // 当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 `{:#?}` 来替代 `{:?}`。
    println!("rect1 is {:#?}", rect1);

    // {} -> std::fmt::Display
    // {:?} -> std::fmt::Debug
    // {:#?} -> std::fmt::Debug
    // {:b} -> std::fmt::Binary
    // {:x} -> std::fmt::LowerHex
    // {:X} -> std::fmt::UpperHex

    println!("-----------------使用 `dbg!` 来打印结构体的信息-----------------");
    // 还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏，
    // 它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，
    // 当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！
    // dbg!` 输出到标准错误输出 stderr。
    dbg!(rect1);
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

struct AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl std::fmt::Display for AlwaysEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AlwaysEqual")
    }
}

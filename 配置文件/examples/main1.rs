use serde::{Deserialize, Serialize};

// https://serde.rs/examples.html

#[derive(Serialize, Deserialize, Debug)]
struct Foo<'a> {
    a: u8,
    b: i32,
    s: String,
    s1: &'a str,
    f: f64,
    // 忽略字段
    #[serde(skip)]
    ii: String,

    foo: Bar,

    #[serde(flatten)]
    // 效果1： "A": 7
    // 效果2： "my_enum": { "A": 7 }
    my_enum: MyEnum,
    my_enum2: MyEnum,
    my_enum3: MyEnum,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bar {
    bar_a: u8,
    bar_b: i32,
    bar_s: String,
    bar_f: f64,
}

#[derive(Serialize, Deserialize, Debug)]
enum MyEnum {
    A(u8),
    #[serde(rename = "BBB")]
    B(Vec<String>),
    C,
    #[serde(alias = "DDD")]
    // 添加别名，反序列化时，可以使用别名
    D(u128),
}

fn main() {
    let foo = Foo {
        a: 1,
        b: 2,
        s: "hello".to_string(),
        s1: "world",
        f: 3.14,
        ii: "ignore".to_string(),
        foo: Bar {
            bar_a: 4,
            bar_b: 5,
            bar_s: "bar".to_string(),
            bar_f: 6.28,
        },
        my_enum: MyEnum::A(7),
        my_enum2: MyEnum::C,
        my_enum3: MyEnum::D(8),
    };
    println!("field ii:{:?}", foo.ii);

    let json = serde_json::to_string(&foo).unwrap();
    println!("序列化：{}", json);

    let foo1: Foo = serde_json::from_str(&json).unwrap();
    println!("反序列化：{:?}", foo1);

    let my_enum = MyEnum::B(vec!["hello".to_string(), "world".to_string()]);
    let json = serde_json::to_string(&my_enum).unwrap();
    println!("序列化：{}", json);

    let my_enum1: MyEnum = serde_json::from_str(&json).unwrap();
    println!("反序列化：{:?}", my_enum1);

    let my_enum = MyEnum::D(8);
    let json = serde_json::to_string(&my_enum).unwrap();
    println!("序列化：{}", json);

    let json_str = r#"{"DDD":8}"#;
    let my_enum1: MyEnum = serde_json::from_str(&json_str).unwrap();
    println!("反序列化：{:?}", my_enum1);
}

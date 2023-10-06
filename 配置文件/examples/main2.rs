use serde::ser::Error;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum Gender {
    Female,                                       // 雌性
    Male,                                         // 雄性
    Neither,                                      // 无性
    Both,                                         // 中性
    Trans { from: Box<Gender>, to: Box<Gender> }, // 跨性别
}

//  自定义序列化实现。
// https://www.luogu.com.cn/blog/HoshinoTented/serde-in-rust
impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let str = match self {
            // 根据枚举值来决定字符串内容
            Gender::Female => "female",
            Gender::Male => "male",
            Gender::Neither => "none",
            Gender::Both => "both",
            Gender::Trans { from, to } => {
                let pair = (from.as_ref(), to.as_ref());

                match pair {
                    (Gender::Female, Gender::Male) => "ftm",
                    (Gender::Male, Gender::Female) => "mtf",

                    _ => Err(<S as Serializer>::Error::custom(
                        "Trans variant only supports mtf and ftm",
                    ))?, // 为了简单这里只支持双性的跨性别
                }
            }
        };

        serializer.serialize_str(str) // 序列化一个字符串
    }
}

#[derive(Debug, Serialize)] // 注意删去 Deserialize，因为我们还没为 Gender 提供反序列化
struct Person {
    name: String,
    age: i32,
    gender: Gender,
}

fn main() {
    let hoshino = Person {
        name: String::from("hoshino"),
        age: 4,
        gender: Gender::Trans {
            from: Gender::Male.into(),
            to: Gender::Female.into(),
        },
    };

    let json = serde_json::to_string(&hoshino).unwrap(); // 使用 serde_json 库来进行 JSON 的序列化
    println!("{}", json);
}

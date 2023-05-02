pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(left: i32) -> i32 {
    left + 2
}

// `tests` 就是一个测试模块，`it_works` 则是我们的主角：测试函数。
// 测试函数需要使用 `test` 属性进行标注。
// 关于属性( `attribute` )，我们在之前的章节已经见过类似的 `derive`，
// 使用它可以派生自动实现的 `Debug` 、`Copy` 等特征，
// 同样的，使用 `test` 属性，我们也可以获取 Rust 提供的测试特性。
// 当然，在测试模块 `tests` 中，还可以定义非测试函数， 这些函数可以用于设置环境或执行一些通用操作：
// 例如为部分测试函数提供某个通用的功能，这种功能就可以抽象为一个非测试函数。

#[cfg(test)]
mod tests {
    use super::*; // 使用 super::* 导入父模块中的所有内容，这样就可以直接使用 add 函数了

    // `#[test]` 属性标注了 `it_works` 函数，这样 Rust 就知道这是一个测试函数。
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        // 默认情况下，如果测试通过，那写入标准输出的内容是不会显示在测试结果中的。
        // 除非：$ cargo test -- --show-output
        println!("测试 greeting_contains_name");
        let result = greeting("Sunface");
        let target = "孙飞";
        // 使用 `assert!` 自定义失败信息
        assert!(
            result.contains(target),
            "你的问候中并没有包含目标姓名 {} ，你的问候是 `{}`",
            target,
            result
        );
    }
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    // 测试 panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 虽然 `panic` 被成功测试到，但是如果代码发生的 `panic` 和我们预期的 `panic` 不符合呢？
    // 鉴于此，我们可以使用可选的参数 `expected` 来说明预期的 `panic` 长啥样。
    // `expected` 的字符串和实际 `panic` 的字符串可以不同，前者只需要是后者的字符串前缀即可。
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }
}

#[cfg(test)]
mod tests4 {
    use super::*; // 使用 super::* 导入父模块中的所有内容

    // 在测试中使用 `?` 操作符进行链式调用
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            // 如果返回 `Err`，测试就会失败
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[cfg(test)]
mod tests5 {
    use super::*;
    // 引入只在开发测试场景使用的外部依赖
    use pretty_assertions::assert_eq; // 该包仅能用于测试，提供彩色字体的结果对比。

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 7);
    }
}

fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    println!("{}", penguin_data);

    // lines() 方法返回一个迭代器，每次迭代返回一行
    let records = penguin_data.lines();

    // enumerate() 方法返回一个迭代器，每次迭代返回一个元组，元组的第一个元素是索引，第二个元素是迭代器中的值
    for (i, record) in records.enumerate() {
        // 跳过第一行,trim() 方法用来去掉字符串首尾的空白字符
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        // cfg!() 是一个宏，用来判断当前的编译配置，如果是 debug 模式，则输出调试信息
        if cfg!(debug_assertions) {
            // 输出到标准错误输出, {:?} 是一个格式化参数，用来输出 fields 变量的值
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

// 上面代码中，值得注意的 Rust 特性有：

// - 控制流：`for` 和 `continue` 连在一起使用，实现循环控制。
// - 方法语法：由于 Rust 没有继承，因此 Rust 不是传统意义上的面向对象语言，
//   但是它却从 `OO` 语言那里偷师了方法的使用 `record.trim()`，`record.split(',')` 等。
// - 高阶函数编程：函数可以作为参数也能作为返回值，例如 `.map(|field| field.trim())`，
//   这里 `map` 方法中使用闭包函数作为参数，也可以称呼为 `匿名函数`、`lambda 函数`。
// - 类型标注：`if let Ok(length) = fields[1].parse::<f32>()`，通过 `::<f32>` 的使用，
//   告诉编译器 `length` 是一个 `f32` 类型的浮点数。这种类型标注不是很常用，
//   但是在编译器无法推断出你的数据类型时，就很有用了。
// - 条件编译：`if cfg!(debug_assertions)`，说明紧跟其后的输出（打印）只在 `debug` 模式下生效。
// - 隐式返回：Rust 提供了 `return` 关键字用于函数返回，但是在很多时候，我们可以省略它。
//   因为 Rust 是 [**基于表达式的语言**](https://course.rs/basic/base-type/statement-expression.html)。

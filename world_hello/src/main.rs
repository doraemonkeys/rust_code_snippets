fn greet_world() {
    // Rust 原生支持 UTF-8 编码的字符串
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    // let english = "World, hello";
    let english = "World, hello";

    // 和其它语言不同，Rust 的集合类型不能直接进行循环，需要变成迭代器（这里是通过 `.iter()` 方法），才能用于迭代循环
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        // !是一个格式化宏，用于格式化字符串
        // 对于 `println` 来说，我们没有使用其它语言惯用的 `%s`、`%d` 来做输出占位符，而是使用 `{}`，
        // 因为 Rust 在底层帮我们做了大量工作，会自动识别输出数据的类型
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}

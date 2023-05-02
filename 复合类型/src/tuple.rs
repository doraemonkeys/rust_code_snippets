pub fn study_tuple() {
    println!("-----------------元组-----------------");
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    dbg!(tup);
    // 用模式匹配解构元组
    println!("-----------------用模式匹配解构元组-----------------");
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    // 用 . 来访问元组
    println!("-----------------用 . 来访问元组-----------------");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one
    );
    // 函数返回多个值可以使用元组
    println!("-----------------函数返回多个值可以使用元组-----------------");
    let (x, y) = return_tuple();
    println!("x: {}, y: {}", x, y);
}

fn return_tuple() -> (i32, i32) {
    (1, 2)
}

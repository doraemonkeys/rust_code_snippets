pub fn study_array() {
    println!("-----------------数组-----------------");
    // 在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 `array`，
    // 第二种是可动态增长的但是有性能损耗的 `Vector` 类型。
    // 这两个数组的关系跟 `&str` 与 `String` 的关系很像，前者是长度固定的字符串切片，后者是可动态增长的字符串。
    // 其实，在 Rust 中无论是 `String` 还是 `Vector`，它们都是 Rust 的高级类型：集合类型。
    // 数组 `array` 是存储在栈上，性能也会非常优秀。与此对应，动态数组 `Vector` 是存储在堆上。
    println!("-----------------创建数组-----------------");
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the first element of a is: {}", a[0]);
    if a == b {
        // 数组默认实现了 PartialEq trait，因此可以直接使用 == 进行比较
        println!("a == b");
    }
    let c = [0; 5]; // 创建一个长度为 5 的数组，每个元素都是 0
    println!("c = {:?}", c);
    println!("-----------------数组的遍历-----------------");
    // 1. for 循环
    for i in 0..a.len() {
        print!("{} ", a[i]);
    }
    println!();
    // 2. for_each
    a.iter().for_each(|x| print!("{} ", x));
    println!();
    // 3. for in
    for x in a.iter() {
        print!("{} ", x);
    }
    println!();
    // 4. for in &a
    for x in &a {
        print!("{} ", x);
    }
    println!();
    // 数组元素为非基础类型
    println!("-----------------数组元素为非基础类型-----------------");
    // 前面几个例子都是Rust的基本类型，而基本类型在Rust中赋值是以Copy的形式，这时候你就懂了吧，
    // let array=[3;5]底层就是不断的Copy出来的。而如果数组元素是非基础类型，那么就会出现所有权的问题。
    // let array = [String::from("rust is good!"); 8]; // error
    // 正确的写法
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("array = {:?}", array);

    // 数组切片
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用。
    println!("-----------------数组切片-----------------");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[2, 3]);
    // 也可以使用 .. 运算符来创建一个包含数组中所有元素的切片
    let slice: &[i32] = &a[..];
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[1, 2, 3, 4, 5]);

    // 从字符串字面量创建切片
    println!("-----------------从字符串字面量创建切片-----------------");
    let a: &[u8] = b"hello"; //b means byte
    println!("a = {:?}", a);
    assert_eq!(a, &[b'h', b'e', b'l', b'l', b'o']);

    // 从vector创建数组切片
    println!("-----------------从vector创建切片-----------------");
    let v = vec![111, 222, 333, 444, 555];
    let slice = &v[1..3];
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[222, 333]);

    // 从 Vector 创建数组切片
    let v = vec![111, 222, 333, 444, 555];
    let slice = v.as_slice();
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[111, 222, 333, 444, 555]);

    // 切片可以是对数组的可变引用
    println!("-----------------切片可以是对数组的可变引用-----------------");
    let mut b: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &mut [i32] = &mut b[1..3];
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[2, 3]);
    slice[0] = 10;
    slice[1] = 20;
    println!("slice = {:?}", slice);
    println!("b = {:?}", b);

    // &[u8] 和 &[u8; N] 的区别
    println!("-----------------&[u8] 和 &[u8; N] 的区别-----------------");
    // &[u8] 是切片的引用(平常所说的切片就是指切片的引用)，包含一个长度信息。
    // &[u8; N] 是一个 u8数组 的引用，不包含长度信息。
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let _slice: &[u8] = &a[..];
    println!("sizeof(&[u8]) = {}", std::mem::size_of::<&[u8]>()); // 16

    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let _slice: &[u8; 5] = &a;
    println!("sizeof(&[u8; 5]) = {}", std::mem::size_of::<&[u8; 5]>()); // 8
}

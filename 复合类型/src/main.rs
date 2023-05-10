mod array;
mod collection;
mod enumeration;
mod struct1;
mod tuple;
fn main() {
    // 切片
    study_slice();
    // 字符串
    study_string();
    //操作字符串
    study_string_operation();
    // 字符串转义
    study_string_escape();
    // 元组
    tuple::study_tuple();
    // 结构体
    struct1::study_struct();
    // 枚举
    enumeration::study_enumeration();
    // 数组
    array::study_array();
    // 集合类型
    collection::study_collection();
}

fn study_string_escape() {
    println!("-----------------字符串转义-----------------");
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );
    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
      can span multiple lines.
      The linebreak and indentation here ->\
      <- can be escaped too!";
    println!("{}", long_string);
    // 原始字符串
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes); // And then I said: "There is no escape!"

    // 如果还是有歧义，可以继续增加，没有限制(r##"..."##,前后#个数必须相同)
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter); // A string with "# in it. And even "##!
}

fn study_string_operation() {
    println!("-----------------操作字符串-----------------");
    println!("-----------------追加(Push)-----------------");
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    println!("{}", s);

    println!("-----------------插入(Insert)-----------------");
    let mut s = String::from("hello rust");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like it");
    println!("插入字符串 insert_str() -> {}", s);

    println!("-----------------替换(Replace)-----------------");
    // replace适用于 `String` 和 `&str` 类型，该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    // dbg! Prints and returns the value of a given expression for quick and dirty debugging.
    dbg!(new_string_replace);
    // replacen方法与replace方法类似，不同的是它可以指定替换的次数。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    // replace_range方法可以指定替换的范围，该方法是直接操作原来的字符串，不会返回新的字符串。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    println!("-----------------删除(Delete)-----------------");
    // 与字符串删除相关的方法有 4 个，他们分别是 `pop()`，`remove()`，`truncate()`，`clear()`。
    // 这四个方法仅适用于 `String` 类型。
    // pop方法用于删除字符串的最后一个字符。
    // remove方法删除并返回字符串中指定位置的字符(如果参数所给的位置不是合法的字符边界，则会发生错误)。
    // truncate方法用于删除字符串中从指定位置开始到结尾的全部字符。
    // clear方法用于清空字符串。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    println!("-----------------分割(Split)-----------------");
    // split方法用于将字符串按照指定的分隔符进行分割，返回一个迭代器。
    let string_split = "I like rust 我喜欢 rust";
    for word in string_split.split(' ') {
        println!("{}", word);
    }
    let mut it = string_split.split(' ');
    println!("迭代器：{:?}", it.next()); // 迭代器：Some("I")
    println!("迭代器：{:?}", it.next()); // 迭代器：Some("like")

    // split_whitespace方法用于将字符串按照空白字符进行分割，返回一个迭代器。
    let string_split_whitespace = "I like rust";
    for word in string_split_whitespace.split_whitespace() {
        println!("{}", word);
    }
    // split_terminator方法用于将字符串按照指定的分隔符进行分割，返回一个迭代器。
    // 与split方法不同的是，split_terminator方法会将分隔符也包含在结果中。

    println!("-----------------连接(Concatenate)-----------------");
    println!("-----------------使用 + 或者 += 连接字符串-----------------");
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // +必须必须传递切片引用类型，&string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    // 其实当调用 `+` 的操作符时，相当于调用了 string_append 标准库中的 `add()` 方法
    // let result = string_append.add(&string_rust);
    let mut result = result + "!";
    result += "!!!";
    // fn add(mut self, other: &str) -> String
    // string_append调用Add方法后，string_append的所有权已经转移了，所以不能再使用string_append
    // println!("连接字符串 + -> {}", string_append); // error: value borrowed here after move
    println!("连接字符串 + -> {}", result);
    println!("-----------------使用 format! 连接字符串-----------------");
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2); // format! 不会获取任何所有权，类似于C语言的sprintf
    println!("{}", s);
}

fn study_string() {
    println!("-----------------字符串-----------------");
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)。
    // Rust 在语言级别，只有一种字符串类型：str，它通常是以(不可变)引用类型出现 &str，也就是字符串切片。
    // 在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。
    // String 是一个可增长、可改变且具有所有权的 UTF-8 编码字符串类型。

    // String 与 &str 的转换
    println!("-----------------String 与 &str 的转换-----------------");
    // 1. &str -> String
    let s = "hello world";
    let s = s.to_string();
    println!("{}", s);
    let s2 = String::from(s);
    println!("{}", s2);
    // 2. String -> &str
    // 何将 `String` 类型转为 `&str` 类型呢？答案很简单，取引用即可。
    let s = String::from("Tom");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // [u8] -> String
    println!("-----------------[u8] -> String-----------------");
    // 1. 使用 from_utf8() 函数
    let bytes = [104u8, 101, 108, 108, 111];
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{}", s);
    // 2. 使用 from_utf8_lossy() 函数
    let bytes = [104u8, 101, 108, 108, 111, 226, 128, 141];
    let s = String::from_utf8_lossy(&bytes); // 会将无效的字节序列替换为 U+FFFD REPLACEMENT CHARACTER
    println!("{}", s);
    // 3. 使用 from_raw_parts() 函数
    let bytes = [104u8, 101, 108, 108, 111];
    let s = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
    println!("{}", s);

    // String -> [u8]
    println!("-----------------String -> [u8]-----------------");
    // 1. 使用 as_bytes() 方法
    let s = String::from("hello");
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    // 2. 使用 into_bytes() 方法
    let s = String::from("hello");
    let bytes = s.into_bytes();
    println!("{:?}", bytes);
    // 3. 使用 as_ptr() 方法
    let s = String::from("hello");
    let ptr = s.as_ptr();
    let bytes = unsafe { std::slice::from_raw_parts(ptr, s.len()) };
    println!("{:?}", bytes);

    // [u8] -> &str
    println!("-----------------[u8] -> &str-----------------");
    // 1. 使用 from_utf8() 函数
    let bytes = [104u8, 101, 108, 108, 111];
    let s = std::str::from_utf8(&bytes).unwrap();
    println!("{}", s);
    // 2. 使用 from_utf8_unchecked() 函数
    let bytes = [104u8, 101, 108, 108, 111];
    let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
    println!("{}", s);
    // 3. 使用 from_raw_parts() 函数
    let bytes = [104u8, 101, 108, 108, 111];
    let ptr = bytes.as_ptr();
    let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, bytes.len())) };
    println!("{}", s);
}

fn say_hello(s: &str) {
    println!("hello {}", s);
}

fn study_slice() {
    println!("-----------------切片-----------------");
    let s = String::from("hello world");
    // 字符串切片的类型标识是 &str，字符串切片可以是对str(字符串字面量)的引用，也可以是对String的引用。
    let hello: &str = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let hello_world = &s[..];
    println!("{}", hello_world);
    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，
    // 例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃。
    let s = String::from("你好，世界");
    // let hello = &s[0..2];
    // println!("{} {}", hello);
    // 你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法。
    for c in s.chars() {
        println!("{}", c);
    }
    println!("统计Unicode字符个数：{}", s.chars().count());
    // 获取子串
    // 想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情，例如
    // 想要从 holla中国人नमस्ते 这种变长的字符串中取出某一个子串，使用标准库你是做不到的。
    // 你需要在 crates.io 上搜索 utf8 来寻找想要的功能。
    // 可以考虑尝试下这个库：utf8_slice。
    // 字符串切片是非常危险的操作，因为切片的索引是通过字节来进行，
    // 但是字符串又是 UTF-8 编码，因此你无法保证索引的字节刚好落在字符的边界上

    let s = String::from("hello world");
    let word = first_word(&s); // 传入不可变借用，函数返回了不可变借用

    // 当我们已经有了不可变借用时(word)，就无法再拥有不可变的借用。因为 `clear` 需要清空改变 `String`，
    // 因此它需要一个可变借用（利用 VSCode 可以看到该方法的声明是 `pub fn clear(&mut self)` ，
    // 参数是对自身的可变借用 ）
    // s.clear(); // error!
    println!("\"{}\" the first word is: {}", s, word);

    // 取出字符串切片的第一个字符
    // 注意只有数组或数组切片才能使用索引，它们实现了 SliceIndex<[T]> trait。
    assert!(word == "h");
    assert!(word.as_bytes().get(0).unwrap() == &b'h');
    assert!(word.as_bytes()[0] == b'h');
    let byte = s.as_bytes()[0];
    assert!(byte == b'h');
    assert!(word.bytes().nth(0).unwrap() == b'h');
    assert!(word.chars().nth(0).unwrap() == 'h');

    // 切片是对集合的部分引用，因此不仅仅字符串有切片，其它集合类型也有，例如数组。
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}

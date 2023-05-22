pub fn study_collection() {
    println!("-----------------集合类型-----------------");
    // 动态数组
    study_vector();
    // HashMap
    study_hash_map();
}

fn study_hash_map() {
    println!("-----------------HashMap-----------------");
    // HashMap 的 创建
    // 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = std::collections::HashMap::new();

    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    // 从表中读取宝石的数量
    // get 方法返回一个 `Option<&i32>` 类型：当查询不到时，会返回一个 `None`,
    // 如果我们想直接获得值类型，可以使用 `unwrap` 方法，如果查询不到，会 panic。
    // 如果不想 panic，可以使用 `unwrap_or` 方法，当查询不到时，会返回一个默认值。
    let red_gem_count = my_gems.get("红宝石");
    println!("红宝石的数量是：{:?}", red_gem_count);
    let blue_gem_count = my_gems.get("蓝宝石").unwrap();
    println!("蓝宝石的数量是：{:?}", blue_gem_count);
    let fake_gem_count = my_gems.get("河边捡的误以为是宝石的破石头");
    println!("破石头的数量是：{:?}", fake_gem_count);
    if let None = my_gems.get("钻石") {
        println!("钻石不存在");
    }

    // 从表中删除宝石
    my_gems.remove("河边捡的误以为是宝石的破石头");
    let fake_gem_count = my_gems.get("河边捡的误以为是宝石的破石头").unwrap_or(&0);
    println!("破石头的数量是：{:?}", fake_gem_count);

    println!("----------------------------------");
    // 遍历 HashMap
    for (gem_type, count) in &my_gems {
        println!("{}的数量是：{}", gem_type, count);
    }

    // 跟 `Vec` 一样，如果预先知道要存储的 `KV` 对个数，
    // 可以使用 `HashMap::with_capacity(capacity)` 创建指定大小的 `HashMap`，
    // 避免频繁的内存分配和拷贝，提升性能。
    let mut my_gems = std::collections::HashMap::with_capacity(10);
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);
    println!("len = {}", my_gems.len());
    println!("capacity = {}", my_gems.capacity());

    // 使用迭代器和 collect 方法创建 HashMap
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // collect 方法在内部实际上支持生成多种类型的目标集合，因此我们需要通过类型标注 HashMap<_,_> 来告诉编译器：
    // 请帮我们收集为 `HashMap` 集合类型，具体的 `KV` 类型，麻烦编译器您老人家帮我们推导。
    let teams_map: std::collections::HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map);

    // 更新 HashMap
    let mut scores = std::collections::HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    let v = scores.entry("Black");
    match v {
        std::collections::hash_map::Entry::Occupied(o) => {
            println!("Black已经存在，值为：{}", o.get());
        }
        std::collections::hash_map::Entry::Vacant(v) => {
            println!("Black不存在，插入新值");
            v.insert(6);
        }
    }

    // 查询Yellow对应的值，若不存在则插入新值,返回一个可变引用
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5
    *v = 10;

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 10); // 已经存在，因此50没有插入

    // 在已有值的基础上更新,
    // example: 为每个单词计数
    let text = "hello world wonderful world";

    let mut map = std::collections::HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn change_value(v: Vec<i32>) {
    let mut v = v;
    v[0] = 999;
}

fn study_vector() {
    println!("-----------------动态数组-----------------");
    // 数组是静态分配的，长度不可变，在栈上分配，拥有Copy trait。
    // 而动态数组 Vec 是在堆上分配的，长度可变。
    let mut v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);
    v.push(1);
    v.push(2);
    println!("v = {:?}", v);

    // vector 是值传递
    let v = vec![1, 2, 3, 4, 5];
    change_value(v.clone());
    println!("{:?}", v);

    // 如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 来创建一个空的 Vec，
    // 这样可以避免多次扩容。
    let mut v: Vec<usize> = Vec::with_capacity(10);
    println!("v = {:?}", v);
    v.push(1);
    v.push(2);
    println!("v = {:?}", v);
    // len 与 cap
    println!("v.len() = {}", v.len());
    println!("v.capacity() = {}", v.capacity());
    // 通过索引访问元素
    println!("v[0] = {}", v[0]);
    // 通过索引修改元素
    v[0] = 3;
    println!("v[0] = {}", v[0]);

    // 从数组创建 Vector
    let a = [11, 22, 33, 44, 55];
    let v = a.to_vec(); // copy
    println!("v = {:?}", v);

    let v = Vec::from(a); // move
    println!("v = {:?}", v);

    // 使用宏 vec! 来创建数组
    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

    // 当Vector 类型在超出作用域后，它的所有权将会被释放，同时它所占用的内存也会被释放。
    //  Vector 被删除后，它内部存储的所有内容也会随之被删除。
    // 目前来看，这种解决方案简单直白，但是当 `Vector` 中的元素被引用后，事情可能会没那么简单。

    // 从 Vector 中删除元素
    println!("-----------------从 Vector 中删除元素-----------------");
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // 通过索引删除元素
    v.remove(2); // [1, 2, 4, 5]
    println!("v = {:?}", v);
    println!("v.len() = {}", v.len()); // 4
    println!("v.capacity() = {}", v.capacity()); // 5

    // 通过 pop 方法删除元素
    let last = v.pop();
    println!("v = {:?}", v);
    println!("last = {:?}", last);
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    // 通过 clear 方法清空 Vector (不会释放内存)
    v.clear();
    println!("v = {:?}", v);
    println!("v.len() = {}", v.len());
    println!("v.capacity() = {}", v.capacity());
    // 通过 drain 方法删除元素
    let mut v = vec![1, 2, 3, 4, 5];
    // 去除前2个元素 erase
    v.drain(0..2);
    println!("v: {:?}", v); // v: [3, 4, 5]
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    // 通过 retain 方法删除元素
    let mut v = vec![1, 2, 3, 4, 5];
    // 保留所有奇数
    v.retain(|x| x % 2 == 1);
    println!("v: {:?}", v); // v: [1, 3, 5]
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    //resize(不会释放内存)
    let mut v = vec![1, 2, 3, 4, 5];
    v.resize(3, 0);
    println!("v: {:?}", v); // v: [1, 2, 3]
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    // truncate(不会释放内存)
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(3);
    println!("v: {:?}", v); // v: [1, 2, 3]
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    // resize_with(不会释放内存)
    let mut v = vec![1, 2, 3, 4, 5];
    // If new_len is greater than len, the Vec is extended by the difference
    v.resize_with(10, Default::default);
    println!("v: {:?}", v); // v: [1, 2, 3, 4, 5, 0, 0, 0, 0, 0]
    println!("v.len() = {}", v.len()); // 10
    println!("v.capacity() = {}", v.capacity()); // 10

    // resize_with(不会释放内存)
    let mut v = vec![1, 2, 3, 4, 5];
    v.resize_with(3, Default::default);
    println!("v: {:?}", v); // v: [1, 2, 3]
    println!("v.len() = {}", v.len()); // 3
    println!("v.capacity() = {}", v.capacity()); // 5

    // 释放内存shrink_to_fit
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(2);
    v.shrink_to_fit();
    println!("v: {:?}", v); // v: [1, 2]
    println!("v.len() = {}", v.len()); // 2
    println!("v.capacity() = {}", v.capacity()); // >= 2
    assert!(v.capacity() >= 2);

    let v = vec![1, 2, 3, 4, 5];

    // 从 Vector 中读取元素
    // 1. 通过下标索引访问。
    // 1. 使用 `get` 方法。
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    //  v.get 的使用方式非常安全
    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }
    // vector 的遍历
    for i in &v {
        print!("{} ", i);
    }
    println!();
    // vector 的扩容机制(跟Go的slice类似)
    let mut v = Vec::new();
    for i in 0..70 {
        v.push(i);
        println!("len = {}, capacity = {}", v.len(), v.capacity());
    }
}

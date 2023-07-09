pub fn study_sort() {
    println!("-----------------排序-----------------");
    let mut vec = vec![1, 5, 10, 2, 15];
    // 升序
    vec.sort();
    println!("vec = {:?}", vec);
    // 降序
    vec.sort_by(|a, b| b.cmp(a));
    println!("vec = {:?}", vec);

    // unstable sort
    vec.sort_unstable();
    println!("vec = {:?}", vec);

    // sort_by_key
    let mut a = [-5i32, 4, 1, -3, 2];
    a.sort_by_key(|k| k.abs());
    println!("array = {:?}", a);

    // 对浮点数排序
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("vec = {:?}", vec);

    // 排序结构的 vector
    // 对 Person 结构的 Vector 进行排序，通过属性name和age的自然顺序（按名称和年龄）。
    // 为了使 Person 可排序，你需要四个 traitEq，PartialEq，Ord和PartialOrd。
    // 可以简单地derive出这些特征。您还可以使用一个vec:sort_by方法，提供自定义比较函数：只按年龄排序。
    println!("-----------------排序结构的 vector-----------------");

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 自然顺序，排序 people  (名字 和 年龄)
    people.sort();

    for p in &people {
        println!("{:?}", p);
    }
    println!("-----------------排序结构的 vector-----------------");
    // 用 年龄 排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    for p in &people {
        println!("{:?}", p);
    }
}

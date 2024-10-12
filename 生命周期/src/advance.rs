pub fn stuidy_lifetime_advance() {
    println!("-----------------生命周期进阶(复杂的例子)-----------------");
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);

    println!("------------------高阶特征约束(Higher-ranked trait bounds)-----------------");

    higher_ranked_lifetime_bounds();
}

fn higher_ranked_lifetime_bounds() {
    /*
    fn call_on_ref_zero<'a, F>(f: F)
    where
        F: Fn(&'a i32),
    {
        let zero = 0;
        f(&zero); //`zero` does not live long enough
    }
     */
    // 我们想要f函数能够处理任何生命周期的引用
    fn call_on_ref_zero<F>(f: F)
    where
        F: for<'a> Fn(&'a i32),
    {
        let zero = 0;
        f(&zero);
    }
    // 通过使用 for<'a>，我们告诉Rust编译器："这个函数 F 可以处理任何生命周期的引用，包括非常短暂的生命周期"。
    // 这允许我们在函数内部创建一个临时值（zero），取它的引用，并将这个短生命周期的引用传递给 F，而不会有任何生命周期冲突。

    call_on_ref_zero(|a: &i32| println!("{:?}", a));
}

// 生命周期存在的意义是保证不会出现空引用(悬垂引用)，
// 即引用对象的生命周期至少要比当前结构体的生命周期长(可以是相同的)。
// manager为持有引用的变量，生命周期的标注是对于引用来说的，而不是变量。
struct Interface<'a, 'b: 'a> {
    _manager: &'a mut Manager<'b>,
}

impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    // pub fn get_interface(&'a mut self) -> Interface {
    // 注释中方法的生命周期标注是有问题的，
    // 该方法的参数的生命周期是 'a，而 List 的生命周期也是 'a，说明该方法至少活得跟 List 一样久，
    // 而实际上这个可变引用在方法执行完毕后就会被释放，所以应该把 'a 的生命周期标注改成 'b，
    // 代表这个方法中可变引用的生命周期至少为 'b , 'b 是 'a 的子生命周期，即 'b <= 'a 。
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> {
        Interface {
            _manager: &mut self.manager,
        }
    }
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

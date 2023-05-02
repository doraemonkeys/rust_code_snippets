pub fn stuidy_lifetime_advance() {
    println!("-----------------生命周期进阶(复杂的例子)-----------------");
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
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

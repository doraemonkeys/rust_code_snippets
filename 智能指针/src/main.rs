mod cow;
fn main() {
    // 何为智能指针？能不让你写出 `****s` 形式的解引用，我认为就是智能: )，
    // 智能指针的名称来源，主要就在于它实现了 `Deref` 和 `Drop` 特征，
    // 这两个特征可以智能地帮助我们节省使用上的负担。

    // 什么是智能指针
    // 首先得实现解引用，才能叫指针。
    // 与普通指针相比，智能指针利用了 RAII（资源获取即初始化）技术对普通的指针进行封装，
    // 这使得智能指针实质是一个对象，行为表现的却像一个指针。

    // 智能指针主要用于管理在堆上分配的内存，它将普通的指针封装为一个栈对象。
    // 当栈对象的生存周期结束后，会在析构函数中释放掉申请的内存，从而防止内存泄漏。
    // 简要的说，智能指针利用了 RAII 机制，在智能指针对象作用域结束后，
    // 会自动做内存释放的相关操作，不需要我们再手动去操作内存。
    // 智能指可以自动释放内存，防止忘记调用 delete。当然还有另一个作用，就是异常安全(C++)。
    // 在一段进行了 try/catch 的代码段里面，即使你写入了 delete，也有可能因为发生异常。
    // 程序进入 catch 块，从而忘记释放内存，这些都可以通过智能指针解决。

    // 智能指针通常使用结构体实现。智能指针不同于结构体的地方在于其实现了 Deref 和 Drop trait。
    // Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码。
    // Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。

    // 实际上之前我们已经见过一些智能指针，比如第 String 和 Vec<T>，
    // 虽然当时我们并不这么称呼它们。这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们。
    // 它们也拥有元数据和额外的功能或保证。例如 String 存储了其容量作为元数据，
    // 并拥有额外的能力确保其数据总是有效的 UTF-8 编码。

    // rust常见智能指针
    // Box<T> 用于在堆上分配内存，普通胖指针
    // RC<T> 引用计数智能指针，只能用于单线程，用于在同一时刻拥有多个所有者
    // ARC<T> 用于多线程的引用计数智能指针
    // RefCell<T> 拥有内部可变性的智能指针，规避编译期的借用检查，实现Send + !Sync (并发编程章节)。
    // Mutex<T> 互斥锁，同样拥有内部可变性，用于多线程，实现Send + Sync。
    // Mutex<T> 和 RefCell<T> 的区别在于，Mutex<T> 可以在多线程中安全的传递引用。
    // 更多：并发编程\src\main.rs:154

    // Box的使用场景
    study_box();

    // Deref 和 Drop
    study_deref_and_drop();

    // Rc 与 Arc
    study_rc_and_arc();

    // Cell 和 RefCell
    study_cell();

    // Cow(Clone on Write)
    cow::study_cow();
}

fn study_rc_and_arc() {
    println!("--------------------Rc 与 Arc--------------------");
    // Rust 所有权机制要求一个值只能有一个所有者，在大多数情况下，都没有问题，但是考虑以下情况：
    // - 在图数据结构中，多个边可能会拥有同一个节点，该节点直到没有边指向它时，才应该被释放清理
    // - 在多线程中，多个线程可能会持有同一个数据，但是你受限于 Rust 的安全机制，无法同时获取该数据的可变引用
    // 以上场景不是很常见，但是一旦遇到，就非常棘手，为了解决此类问题，
    // Rust 在所有权机制之外又引入了额外的措施来简化相应的实现：
    // 通过引用计数的方式，允许一个数据资源在同一时刻拥有多个所有者。
    // 这种实现机制就是 `Rc` 和 `Arc`，前者适用于单线程，后者适用于多线程。
    // 由于二者大部分情况下都相同，因此本章将以 `Rc` 作为讲解主体，对于 `Arc` 的不同之处，另外进行单独讲解。

    // 引用计数(reference counting)，顾名思义，通过记录一个数据被引用的次数来确定该数据是否正在被使用。
    // 当引用次数归零时，就代表该数据不再被使用，因此可以被清理释放。
    // 当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，
    // 就可以使用 `Rc` 成为数据值的所有者，例如之前提到的多线程场景就非常适合。

    // 下面是经典的所有权被转移导致报错的例子：
    /*
        let s = String::from("hello, world");
        // s在这里被转移给a
        let a = Box::new(s);
        // 报错！此处继续尝试将 s 转移给 b
        let b = Box::new(s);
    */
    let a = std::rc::Rc::new(String::from("hello, world"));
    // 智能指针 `Rc<T>` 在创建时，还会将引用计数加 1，
    // 此时获取引用计数的关联函数 `Rc::strong_count` 返回的值将是 `1`。
    println!("strong_count(&a) = {}", std::rc::Rc::strong_count(&a));
    // 使用 `Rc::clone` 克隆了一份智能指针 `Rc<String>`，并将该智能指针的引用计数增加到 `2`。
    // 这里的 `clone` 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，
    // 因此 `a` 和 `b` 是共享了底层的字符串 `s`，这种复制效率是非常高的。
    // 当然你也可以使用 `a.clone()` 的方式来克隆，但是从可读性角度，我们更加推荐 `Rc::clone` 的方式。
    // 实际上在 Rust 中，还有不少 `clone` 都是浅拷贝，例如迭代器的克隆。
    let b = std::rc::Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
    // Gets the number of strong (Rc) pointers to this allocation
    println!("strong_count(&a) = {}", std::rc::Rc::strong_count(&a));
    println!("strong_count(&b) = {}", std::rc::Rc::strong_count(&b));

    // 观察引用计数的变化
    println!("--------------------观察引用计数的变化--------------------");
    let a = std::rc::Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", std::rc::Rc::strong_count(&a));
    let _b = std::rc::Rc::clone(&a);
    println!("count after creating b = {}", std::rc::Rc::strong_count(&a));
    {
        // 由于变量 `c` 在语句块内部声明，当离开语句块时它会因为超出作用域而被释放，所以引用计数会减少 1。
        let c = std::rc::Rc::clone(&a);
        println!("count after creating c = {}", std::rc::Rc::strong_count(&c));
    }
    println!(
        "count after c goes out of scope = {}",
        std::rc::Rc::strong_count(&a)
    );

    // 不可变引用
    // 事实上，`Rc<T>` 是指向底层数据的不可变的引用，因此你无法通过它来修改数据，
    // 这也符合 Rust 的借用规则：要么存在多个不可变借用，要么只能存在一个可变借用。
    // 但是实际开发中我们往往需要对数据进行修改，这时单独使用 `Rc<T>` 无法满足我们的需求，
    // 需要配合其它数据类型来一起使用，例如内部可变性的 `RefCell<T>` 类型以及互斥锁 `Mutex<T>`。
    // 事实上，在多线程编程中，`Arc` 跟 `Mutex` 锁的组合使用非常常见，
    // 它们既可以让我们在不同的线程中共享数据，又允许在各个线程中对其进行修改。
    let mut _a = std::rc::Rc::new(String::from("hello, world"));
    // 报错！`Rc<T>` 未实现 DerefMut 特征，不能通过可变引用来修改数据
    // _a.push_str("!");
    // println!("a = {}", a);

    let a = std::rc::Rc::new(std::cell::RefCell::new(String::from("hello, world")));
    // RefCell实现内部可变性，可以修改
    a.borrow_mut().push_str("!");
    println!("a = {}", a.borrow());

    // 一个综合例子
    complex_example();

    // - `Rc/Arc` 是不可变引用，你无法修改它指向的值，只能进行读取，如果要修改，
    //    需要配合后面章节的内部可变性 `RefCell` 或互斥锁 `Mutex`
    // - 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
    // - `Rc` 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 `Arc`
    // - `Rc<T>` 是一个智能指针，实现了 `Deref` 特征，因此你无需先解开 `Rc` 指针，再使用里面的 `T`。

    // Arc
    study_arc();
}

fn study_cell() {
    println!("--------------------Cell 和 RefCell--------------------");
    // `Cell` 和 `RefCell` 在功能上没有区别，区别在于 `Cell<T>` 适用于 `T` 实现 `Copy` 的情况：
    // - "asdf" 是 `&str` 类型，它实现了 `Copy` 特征
    // - `c.get` 用来取值，`c.set` 用来设置新值
    let c = std::cell::Cell::new("asdf");
    let one = c.get();
    // Cell取到值保存在 `one` 变量后，还能同时进行修改，这个违背了 Rust 的借用规则，
    // 但是由于 `Cell` 的存在，我们很优雅地做到了这一点。
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two); // asdf,qwer

    // 由于 `Cell` 类型针对的是实现了 `Copy` 特征的值类型，因此在实际开发中，`Cell` 使用的并不多，
    // 因为我们要解决的往往是可变、不可变引用共存导致的问题，此时就需要借助于 `RefCell`。
    // `Rc/Arc`让一个数据可以拥有多个所有者,`RefCell`实现编译期可变、不可变引用共存。
    // `Rc/Arc` 和 `RefCell` 合在一起，解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题。
    // 但是它们并不是银弹，例如 `RefCell` 实际上并没有解决可变引用和引用可以共存的问题，
    // 只是将报错从编译期推迟到运行时，从编译器错误变成了 `panic` 异常：
    let s = std::cell::RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    // 可变引用和引用共存，运行时panic
    // let s2 = s.borrow_mut();
    println!("{}", s1);

    // Rust 保持编译期的宁可错杀，绝不放过的原则，当编译器不能确定你的代码是否正确时，
    // 就统统会判定为错误，因此难免会导致一些误报。
    // 而 `RefCell` 正是用于你确信代码是正确的，而编译器却发生了误判时。

    // - `Cell` 只适用于 `Copy` 类型，用于提供值，而 `RefCell` 用于提供引用
    // - `Cell` 不会 `panic`，而 `RefCell` 会
    // `Cell` 没有额外的性能损耗，例如以下两段代码的性能其实是一致的：
    // code snipet 1
    let x = std::cell::Cell::new(1);
    let y = &x;
    let z = &x;
    // xyz均可对cell进行修改
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get()); // 4

    // 虽然性能一致，但代码 `1` 拥有代码 `2` 不具有的优势：它能编译成功:)
    // code snipet 2
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;// 不能同时存在两个可变引用
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);

    // RefCell 具有内部可变性
    println!("--------------------RefCell 具有内部可变性--------------------");
    // 何为内部可变性？简单来说，对一个不可变的值进行可变借用。
    // cell里面装着一个东西(拥有所有权或可变借用)，可以通过调用方法来获取这个东西的引用或者可变引用，
    // 而无需获得cell本身的可变引用，这就是内部可变性。
    // 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，
    // 这通常是借用规则所不允许的。为了改变数据，该模式在数据结构中使用 unsafe 代码
    // 来模糊 Rust 通常的可变性和借用规则。不安全代码表明我们在手动检查这些规则而不是让编译器替我们检查。

    // error
    // let x = 5;
    // let y = &mut x;

    // 在某些场景中，一个值可以在其方法内部被修改，同时对于其它代码不可变，
    // 定义在外部库中的特征
    pub trait Messenger {
        fn send(&self, msg: String);
    }

    // --------------------------
    // 我们的代码中的数据结构和实现
    struct _MsgQueue1 {
        msg_cache: Vec<String>,
    }

    // impl Messenger for _MsgQueue1 {
    //     fn send(&self, msg: String) {
    //         self.msg_cache.push(msg) // error
    //     }
    // }
    // 这时除了将 `&self` 修改为 `&mut self` 之外，还有一种解决方案，那就是使用 `RefCell`：

    pub struct MsgQueue {
        msg_cache: std::cell::RefCell<Vec<String>>,
    }
    impl Messenger for MsgQueue {
        fn send(&self, msg: String) {
            self.msg_cache.borrow_mut().push(msg)
        }
    }
    let mq = MsgQueue {
        msg_cache: std::cell::RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());

    // Rc + RefCell 组合使用
    // 两者结合的数据结构与下面Wrapper类似:
    struct _RcWrapper<T> {
        // Rc
        strong_count: usize,
        // 弱引用计数
        weak_count: usize,
        // 包裹的数据
        item: _RefCellWrapper<T>,
    }
    struct _RefCellWrapper<T> {
        // Refcell
        borrow_count: isize,
        item: T,
    }
    println!("--------------------Rc + RefCell 组合使用--------------------");
    let s = std::rc::Rc::new(std::cell::RefCell::new(
        "我很善变，还拥有多个主人".to_string(),
    ));

    let s1 = s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(", on yeah!");
    s1.borrow_mut().push_str(", haha!");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);

    // 通过 `Cell::from_mut` 解决借用冲突
    println!("--------------------通过 Cell::from_mut 解决借用冲突--------------------");
    // - Cell::from_mut，该方法将 `&mut T` 转为 `&Cell<T>`
    // - Cell::as_slice_of_cells，该方法将 `&Cell<[T]>` 转为 `&[Cell<T>]`
    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }
    // 索引的方式
    fn retain_even1(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if is_even(nums[j]) {
                nums[i] = nums[j];
                i += 1;
            }
        }
        nums.truncate(i);
    }
    // 使用 Cell
    fn retain_even2(nums: &mut Vec<i32>) {
        // 先用cell裹一个可变借用数组，
        // 再转化为cell的切片，分别包裹数组中的每一个元素，这样就能进行迭代了
        let slice: &[std::cell::Cell<i32>] =
            std::cell::Cell::from_mut(&mut nums[..]).as_slice_of_cells();

        let mut i = 0;
        // 直接对nums使用iter()方法，会存在可变引用和不可变引用同时存在的问题
        for num in slice.iter().filter(|num| is_even(num.get())) {
            // 虽然num是对cell的不可变引用，但是利用cell的内部可变性，可以修改cell包裹的东西的值
            slice[i].set(num.get());
            i += 1;
        }

        nums.truncate(i);
    }
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    retain_even1(&mut nums);
    println!("{:?}", nums);
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    retain_even2(&mut nums);
    println!("{:?}", nums);
    // 总结
    // `Cell` 和 `RefCell` 都为我们带来了内部可变性这个重要特性，
    // 同时还将借用规则的检查从编译期推迟到运行期，
    // 但是这个检查并不能被绕过，该来早晚还是会来，`RefCell` 在运行期的报错会造成 `panic`。
}

fn study_arc() {
    println!("--------------------Arc--------------------");
    // `Rc<T>` 不能在线程间安全的传递，实际上是因为它没有实现 `Send` 特征，
    // 而该特征是恰恰是多线程间传递数据的关键。
    // 当然，还有更深层的原因：由于 `Rc<T>` 需要管理引用计数，
    // 但是该计数器并没有使用任何并发原语，因此无法实现原子化的计数操作，最终会导致计数错误。
    // `Arc` 是 `Rc` 的多线程版本，其全称是 `Atomically Reference Counted`，即原子引用计数。

    // T实现了Send和Sync, Arc<T>才会实现Send和Sync，为什么不能将非线程安全的类型T放在Arc<T>中以使其线程安全?
    // 这乍一看可能有点反直觉:毕竟，Arc<T的重点不就是>线程安全吗?
    // 关键是:Arc<T>使其对同一数据具有多个所有权是线程安全的，但它没有为其数据增加线程安全性。
    // 考虑Arc<RefCell<T>>。[RefCell<T>]没有实现Sync，
    // 如果总是为Arc<T>实现Send，那么Arc<RefCell<T>>相当于为RefCell<T>做了多线程的引用。

    // T: Sync 等价于&T: Send，意义上是某个数据能否被多个线程同时不可变地访问。
    // 只需实现Sync，则可以包括但不限于 放进Arc里并将Arc复制后传给别的线程。

    // 多线程直接引用会报错:`closure may outlive the current function,`,
    // 原因在于编译器无法确定主线程`main`和子线程`t`谁的生命周期更长，特别是当两个线程都是子线程时，
    // 没有任何人知道哪个子线程会先结束，包括编译器！
    // 因此我们得配合`Arc`去使用:
    let s = std::sync::Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = std::sync::Arc::clone(&s);
        // 首先通过 `thread::spawn` 创建一个线程，
        // 然后使用 `move` 关键字把克隆出的 `s` 的所有权转移到线程中。
        let _handle = std::thread::spawn(move || println!("{}", s));
    }
    // wait
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn complex_example() {
    println!("--------------------一个综合例子--------------------");
    struct Owner {
        name: String,
        // ...其它字段
    }

    struct Gadget {
        id: i32,
        owner: std::rc::Rc<Owner>,
        // ...其它字段
    }

    // 创建一个基于引用计数的 `Owner`.
    let gadget_owner: std::rc::Rc<Owner> = std::rc::Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });

    // 创建两个不同的工具，它们属于同一个主人
    let gadget1 = Gadget {
        id: 1,
        owner: std::rc::Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: std::rc::Rc::clone(&gadget_owner),
    };

    // 释放掉第一个 `Rc<Owner>`
    drop(gadget_owner);

    // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
    // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
    // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
    // 引用指向底层的 owner 数据，引用计数尚未清零
    // 因此 owner 数据依然可以被使用
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
    // 数据也被清理释放
}

fn study_deref_and_drop() {
    println!("--------------------Deref 和 Drop--------------------");
    // 考虑一下智能指针，它是一个结构体类型，如果你直接对它进行 `*myStruct`，显然编译器不知道该如何办，
    // 因此我们可以为智能指针结构体实现 `Deref` 特征。
    // 实现 `Deref` 后的智能指针结构体，就可以像普通引用一样，通过 `*` 进行解引用，例如 `Box<T>` 智能指针。
    let x = Box::new(1);
    let sum = *x + 1;
    println!("sum = {}", sum); // sum = 2

    // 定义自己的智能指针
    println!("--------------------定义自己的智能指针--------------------");
    // 让我们一起来实现一个智能指针，功能上类似 `Box<T>`。
    // 一个类型为 `T` 的对象 `foo`，如果 `T: Deref<Target=U>`，
    // 那么，相关 `foo` 的引用 `&foo` 在应用的时候会自动转换为 `&U`。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = MyBox::new(1);
    let sum = *x + 1;
    println!("sum = {}", sum); // sum = 2

    // 函数和方法中的隐式 Deref 转换
    println!("--------------------函数和方法中的隐式 Deref 转换--------------------");
    // 若一个类型实现了 `Deref` 特征，那它的引用在传给函数或方法时，
    // 会根据参数签名来决定是否进行隐式的 `Deref` 转换。
    fn display(s: &str) {
        println!("{}", s);
    }
    let s = String::from("hello world");
    display(&s);

    // `Deref` 可以支持连续的隐式转换，直到找到适合的形式为止：
    let s = MyBox::new(String::from("hello world"));
    display(&s);

    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    // 实际上 `MyBox` 根本没有没有实现该方法，能调用 `to_string`，
    // 完全是因为编译器对 `MyBox` 应用了 `Deref` 的结果（方法调用会自动解引用）。
    let s2: String = s.to_string();
    println!("s1 = {}, s2 = {}", s1, s2);
    deref_example();

    // 三种 Deref 转换
    study_three_deref();

    // Drop
    study_drop();
}

fn study_drop() {
    println!("--------------------Drop--------------------");
    // 在 Rust 中，我们之所以可以一拳打跑 GC 的同时一脚踢翻手动资源回收，
    // 主要就归功于 `Drop` 特征，同时它也是智能指针的必备特征之一。
    // 在一些无 GC 语言中，程序员在一个变量无需再被使用时，需要手动释放它占用的内存资源，
    // 如果忘记了，那么就会发生内存泄漏，最终臭名昭著的 `OOM` 问题可能就会发生。
    // 而在 Rust 中，你可以指定在一个变量超出作用域时，执行一段特定的代码，
    // 最终编译器将帮你自动插入这段收尾代码。这样，就无需在每一个使用该变量的地方，
    // 都写一段代码来进行收尾工作和资源释放。不禁让人感叹，Rust 的大腿真粗，香！

    // 一个简单的 Drop 例子
    drop_example();

    println!("--------------------drop使用场景--------------------");
    // 当使用智能指针来管理锁的时候，你可能希望提前释放这个锁，然后让其它代码能及时获得锁，
    // 此时就需要提前去手动 `drop`。 但是在之前我们提到一个悬念，`Drop::drop` 只是借用了目标值的可变引用，
    // 所以，就算你提前调用了 `drop`，后面的代码依然可以使用目标值，
    // 但是这就会访问一个并不存在的值，非常不安全，好在 Rust 会阻止你。
    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }
    let foo = Foo;
    // 编译器直接阻止了我们调用 `Drop` 特征的 `drop` 方法，原因是对于 Rust 而言，不允许显式地调用析构函数。
    //foo.drop(); // explicit destructor calls not allowed
    // 好在在报错的同时，编译器还给出了一个提示：使用 `drop` 函数。
    // drop 函数的定义如下：
    // pub fn drop<T>(_x: T)
    // 可以看出drop函数能够拿走目标值的所有权，这样就不会出现访问一个并不存在的值的情况了。

    println!("Running!:{:?}", foo);

    // 在绝大多数情况下，我们都无需手动去 `drop` 以回收内存资源，因为 Rust 会自动帮我们完成这些工作，
    // 它甚至会对复杂类型的每个字段都单独的调用 `drop` 进行回收！
    // 但是确实有极少数情况，需要你自己来回收资源的，例如文件描述符、网络 socket 等，
    // 当这些值超出作用域不再使用时，就需要进行关闭以释放相关的资源，
    // 在这些情况下，就需要使用者自己来解决 `Drop` 的问题。

    // 互斥的 Copy 和 Drop
    println!("--------------------互斥的 Copy 和 Drop--------------------");
    // 我们无法为一个类型同时实现 `Copy` 和 `Drop` 特征。因为实现了 `Copy` 的特征会被编译器隐式的复制，
    // 因此非常难以预测析构函数执行的时间和频率。因此这些实现了 `Copy` 的类型无法拥有析构函数。
    // copy可以理解为栈内存的简单复制，通常意义上的浅拷贝，trivial copy。
    // 简单举例来说，有一个结构体只包含一个指针，这个指针指向分配出来的堆内存。
    // 它实现了Drop，作用是释放堆上的内存（类似于c++里面的析构函数）。
    // 编译器做的工作是在栈上分配的类或者结构体，在离开作用域时自动插入析构的函数。
    // copy会把这个指针复制一遍，这时候就有两个结构体在栈上。
    // 结构体离开了作用域，会调用drop，这个时候就会调用两遍析构，
    // 但是结构体管理的实际资源（堆上的一段内存）只有一个，此时资源就被释放两遍了。这是一种内存错误。
}

fn drop_example() {
    println!("--------------------一个简单的 Drop 例子--------------------");
    struct HasDrop1;
    struct HasDrop2;
    // `Drop` 特征中的 `drop` 方法借用了目标的可变引用，而不是拿走了所有权，这里先设置一个悬念，后边会讲到。
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }
    struct HasTwoDrops {
        _one: HasDrop1,
        _two: HasDrop2,
    }
    // 实际上，就算你不为 `HasTwoDrops` 结构体实现 `Drop` 特征，它内部的两个字段依然会调用 `drop`，
    // 原因在于，Rust 自动为几乎所有类型都实现了 `Drop` 特征，因此就算你不手动为结构体实现 `Drop`，
    // 它依然会调用默认实现的 `drop` 函数，同时再调用每个字段的 `drop` 方法。
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }
    let _x = HasTwoDrops {
        _two: HasDrop2,
        _one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");
    // Running!
    // Dropping Foo!
    // Dropping HasTwoDrops!
    // Dropping HasDrop1!
    // Dropping HasDrop2!
}

fn study_three_deref() {
    println!("--------------------三种 Deref 转换--------------------");
    // 实际上 Rust 还支持将一个可变的引用转换成另一个可变的引用以及将一个可变引用转换成不可变的引用，规则如下：
    // - 当 `T: Deref<Target=U>`，可以将 `&T` 转换成 `&U`，也就是我们之前看到的例子
    // - 当 `T: DerefMut<Target=U>`，可以将 `&mut T` 转换成 `&mut U`
    // - 当 `T: Deref<Target=U>`，可以将 `&mut T` 转换成 `&U`
    struct MyBox<T> {
        v: T,
    }
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox { v: x }
        }
    }
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            println!("deref");
            &self.v
        }
    }
    // DerefMut 将 &mut T 转换为 &mut U
    impl<T> std::ops::DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            println!("deref_mut");
            &mut self.v
        }
    }
    fn display(s: &mut String) {
        // 对于可变引用，编译器会自动调用 `DerefMut`。
        s.push_str("world");
        println!("{}", s);
    }
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s)
}

fn deref_example() {
    println!("--------------------example--------------------");
    fn foo(s: &str) {
        println!("foo: {}", s);
    }
    // String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // 且 Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String ，然后 &String 再自动解引用为 &str
    let counted = std::rc::Rc::new(owned);

    // 因此下面的函数可以正常运行:
    foo(&counted);

    struct Foo;
    impl Foo {
        fn foo(&self) {
            println!("Foo");
        }
    }
    let f = &&Foo;

    // 把多重`&`，例如 `&&&&&&&v`，归一成 `&v`。
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

fn study_box() {
    // 堆栈的性能
    println!("--------------------堆栈的性能--------------------");
    // 由于我们在后面的性能专题会专门讲解堆栈的性能问题，因此这里就大概给出结论：
    // - 小型数据，在栈上的分配性能和读取性能都要比堆上高
    // - 中型数据，栈上分配性能高，但是读取性能和堆上并无区别，
    //   因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
    // - 大型数据，只建议在堆上分配和使用
    // 总之，栈的分配速度肯定比堆上快，但是读取速度往往取决于你的数据能不能放入寄存器或 CPU 高速缓存。
    // 因此不要仅仅因为堆上性能不如栈这个印象，就总是优先选择栈，导致代码更复杂的实现。

    // Box的使用场景
    // 由于 `Box` 是简单的封装，除了将值存储在堆上外，并没有其它性能上的损耗。
    // `Box` 相比其它智能指针，功能较为单一，可以在以下场景中使用它：
    // - 特意的将数据分配在堆上
    // - 数据较大时，又不想在转移所有权时进行数据拷贝
    // - 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
    // - 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

    // 使用 `Box` 将数据存储在堆上
    println!("--------------------使用 `Box` 将数据存储在堆上--------------------");
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
    let b = *a + 1; // 通过解引用操作符 `*` 来解引用 `Box`，然后再进行加法运算
    println!("b = {}", b); // b = 4

    //  智能指针往往都实现了 `Deref` 和 `Drop` 特征，因此：
    // - `println!` 可以正常打印出 `a` 的值，是因为它隐式地调用了 `Deref` 对智能指针 `a` 进行了解引用。
    // - 最后一行代码 `let b = a + 1` 报错，是因为在表达式中，我们无法自动隐式地执行 `Deref` 解引用操作，
    //   你需要使用 `*` 操作符 `let b = *a + 1`，来显式的进行解引用。
    // - `a` 持有的智能指针将在作用域结束时，被释放掉，这是因为 `Box<T>` 实现了 `Drop` 特征。
    // 以上的例子在实际代码中其实很少会存在，因为将一个简单的值分配到堆上并没有太大的意义。
    // 将其分配在栈上，由于寄存器、CPU 缓存的原因，它的性能将更好，而且代码可读性也更好。

    // 避免栈上数据的拷贝
    println!("--------------------避免栈上数据的拷贝--------------------");
    // 当栈上数据转移所有权时，实际上是把数据拷贝了一份，最终新旧变量各自拥有不同的数据，因此所有权并未转移。
    // 而堆上则不然，底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，再将新的指针赋予新的变量，
    // 然后让拥有旧指针的变量失效，最终完成了所有权的转移：

    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;
    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    // from_raw
    println!("--------------------from_raw--------------------");
    // Box::new 将数据分配到堆上，但有一个初始化的过程。
    fn _box_example() {
        //  overflowed its stack
        let _v = Box::new([255u8; 512 * 2 * 1024 * 512]);
    }
    // 上面的例子在debug模式中，数组会先在栈中创建，再传递给Box复制到堆中，但是占用内存太大，会报栈溢出的错误。
    // release模式下会开启优化，直接在堆上创建，不会报错。
    // 实际开发中我们会直接使用vector来创建堆上的数组。

    // 为了解决这个问题，我们可以使用Box::from_raw。
    fn _box_example2() {
        use std::alloc::{Layout, alloc};
        // 通过数据类型构建内存布局
        let layout = Layout::new::<[u8; 512 * 2 * 1024 * 512]>();
        // 分配内存
        let ptr = unsafe { alloc(layout) };
        let mut v = unsafe { Box::from_raw(ptr as *mut [u8; 512 * 2 * 1024 * 512]) };
        for i in 0..v.len() {
            v[i] = 255;
        }
        // 通过 Box::from_raw引用后，不用再使用 dealloc 方法手动释放内存。
    }
    _box_example();

    // 将动态大小类型变为 Sized 固定大小类型
    println!("--------------------将动态大小类型变为 Sized 固定大小类型--------------------");
    // Rust 需要在编译时知道类型占用多少空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小类型 DST。
    // 其中一种无法在编译时知道大小的类型是递归类型：在类型定义中又使用到了自身，
    // 或者说该类型的值的一部分可以是相同类型的其它值，这种值的嵌套理论上可以无限进行下去，
    // 所以 Rust 不知道递归类型需要多少空间：
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // 此时若想解决这个问题，就可以使用我们的 `Box<T>`：
    // 将 List 存储到堆上，然后使用一个智能指针指向它，即可完成从 DST 到 Sized 类型(固定大小类型)的华丽转变。
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", list);

    // 特征对象
    study_trait_object();

    // Box内存布局
    study_box_memory_layout();

    // Box::leak
    study_box_leak();
}

fn study_box_leak() {
    println!("--------------------Box::leak--------------------");
    // `Box` 中还提供了一个非常有用的关联函数：`Box::leak`，它可以消费掉 `Box` 并且强制目标值从内存中泄漏。

    // 你可以把一个 `String` 类型，变成一个 `'static` 生命周期的 `&str` 类型：
    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello, world");

        Box::leak(s.into_boxed_str())
    }
    let s = gen_static_str();
    println!("{}", s);
    //在之前的代码中，如果 `String` 创建于函数中，那么返回它的唯一方法就是转移所有权给调用者
    // `fn move_str() -> String`，
    // 而通过 `Box::leak` 我们不仅返回了一个 `&str` 字符串切片，它还是 `'static` 生命周期的！

    // 要知道真正具有 `'static` 生命周期的往往都是编译期就创建的值，
    // 例如 `let v = "hello, world"`，这里 `v` 是直接打包到二进制可执行文件中的，
    // 因此该字符串具有 `'static` 生命周期，再比如 `const` 常量。
    // 又有读者要问了，我还可以手动为变量标注 `'static` 啊。其实你标注的 `'static` 只是用来忽悠编译器的，
    // 但是超出作用域，一样被释放回收。而使用 `Box::leak` 就可以将一个运行期的值转为 `'static`。

    // 我说一个简单的场景，你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，
    // 那么就可以使用 `Box::leak`，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，
    // 那么就可以将其转为全局有效，虽然 `Rc/Arc` 也可以实现此功能，但是 `Box::leak` 是性能最高的。

    // `Box` 背后是调用 `jemalloc` 来做内存管理，所以堆上的空间无需我们的手动管理。
    // 与此类似，带 GC 的语言中的对象也是借助于 `Box` 概念来实现的，
    // 一切皆对象 = 一切皆 Box， 只不过我们无需自己去 `Box` 罢了。

    // 其实很多时候，编译器的鞭笞可以助我们更快的成长，
    // 例如所有权规则里的借用、move、生命周期就是编译器在教我们做人，
    // 哦不是，是教我们深刻理解堆栈、内存布局、作用域等等你在其它 GC 语言无需去关注的东西。
    // 刚开始是很痛苦，但是一旦熟悉了这套规则，写代码的效率和代码本身的质量将飞速上升，
    // 直到你可以用 Java 开发的效率写出 Java 代码不可企及的性能和安全性，
    // 最终 Rust 语言所谓的开发效率低、心智负担高，对你来说终究不是个事。
}

fn study_box_memory_layout() {
    println!("--------------------Box内存布局--------------------");
    // 先来看看 `Vec<i32>` 的内存布局：
    /*
            (stack)    (heap)
            ┌──────┐   ┌───┐
            │ vec1 │──→│ 1 │
            └──────┘   ├───┤
                       │ 2 │
                       ├───┤
                       │ 3 │
                       ├───┤
                       │ 4 │
                       └───┘
    */
    // `Vec` 和 `String` 都是智能指针，从上图可以看出，该智能指针存储在栈中，然后指向堆上的数组数据。

    // 那如果数组中每个元素都是一个 `Box` 对象呢？来看看 `Vec<Box<i32>>` 的内存布局：
    /*
                                   (heap)
               (stack)    (heap)   ┌───┐
               ┌──────┐   ┌───┐ ┌─→│ 1 │
               │ vec2 │──→│B1 │─┘  └───┘
               └──────┘   ├───┤    ┌───┐
                          │B2 │───→│ 2 │
                          ├───┤    └───┘
                          │B3 │─┐  ┌───┐
                          ├───┤ └─→│ 3 │
                          │B4 │─┐  └───┘
                          └───┘ │  ┌───┐
                                └─→│ 4 │
                                   └───┘
    */
    // 可以看出智能指针 `vec2` 依然是存储在栈上，然后指针指向一个堆上的数组，
    // 该数组中每个元素都是一个 `Box` 智能指针，最终 `Box` 智能指针又指向了存储在堆上的实际值。
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("sum = {}", sum);
}

fn study_trait_object() {
    println!("--------------------特征对象--------------------");
    // 在 Rust 中，想实现不同类型组成的数组只有两个办法：枚举和特征对象，
    // 前者限制较多，因此后者往往是最常用的解决办法。
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("这是屏幕上第{}号按钮", self.id)
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("这个选择框贼难用{}", self.id)
        }
    }
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for e in elems {
        e.draw()
    }
    // 以上代码将不同类型的 `Button` 和 `Select` 包装成 `Draw` 特征的特征对象，放入一个数组中，
    // `Box<dyn Draw>` 就是特征对象。
    // 其实，特征也是 DST 类型，而特征对象在做的就是将 DST 类型转换为固定大小类型。
}

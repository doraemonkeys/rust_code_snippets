use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Weak 与循环引用
    study_weak_and_cycle_reference();

    // 结构体的自引用
    study_struct_self_reference();

    // Rc + RefCell 或 Arc + Mutex
    study_rc_refcell_or_arc_mutex();
}

fn study_rc_refcell_or_arc_mutex() {
    println!("-----------------Rc + RefCell 或 Arc + Mutex-----------------");
}

fn study_struct_self_reference() {
    println!("-----------------结构体的自引用-----------------");
    unsafe_example();
    pin_example();
    // 第三方库提供的自引用类型
    ouroboros_example();
}

fn ouroboros_example() {
    println!("-----------------ouroboros_example-----------------");
    // 对于自引用结构体，三方库也有支持的，其中一个就是 ouroboros，当然它也有自己的限制。
    use ouroboros::self_referencing;

    #[self_referencing]
    struct SelfRef {
        value: String,

        #[borrows(value)]
        pointer_to_value: &'this str,
    }

    let v = SelfRefBuilder {
        value: "aaa".to_string(),
        pointer_to_value_builder: |value: &String| value,
    }
    .build();

    // 借用value值
    let s = v.borrow_value();
    // 借用指针
    let p = v.borrow_pointer_to_value();
    // value值和指针指向的值相等
    assert_eq!(s, *p);
    // 它确实帮助我们解决了问题，但是一个是破坏了原有的结构，另外就是并不是所有数据类型都支持：
    // 它需要目标值的内存地址不会改变，因此 `Vec` 动态数组就不适合，
    // 因为当内存空间不够时，Rust 会重新分配一块空间来存放该数组，这会导致内存地址的改变。
}

fn pin_example() {
    println!("-----------------pin_example-----------------");
    // `unsafe` 虽然简单好用，但是它不太安全。
    // 自引用最麻烦的就是创建引用的同时，值的所有权会被转移，而通过 `Pin` 就可以很好的防止这一点。
    // 一个不占用空间的类型，用于标记一个类型的实例不能被移动。
    // A marker type which does not implement Unpin.
    // If a type contains a PhantomPinned, it will not implement Unpin by default.
    use std::marker::PhantomPinned;
    // A pinned pointer.
    // This is a wrapper around a kind of pointer which
    // makes that pointer "pin" its value in place,
    // preventing the value referenced by that pointer
    // from being moved unless it implements Unpin.
    use std::pin::Pin;
    use std::ptr::NonNull;

    // 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
    // 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
    // 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
    struct Unmovable {
        data: String,
        slice: NonNull<String>,
        _pin: PhantomPinned,
    }

    impl Unmovable {
        // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针。
        // Pin可以固定住一个值，防止该值在内存中被移动。
        // 自引用最麻烦的就是创建引用的同时，值的所有权会被转移，而通过 `Pin` 就可以很好的防止这一点：
        fn new(data: String) -> Pin<Box<Self>> {
            let res = Unmovable {
                data,
                // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
                slice: NonNull::dangling(),
                _pin: PhantomPinned,
            };
            let mut boxed = Box::pin(res);

            let slice = NonNull::from(&boxed.data);
            // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权。
            // 虽然使用了 unsafe，其实更多的是无奈之举，跟之前的 unsafe 实现完全不可同日而语。
            // 其实 Pin 在这里并没有魔法，它也并不是实现自引用类型的主要原因，
            // 最关键的还是里面的裸指针的使用，而 Pin 起到的作用就是确保我们的值不会被移走，否
            // 则指针就会指向一个错误的地址！
            unsafe {
                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice;
            }
            boxed
        }
    }
    let unmoved = Unmovable::new("hello".to_string());
    // 只要结构体没有被转移，那指针就应该指向正确的位置，而且我们可以随意移动指针
    let still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    // 因为我们的类型没有实现 `Unpin` 特征，下面这段代码将无法编译
    // let mut new_unmoved = Unmovable::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
fn unsafe_example() {
    println!("-----------------unsafe_example-----------------");
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        //  该指针指向上面的value
        pointer_to_value: *mut String,
    }

    impl SelfRef {
        fn new(txt: &str) -> Self {
            let mut ret = SelfRef {
                value: String::from(txt),
                pointer_to_value: std::ptr::null_mut(),
            };
            ret.init();
            ret
        }

        fn init(&mut self) {
            let self_ref: *mut String = &mut self.value;
            self.pointer_to_value = self_ref;
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn pointer_to_value(&self) -> &String {
            assert!(
                !self.pointer_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.pointer_to_value) }
        }
    }
    let mut t = SelfRef::new("hello");
    println!("{}, {:p}", t.value(), t.pointer_to_value());

    t.value.push_str(", world");
    unsafe {
        (&mut *t.pointer_to_value).push_str("!");
    }
    println!("{}, {:p}", t.value(), t.pointer_to_value());
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), //Cons means construct
    Nil,
}

impl List {
    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn study_weak_and_cycle_reference() {
    println!("-----------------Weak 与循环引用-----------------");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    //    a
    // ┌──────┐    ┌──────┐
    // │ 5 | ─┼─>  │ Nil  │
    // └──────┘    └──────┘

    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.next());

    // 创建`b`到`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //    b           a
    // ┌──────┐    ┌──────┐   ┌──────┐
    // │ 10 |─┼─>  │ 5 | ─┼─> │ Nil  │
    // └──────┘    └──────┘   └──────┘

    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.next());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a.next() {
        *(link.borrow_mut()) = Rc::clone(&b);
    }

    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    // println!("a next item = {:?}", a.next());

    // 那么问题来了？ 如果我们确实需要实现上面的功能，该怎么办？答案是使用 `Weak`。
    // `Weak` 非常类似于 `Rc`，但是与 `Rc` 持有所有权不同，`Weak` 不持有所有权，
    // 它仅仅保存一份指向数据的弱引用：如果你想要访问数据，需要通过 `Weak` 指针的 `upgrade` 方法实现，
    // 该方法返回一个类型为 `Option<Rc<T>>` 的值, 即若数据存在则返回一个强引用。
    // 何为弱引用？就是不保证引用关系依然存在，如果不存在，就返回一个 `None`！
    // 因为 `Weak` 引用不计入所有权，因此它无法阻止所引用的内存值被释放掉。
    // 使用方式简单总结下：
    // 对于父子引用关系，可以让父节点通过 `Rc` 来引用子节点，
    // 然后让子节点通过 `Weak` 来引用父节点。
    // printlin!发现弱引用时，不会进行递归访问，而是直接输出Weak。

    // 创建Rc，持有一个值5
    let five = Rc::new(5);

    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);
    println!("strong_count = {}", Rc::strong_count(&five));

    // Weak引用的资源依然存在，返回强引用
    let strong_five = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five.unwrap());
    println!("strong_count = {}", Rc::strong_count(&five));

    // 手动释放资源`five`这个强引用
    drop(five);

    // Weak引用的资源已不存在，因此返回None
    let strong_five2 = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five2);

    // 例1：工具间的故事
    example1();

    // 例2：数据结构-树
    example2();
}

fn example2() {
    println!("-----------------例2：数据结构-树-----------------");
    use std::rc::Weak;

    #[derive(Debug)]
    struct Node {
        _value: i32,
        parent: RefCell<Weak<Node>>,
        _children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        _value: 3,
        parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![]),
    });
    if let None = leaf.parent.borrow().upgrade() {
        println!("leaf has no parent");
    }

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            _value: 5,
            parent: RefCell::new(Weak::new()),
            _children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // 此时branch的parent指向自己的弱引用，child指向leaf的强引用
        if let Some(parirnt) = leaf.parent.borrow().upgrade() {
            // printlin!发现弱引用时，不会进行递归访问，而是直接输出Weak
            println!("leaf parent = {:?}", parirnt);
        }

        // branch strong = 1, weak = 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // leaf strong = 2, weak = 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // branch离开作用域，其强引用计数为0，被释放，leaf的强引用计数减1

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), //1
        Rc::weak_count(&leaf),   //0
    );
}

fn example1() {
    println!("-----------------例1：工具间的故事-----------------");
    // 工具间里，每个工具都有其主人，且多个工具可以拥有一个主人；同时一个主人也可以拥有多个工具。
    use std::rc::Weak;
    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }
    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }
    // 创建一个 Owner
    // 需要注意，该 Owner 也拥有多个 `gadgets`
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    // 创建工具，同时与主人进行关联：创建两个 gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    });

    // 为主人更新它所拥有的工具
    // 因为之前使用了 `Rc`，现在必须要使用 `Weak`，否则就会循环引用
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));

    // 遍历 gadget_owner 的 gadgets 字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，Option 为 None 的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // 在 main 函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
    // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
    // 首先 gadget2 和 gadget1 被销毁。
    // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
    // 循环引用问题也就避免了
}

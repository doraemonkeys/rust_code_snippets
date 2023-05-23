/*
    1. 包含自身而无限递归的结构体，编译器无法推断结构体大小
    struct Node<T> {
        val: T,
        next: Node<T>,
    }
    解决方法1：
    使用Box智能指针指向堆上的数据

    解决方法2：
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
*/

/*
    2. MyLinkedList 的 head 和 tail 不能同时获得同一个 Node 的所有权(仅有一个元素时)
    3. next 与 prev 不能表示空节点
    struct Node<T> {
        val: T,
        next: Box<Node<T>>,
        prev: Box<Node<T>>,
    }
    2. 使用单链表
    2. 使用引用计数RC + 内部可变的RefCell
    3. 使用Option枚举类型
*/

// 1. 裸指针实现
// 2. 枚举实现
// 3. 结构体实现

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: NodePtr<T>,
}

// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         println!("drop node");
//     }
// }

struct MyLinkedList<T> {
    len: i32,
    head: NodePtr<T>,
}

impl<T: Clone> MyLinkedList<T> {
    fn new() -> Self {
        MyLinkedList { len: 0, head: None }
    }

    fn get(&self, index: i32) -> Option<T> {
        let mut cur_index = 0;
        let mut cur_node = self.head.as_ref();

        while cur_index < self.len {
            if cur_index == index {
                return Some(cur_node.unwrap().val.clone());
            }
            cur_index = cur_index + 1;
            cur_node = cur_node.unwrap().next.as_ref();
        }
        None
    }

    fn add_at_head(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            // 只需要可变引用，就能拿走Option内部元素的所有权
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len = self.len + 1;
    }

    fn add_at_tail(&mut self, val: T) {
        if self.head.is_none() {
            self.add_at_head(val);
            return;
        }
        let mut cur_index = 0;
        let mut cur_node = self.head.as_mut().unwrap();
        loop {
            if cur_index == self.len - 1 {
                break;
            }
            cur_index = cur_index + 1;
            cur_node = cur_node.next.as_mut().unwrap();
        }
        let new_node = Box::new(Node { val, next: None });
        cur_node.next = Some(new_node);
        self.len = self.len + 1;
    }

    fn add_at_index(&mut self, index: i32, val: T) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }
        if index == self.len {
            self.add_at_tail(val);
            return;
        }
        let mut cur_index = 0;
        let mut cur_node = self.head.as_mut();
        let mut new_node = Box::new(Node { val, next: None });
        while cur_index < self.len {
            if cur_index == index - 1 {
                // cur_node是 Option<&mut Box<Node<T>>> ，
                // unwrap()会拿走cur_node的所有权，虽然合法，但后面就不能再继续使用cur_node了，
                // 所以先as_mut()获取cur_node的可变引用。
                // Option<&mut Box<Node<T>>> --> Option<&mut &mut Box<Node<T>>>
                let back_node = cur_node.as_mut().unwrap().next.take();
                new_node.next = back_node;
                cur_node.unwrap().next = Some(new_node);
                self.len = self.len + 1;
                return;
            }
            cur_index = cur_index + 1;
            cur_node = cur_node.unwrap().next.as_mut();
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= self.len {
            return;
        }
        if index == 0 {
            // self是可变引用(&mut self)所以self.head也是head的可变引用，
            // 若直接调用unwrap(),表示正在拿走self.head的所有权，这对于可变引用是不允许的。
            // 所以先as_mut()获取self.head的可变引用,这时调用unwrap()只拿走了可变引用的所有权。
            // Option<Box<Node<T>>> --> Option<&mut Box<Node<T>>>
            self.head = self.head.as_mut().unwrap().next.take();
            self.len = self.len - 1;
            // 函数在返回是会释放函数栈上的内存，但不会释放堆上申请的，
            // 在Rust中，绝大多数情况下，我们都无需手动去 `drop` 以回收堆上的内存资源，
            // 因为 Rust 会自动帮我们完成这些工作，Rust 自动为几乎所有类型都实现了 `Drop` 特征。
            return;
        }
        let mut cur_index = 0;
        let mut cur_node = self.head.as_mut().unwrap();
        while cur_index < self.len {
            if cur_index == index - 1 {
                cur_node.next = cur_node.next.as_mut().unwrap().next.take();
                self.len = self.len - 1;
                return;
            }
            cur_index = cur_index + 1;
            cur_node = cur_node.next.as_mut().unwrap();
        }
    }
}

impl<T> MyLinkedList<T>
where
    T: std::fmt::Display,
{
    fn print(&self) {
        let mut cur_index = 0;
        let mut cur_node = self.head.as_ref();
        while cur_index < self.len {
            // 若Option内部是不可变引用，则Option自动实现Copy trait
            // 所有的不可变引用都拥有Copy trait。
            // 所以这里 unwrap()没有拿走cur_node的所有权只是Copy了一份。
            print!("{} ", cur_node.unwrap().val);
            cur_index = cur_index + 1;
            cur_node = cur_node.unwrap().next.as_ref();
        }
        println!("长度：{}", self.len);
    }
}

fn main() {
    let mut list = MyLinkedList::new();
    let val = list.get(0);
    if val.is_none() {
        println!("None");
    }
    list.print();
    list.add_at_head(55);
    list.print();
    let val = list.get(0);
    println!("{:?}", val);

    list.add_at_head(66);
    let val2 = list.get(0);
    println!("{:?}", val2);
    let val = list.get(1);
    println!("{:?}", val);
    list.print();

    // 66 55
    list.add_at_tail(99);
    let val = list.get(2);
    println!("{:?}", val);

    // 66 55 99
    list.print();

    list.add_at_index(1, 77);
    // 66 77 55 99
    list.print();

    list.delete_at_index(0);
    list.print();
    // 77 55 99
    list.delete_at_index(1);
    list.print();
}

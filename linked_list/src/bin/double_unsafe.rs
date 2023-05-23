use std::{fmt::Debug, ptr::NonNull};
type Link<T> = Option<NonNull<Node<T>>>;
// NonNull<T>表示一个非空的裸指针，所以一般配合Option一起使用。
// NonNull有时比原始指针更危险，如果你不确定是否该使用NonNull<T>，那么就使用*mut T 。
// Box，Vec之类在alloc包里，都是封装好提高易用性的堆上数据结构，适合大部分情况与数据本身打交道的场合。
// NonNull，Unique众在core::ptr目录下，是指针的一种简单包装，适合与指针打交道的偏底层场合。
// NonNull 的优势：
// 非空指针。会自动检查包装的指针是否为空。
// 协变。方便安全抽象。如果用裸指针，则需要配合 PhantomData类型来保证协变。
struct MyLinkedList<T>
where
    T: Debug,
{
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

#[derive(Debug)]
struct Node<T>
where
    T: Debug,
{
    val: T,
    prev: Link<T>,
    next: Link<T>,
}
impl<T> Drop for Node<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("drop node {:?}", self);
    }
}

impl<T: Clone + Debug> MyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn get(&self, index: i32) -> Option<T> {
        if index < 0 || index >= self.len as i32 {
            return None;
        }
        let mut cur_node = self.head.unwrap().as_ptr();
        let mut cur_index = 0;
        while cur_index < index {
            cur_index += 1;
            cur_node = unsafe { cur_node.as_ref().unwrap().next.unwrap().as_ptr() };
        }
        Some(unsafe { cur_node.as_ref().unwrap().val.clone() })
    }

    fn add_at_head(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: None,
            prev: None,
        });
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(new_node));
            // NonNull 实现了Copy trait
            if let Some(old_node) = self.head {
                (*(new_node.as_ptr())).next = Some(old_node);
                (*(old_node.as_ptr())).prev = Some(new_node);
            } else {
                self.tail = Some(new_node);
            }
            self.head = Some(new_node);
            self.len += 1;
        }
    }

    fn add_at_tail(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: None,
            prev: None,
        });
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(new_node));
            // NonNull 实现了Copy trait
            if let Some(old_node) = self.tail {
                (*(new_node.as_ptr())).prev = Some(old_node);
                (*(old_node.as_ptr())).next = Some(new_node);
            } else {
                self.head = Some(new_node);
            }
            self.tail = Some(new_node);
            self.len += 1;
        }
    }

    fn add_at_index(&mut self, index: i32, val: T) {
        match index {
            0 => {
                self.add_at_head(val);
                return;
            }
            _ if self.len as i32 == index => {
                self.add_at_tail(val);
                return;
            }
            _ if index < 0 || index > self.len as i32 => panic!("index out of range"),
            _ => (),
        }
        let new_node = Box::new(Node {
            val,
            next: None,
            prev: None,
        });
        let mut cur_index = 0;
        let mut cur_node = self.head.unwrap();
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(new_node));
            while cur_index < self.len {
                if cur_index as i32 == index - 1 {
                    let front = cur_node;
                    let back = (*cur_node.as_ptr()).next.unwrap();
                    (*front.as_ptr()).next = Some(new_node);
                    (*new_node.as_ptr()).prev = Some(front);
                    (*new_node.as_ptr()).next = Some(back);
                    (*back.as_ptr()).prev = Some(new_node);
                    self.len += 1;
                    return;
                }
                cur_node = (*cur_node.as_ptr()).next.unwrap();
                cur_index += 1;
            }
        }
    }

    fn pop_at_head(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let ret = self.head;
        unsafe {
            if self.len == 1 {
                self.head = None;
                self.tail = None;
            } else {
                self.head = (*self.head.unwrap().as_ptr()).next.take();
                (*self.head.unwrap().as_ptr()).prev = None;
            }
            self.len -= 1;
            return Some((*ret.unwrap().as_ptr()).val.clone());
        }
    }
    fn pop_at_tail(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let ret = self.tail;
        unsafe {
            if self.len == 1 {
                self.head = None;
                self.tail = None;
            } else {
                self.tail = (*self.tail.unwrap().as_ptr()).prev;
                (*self.tail.unwrap().as_ptr()).next = None;
            }
            self.len -= 1;
            return Some((*ret.unwrap().as_ptr()).val.clone());
        }
    }

    fn delete_at_index(&mut self, index: i32) -> Option<T> {
        match index {
            0 => return self.pop_at_head(),
            _ if index == self.len as i32 - 1 => return self.pop_at_tail(),
            _ if index < 0 || index >= self.len as i32 => return None,
            _ => (),
        }

        let mut cur_index = 0;
        let mut cur_node = self.head.unwrap();

        while cur_index < self.len as i32 {
            unsafe {
                if cur_index == index - 1 {
                    let front = cur_node;
                    let target = (*cur_node.as_ptr()).next.unwrap();
                    let back = (*target.as_ptr()).next.unwrap();
                    (*front.as_ptr()).next = Some(back);
                    (*back.as_ptr()).prev = Some(front);
                    self.len -= 1;
                    return Some((*target.as_ptr()).val.clone());
                }
                cur_node = (*cur_node.as_ptr()).next.unwrap();
                cur_index += 1;
            }
        }
        None
    }
}

impl<T: Clone + Debug> MyLinkedList<T> {
    fn print(&self) {
        let mut cur_index = 0;
        let mut cur_node = self.head;
        while cur_index < self.len {
            unsafe {
                print!("{:?} ", (*cur_node.unwrap().as_ptr()).val);
                cur_node = (*cur_node.unwrap().as_ptr()).next;
            }
            cur_index = cur_index + 1;
        }
        println!("长度：{}", cur_index);
        self.print_from_tail();
        println!();
    }
    fn print_from_tail(&self) {
        let mut cur_index = 0;
        let mut cur_node = self.tail;
        let mut v = Vec::with_capacity(self.len);
        while cur_index < self.len {
            unsafe {
                v.push((*cur_node.as_ref().unwrap().as_ptr()).val.clone());
                cur_node = (*cur_node.as_ref().unwrap().as_ptr()).prev;
            }
            cur_index = cur_index + 1;
        }
        v.reverse();
        for i in v {
            print!("{:?} ", i);
        }
        println!("长度：{}", cur_index);
    }
}

fn main() {
    let mut list = MyLinkedList::new();
    list.add_at_head(11);
    let val = list.get(0);
    println!("{:?}", val);
    list.print();
    // 11
    list.pop_at_tail();
    list.print();

    list.add_at_head(12);
    list.print();
    list.add_at_head(13);
    list.print();

    // 13 12
    list.pop_at_tail();
    list.print();

    println!("---------------------------------------");
    let mut list = MyLinkedList::new();
    list.add_at_head(11);
    let val = list.get(0);
    println!("{:?}", val);
    list.print();
    // 11
    list.pop_at_head();
    list.print();

    list.add_at_head(12);
    list.print();
    list.add_at_head(13);
    list.print();

    // 13 12
    list.pop_at_head();
    list.print();

    println!("---------------------------------------");

    let mut list = MyLinkedList::new();
    list.add_at_tail(11);
    let val = list.get(0);
    println!("{:?}", val);
    list.print();
    list.add_at_tail(12);
    list.print();
    list.add_at_tail(13);
    list.print();
    // 11 12 13
    list.add_at_index(1, 14);
    // 11 14 12 13
    list.print();
    list.add_at_index(0, 15);
    // 15 11 14 12 13
    list.print();
    list.add_at_index(5, 16);
    // 15 11 14 12 13 16
    list.print();

    list.delete_at_index(0);
    // 11 14 12 13 16
    list.print();

    list.delete_at_index(4);
    // 11 14 12 13
    list.print();

    list.delete_at_index(2);
    // 11 14 13
    list.print();
}

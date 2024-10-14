use lazy_static::lazy_static;
use std::{fmt::Display, sync::Mutex};

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    host: String,
    port: u32,
}

#[allow(dead_code)]
impl Config {
    fn get_static_str(&self) -> &'static str {
        "static_str"
    }
    fn get_host_ref(&self) -> &String {
        &self.host
    }
}

lazy_static! {
    static ref LOCK1: Mutex<Config> = Mutex::new(Config {
        host: String::from("host"),
        port: 1234,
    });
}

lazy_static! {
    static ref LOCK2: Mutex<String> = Mutex::new(String::from("hello"));
}

// 用完后要及时让锁离开作用域，养成好习惯。
fn _bad_example1() {
    let s = LOCK2.lock().unwrap();
    println!("s = {}", s);
    // do something else
}

fn _good_example1() {
    {
        let s = LOCK2.lock().unwrap();
        println!("s = {}", s);
    }
    // do something else
}

// 临时变量无变量绑定，语句结束立即析构，锁会立即被释放
fn _good_example2() {
    let s = LOCK1.lock().unwrap().host.clone();
    println!("s = {}", s);
    // do something else
}

fn _bad_example2() {
    println!("bad example2");
    // 虽然临时变量会直接析构，但是锁的作用域依然会持续到函数的结束，
    // 因此请不要在同一个函数的参数中多次获取锁，即使是不同的锁(鬼知道他们之间会不会有关联)。
    _foo(LOCK1.lock().unwrap().port, LOCK1.lock().unwrap().port);
    println!("this will never be printed");
}

fn _bad_example22() {
    println!("bad example22");
    // 虽然临时变量会直接析构，但是锁的作用域依然会持续到match块的结束，
    // 同样的, if let 和 while let 也会造成同样的问题
    match LOCK1.lock().unwrap().host.is_empty() {
        true => println!("empty"),
        false => {
            println!("not empty");
            println!("host = {}", LOCK1.lock().unwrap().host); // 这里会死锁
            println!("this will never be printed");
        }
    }
}

fn _bad_example222() {
    println!("bad example222");
    // if不会像match一样，这里锁的作用域在进入if块后就结束了。
    // 但仍然不建议在if块中获取锁，不写含糊不清的代码，养成好习惯。
    if LOCK1.lock().unwrap().host.is_empty() {
        println!("not empty");
        println!("host = {}", LOCK1.lock().unwrap().host); // 这里不会死锁
    } else if LOCK1.lock().unwrap().port != 0 {
        println!("port = {}", LOCK1.lock().unwrap().port); // 这里不会死锁
    }
}

// 通过锁获取到的引用，会一直持有锁，直到引用离开作用域。
fn _bad_example3() {
    println!("bad example3");
    let s: &String = &LOCK1.lock().unwrap().host; // 这里的锁会一直持有，因为这里没有产生临时变量。
    println!("s = {}", s);
    LOCK1.lock().unwrap().host.push_str("xxxxxxxxxxxxxx"); // 这里会死锁
}

// 通过临时变量的锁获取到的'static生命周期的引用，不会造成死锁。
// 虽然如此，但还是建议只要发现获取到引用，就把锁放进一个单独的作用域，养成好习惯。
fn _good_example3() {
    println!("good example3");
    let s = LOCK1.lock().unwrap().get_static_str();
    println!("s = {}", s);
    LOCK1.lock().unwrap().host.push_str("xxxxxxxxxxxxxx");
}

fn _foo<T1: Display, T2: Display>(s: T1, s2: T2) -> Config {
    println!("s = {}{}", s, s2);
    Config {
        host: String::from("host"),
        port: 1234,
    }
}

fn main() {
    println!("Hello, world!");
    _good_example1();
    _good_example2();
    _good_example3();
}

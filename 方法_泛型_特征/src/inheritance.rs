// 基类
struct Animal {
    age: i32,
}

impl Animal {
    fn new(age: i32) -> Self {
        Animal { age }
    }

    // 基类方法，下面的代码中，它将会被子类复用
    fn speak(&self) {
        println!("I'm an animal. Age = {}", self.age);
    }
}

// 子类
struct Dog {
    supper: Animal,
    _name: String,
}

impl Dog {
    fn new(name: String, age: i32) -> Self {
        Dog {
            supper: Animal::new(age),
            _name: name,
        }
    }

    // 实现"复用父类的方法"
    fn speak(&self) {
        self.supper.speak();
    }
}

pub fn study_inheritance() {
    println!("----------------继承----------------");
    // Rust 不支持继承，但可以通过子类嵌套父类的方式，实现类似“继承”的效果，跟Go语言挺像的。
    // Don't look for tricks to simulate inheritance. Rust isn't designed this way and
    // it doesn't help to try to make it OOP.
    // 尽量不要去模拟继承，Rust并不是为了这样设计的，也不会帮助你去尝试OOP。
    let dog = Dog::new(String::from("Tom"), 2);
    dog.speak();
}

pub trait Sound {
    fn sound(&self) -> String;
}

#[derive(Debug)]
pub struct Cat {
    pub name: String,
}

impl Sound for Cat {
    fn sound(&self) -> String {
        format!("{}: miao miao miao~", self.name)
    }
}

pub struct Dog {
    pub name: String,
}

impl Sound for Dog {
    fn sound(&self) -> String {
        format!("{}: wang wang wang~", self.name)
    }
}

pub struct Pig {
    pub name: String,
}

impl Sound for Pig {
    fn sound(&self) -> String {
        format!("{}: ao ao ao~", self.name)
    }
}

fn make_sound(x: &impl Sound) -> String {
    x.sound()
}

// 虽然 `impl Trait` 这种语法非常好理解，但是实际上它只是一个语法糖，
// 本质上是一个 `trait bound` 泛型。
fn make_sound2<T: Sound>(x: &T) -> String {
    x.sound()
}

fn multi_dog_sound(x: Vec<&impl Sound>) {
    println!("many dogs sound: ");
    for i in x {
        println!("{}", i.sound());
    }
}

fn multi_animal_sound(x: Vec<Box<dyn Sound>>) {
    println!("many animals sound: ");
    for i in x {
        println!("{}", i.sound());
    }
}

pub fn study_polymorphism() {
    println!("----------------多态的例子----------------");
    let cat = Cat {
        name: String::from("Tom"),
    };
    let dog = Dog {
        name: String::from("Jack"),
    };
    let pig = Pig {
        name: String::from("Piggy"),
    };
    println!("{}", make_sound(&cat));
    println!("{}", make_sound(&dog));
    println!("{}", make_sound(&pig));
    println!("{}", make_sound2(&pig));

    let dog2 = Dog {
        name: String::from("snoopy"),
    };
    let dog3 = Dog {
        name: String::from("haha"),
    };
    // 多条狗的声音(同一类型，使用静态分发)
    println!("----------------多条狗的声音(同一类型，使用静态分发)----------------");
    let dogs = vec![&dog, &dog2, &dog3];
    multi_dog_sound(dogs);

    // vector中的元素类型不一致，无法使用静态分发
    // let animals = vec![&cat, &dog, &pig];

    // 多种动物的声音(不同类型，使用动态分发)
    println!("----------------多种动物的声音(不同类型，使用动态分发)----------------");
    let mut x: Vec<Box<dyn Sound>> = Vec::new();
    x.push(Box::new(cat));
    x.push(Box::new(dog));
    x.push(Box::new(pig));
    multi_animal_sound(x);
}

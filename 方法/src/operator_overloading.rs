use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    // fn add(self, rhs: Rhs) -> Self::Output;
    // 这个 trait 的定义中，`RHS` 是 `Right Hand Side` 的缩写，代表右操作数。
    // 这里的 `RHS=Self` 表示默认情况下右操作数是 实现该 trait 的类型本身。
    // 如果在实现该 trait 时指定了一个不同的类型作为右操作数，则可以使用另外的类型来替换 `RHS`。
    type Output = Point<T>;

    // 运算符重载
    fn add(self, p: Point<T>) -> Self {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn study_example() {
    study_example1();
}

fn study_example1() {
    println!("------------------运算符重载------------------");
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("p3: {:?}", p3);
    println!("p4: {:?}", p4);
    // 运算符重载
    println!("p3 + p4 = {:?}", p3 + p4);

    // 默认泛型类型参数(不同类型之间的运算符重载)
    println!("------------------默认泛型类型参数(不同类型之间的运算符重载)------------------");
    let p5 = Point3D {
        x: 1i32,
        y: 1i32,
        z: 1i32,
    };
    let p6 = Point2D { x: 2i32, y: 2i32 };
    println!("p5: {:?}", p5);
    println!("p6: {:?}", p6);
    // 运算符重载
    println!("p5 + p6 = {:?}", p5 + p6);
}

// 三维坐标
#[derive(Debug)]
struct Point3D<T: Add<T, Output = T>> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug)]
struct Point2D<T> {
    x: T,
    y: T,
}

// RHS = Point2D<T>
impl<T> Add<Point2D<T>> for Point3D<T>
where
    T: Add<T, Output = T>, // 限制类型T必须实现了Add特征，否则无法进行+操作。
{
    type Output = Point3D<T>;

    fn add(self, p: Point2D<T>) -> Self {
        Point3D {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z,
        }
    }
}

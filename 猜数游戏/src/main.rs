use rand::Rng;
use std::io::Write; //io::stdout().flush() 是Write trait的一个方法，所以需要导入
fn main() {
    println!("猜数游戏！！！");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("秘密数字是：{}", secret_number);
    let mut guess = String::new();

    loop {
        print!("猜一个数：");
        // 默认情况下stdout通常是行缓冲的，因此可能需要使用io::stdout().flush()以确保输出到达终端。
        std::io::stdout().flush().unwrap(); // 刷新缓冲区

        guess.clear();
        std::io::stdin().read_line(&mut guess).expect("读取行失败");
        // parse 返回一个 Result 类型，它是一个枚举，它的成员是 Ok 或 Err。
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{} 不是一个数字: {}", guess.trim(), e);
                continue;
            }
        };
        println!("你猜的数是：{} ", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("\x1b[1;32mYou win!\x1b[0m");
                break;
            }
        }
        println!();
    }
}

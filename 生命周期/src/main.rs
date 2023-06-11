mod advance;

/*
    为什么需要生命周期，来看一个C++一个潜在的坑：

    const string& foo(const string& a, const string& b) {
        return a.empty() ? b : a;
    }
    int main() {
        auto& s = foo("", "foo");
        cout << s << '\n';
    }

    // 上面代码能正常编译和运行，但其实有很大的问题，
    // 在函数 foo 中，返回了一个常量引用，但是该引用所绑定的变量a和b可能会被销毁，
    // 这意味着在main函数中使用s变量时，它将引用一个已经被销毁的对象，
    // 从而导致未定义行为（Undefined Behavior），这就是Rust中生命周期的意义。

*/

fn main() {
    // 生命周期
    stuidy_lifetime();

    // 生命周期进阶
    advance::stuidy_lifetime_advance();
}

/*
编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。
其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。
若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

1. 每一个引用参数都会获得独自的生命周期标注。

    例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，
    两个引用参数的有两个生命周期标注:`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。

2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，
   也就是所有返回值的生命周期都等于该输入生命周期

    例如函数 `fn foo(x: &i32) -> &i32`，`x` 参数的生命周期会被自动赋给返回值 `&i32`，
    因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`

3. 若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期。

   拥有 `&self` 形式的参数，说明该函数是一个 `方法`，该规则让方法的使用便利度大幅提升。

 */

fn stuidy_lifetime() {
    println!("-----------------函数中的生命周期-----------------");
    // 生命周期，简而言之就是引用的有效作用域。在大多数时候，我们无需手动的声明生命周期，
    // 因为编译器可以自动进行推导。
    // 在多种类型存在时，编译器往往要求我们手动标明类型 <-> 。
    // 当多个生命周期存在，且编译器无法推导出某个引用的生命周期时，就需要我们手动标明生命周期。
    // 在编译期，Rust 会比较两个变量的生命周期，如果其中一个变量引用的那个变量的生命周期比当前变量的生命周期短，
    // 那么编译器就会报错。想要通过编译，也很简单，只要保证引用的那个变量的生命周期比当前变量的生命周期长就行了。

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let mut result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 当把具体的引用传给 longest 时，那生命周期 'a 的大小就是 x 和 y 的作用域的重合部分，
    // 换句话说，'a 的大小将等于 x 和 y 中较小的那个。
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // 由于返回值的生命周期也被标记为 'a，因此返回值的生命周期也是 x 和 y 中作用域较小的那个。
        // 也就是说，离开大括号后，返回值的生命周期就结束了，Result 也将无法使用，尽管返回了string1的引用。
        // 作为人类，我们可以很清晰的看出 `result` 实际上引用了 `string1`，
        // 因为 `string1` 的长度明显要比 `string2` 长，既然如此，编译器不该如此矫情才对，
        // 它应该能认识到 `result` 没有引用 `string2`，让我们这段代码通过。
        // 只能说，作为尊贵的人类，编译器的发明者，你高估了这个工具的能力，它真的做不到！
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result);
    //                                       ^^^^^^ `string2` does not live long enough
    //                                       borrowed value does not live long enough

    // 结构体中的生命周期
    println!("-----------------结构体中的生命周期-----------------");
    // 不仅仅函数具有生命周期，结构体其实也有这个概念，只不过我们之前对结构体的使用都停留在非引用类型字段上。
    // 之前为什么不在结构体中使用字符串字面量或者字符串切片，而是统一使用 `String` 类型？
    // 原因很简单，后者在结构体初始化时，只要转移所有权即可，而前者，抱歉，它们是引用，它们不能为所欲为。
    // 既然之前已经理解了生命周期，那么意味着在结构体中使用引用也变得可能：
    // 只要为结构体中的每一个引用标注上生命周期即可!
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // 从函数实现来看，`ImportantExcerpt` 的生命周期从开始，到 函数末尾结束，
    // 而该结构体引用的字符串从上一行开始，也是到 `main` 函数末尾结束，
    // 可以得出结论: 结构体引用的字符串活得比结构体久，这符合了编译器对生命周期的要求，因此编译通过。
    println!("The first sentence is {}", i.part);

    // 方法中的生命周期
    println!("-----------------方法中的生命周期-----------------");
    // 为具有生命周期的结构体实现方法时，我们使用的语法跟泛型参数语法很相似：
    // impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is {}", i.part);
    println!(
        "announce_and_return_part:{}",
        i.announce_and_return_part("hello")
    );
    println!("level:{}", i.level());
    println!(
        "announce_and_return_part2:{}",
        i.announce_and_return_part2("hello")
    );

    // 静态生命周期
    println!("-----------------静态生命周期-----------------");
    // 在 Rust 中有一个非常特殊的生命周期，那就是 'static，拥有该生命周期的引用可以和整个程序活得一样久。
    // 在之前我们学过字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中，
    // 因此这些字符串变量全部具有 'static 的生命周期：
    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
    println!("s:{}", s);
    // 这时候，有些聪明的小脑瓜就开始开动了：当生命周期不知道怎么标时，
    // 对类型施加一个静态生命周期的约束 T: 'static 是不是很爽？这样我和编译器再也不用操心它到底活多久了。
    // 个想法是对的，在不少情况下，`'static` 约束确实可以解决生命周期编译不通过的问题，但是问题来了：
    // 本来该引用没有活那么久，但是你非要说它活那么久，万一引入了潜在的 BUG 怎么办？
    // 因此，遇到因为生命周期导致的编译不通过问题，首先想的应该是：是否是我们试图创建一个悬垂引用，
    // 或者是试图匹配不一致的生命周期，而不是简单粗暴的用 'static 来解决问题。

    // 一个复杂例子: 泛型、特征约束，谢幕生命周期的初见。
    println!("-----------------一个复杂例子: 泛型、特征约束-----------------");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result;
    {
        let string3 = String::from("long string is long");
        result = longest_with_an_announcement(string1.as_str(), string2, string3);
        println!("The longest string is {}", result);
    }
    // 又一个例子
    println!("-----------------又一个例子-----------------");
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
    // println!("The 10 bytes at 0x{:X} stored: {}", 1000, message);
}

fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
    // string为持有引用的变量，生命周期的标注是对于引用来说的，而不是变量。
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer as *const u8, length))
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 为具有生命周期的结构体实现方法时，我们使用的语法跟泛型参数语法很相似：
// impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
impl<'a> ImportantExcerpt<'a> {
    // 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 'a: 'b 是生命周期约束语法，跟泛型约束非常相似，用于说明 `'a` 必须比 `'b` 活得久
// 'a: 'b 可以放在where子句中
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    // 我们做一个有趣的修改，将方法返回的生命周期改为`'b`：
    // 由于 `&'a self` 是被引用的一方，因此引用它的 `&'b str` 必须要活得比它短，否则会出现悬垂引用。
    fn announce_and_return_part2(&'a self, announcement: &'b str) -> &'b str {
        // 我们告诉了编译器，这个方法返回了一个字符串引用，它的生命周期是 b ,
        // b 的生命周期一定不会超过 a 的生命周期，这样就能暗示编译器，
        // 尽管引用吧，反正返回的引用比较短命，爱咋咋地，怎么都不会引用到无效的内容！
        println!("Attention please: {}", announcement);
        self.part
    }
}

struct ImportantExcerpt<'a> {
    // 该生命周期标注说明，part引用的变量的生命周期 至少 和ImportantExcerpt的生命周期一样长。
    // 也就是说，结构体 `ImportantExcerpt` 所引用的字符串 `str` 必须比该结构体活得更久。
    part: &'a str,
}

// 这段 `longest` 实现，非常标准优美，就连多余的 `return` 和分号都没有，可是现实总是给我们重重一击：
// error[E0106]: missing lifetime specifier
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `x` or `y
// 其实主要是编译器无法知道该函数的返回值到底引用 x 还是 y ，
// 因为编译器需要知道这些，来确保函数调用后的引用生命周期分析 !
// 生命周期标注并不会改变任何引用的实际作用域 -- 鲁迅
// 鲁迅说过的话，总是值得重点标注，当你未来更加理解生命周期时，你才会发现这句话的精髓和重要！
// 现在先简单记住，标记的生命周期只是为了取悦编译器，让编译器不要难为我们。
// 例如一个变量，只能活一个花括号，那么就算你给它标注一个活全局的生命周期，
// 它还是会在前面的花括号结束处被释放掉，并不会真的全局存活。
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 生命周期的语法也颇为与众不同，以 ' 开头，
// 和泛型一样，使用生命周期参数，需要先声明 <'a>
// 名称往往是一个单独的小写字母，大多数人都用 'a 来作为生命周期的名称。
// 一个生命周期标注，它自身并不具有什么意义，因为生命周期的作用就是告诉编译器多个引用之间的关系。
// 例如，有一个函数，它的第一个参数 first 是一个指向 i32 类型的引用，具有生命周期 'a，
// 该函数还有另一个参数 second，它也是指向 i32 类型的引用，并且同样具有生命周期 'a。
// 此处生命周期标注仅仅说明，这两个参数 first 和 second **至少** 活得和 'a 一样久，
// 至于到底活多久或者哪个活得更久，抱歉我们都无法得知。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)。
    // 虽然两个参数的生命周期都是标注了 'a，但是实际上这两个参数的真实生命周期可能是不一样的
    // (生命周期 'a 不代表生命周期等于 'a，而是大于等于 'a)。
    // 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，
    // 而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
    // 因此 longest 函数并不知道 `x` 和 `y` 具体会活多久，只要知道它们的作用域至少能持续 'a 这么长就行。
    if x.len() > y.len() {
        x
    } else {
        y
    }
    // 'a 对于单个变量来说，表示这个变量的生命周期 至少 和 'a 一样长。
}

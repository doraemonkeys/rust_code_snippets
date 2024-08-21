fn main() {
    // 细心的读者可能会注意到 println! 后面跟着的是 ()，而 vec! 后面跟着的是 []，
    // 这是因为宏的参数可以使用 ()、[] 以及 {}:
    println!("aaaa");
    println!["aaaa"];
    println! {"aaaa"}

    // 在 Rust 中宏分为两大类：声明式宏( \*declarative macros\* ) macro_rules!
    // 和三种过程宏( \*procedural macros\* ):
    // - #[derive]，在之前多次见到的派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征
    // - 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性
    // - 类函数宏(Function-like macro)，看上去就像是函数调用

    println!("---------------------元编程---------------------");
    // 从根本上来说，宏是通过一种代码来生成另一种代码，如果大家熟悉元编程，就会发现两者的共同点。
    // 在中讲到的 derive 属性，就会自动为结构体派生出相应特征所需的代码，
    // 例如 #[derive(Debug)]，还有熟悉的 println! 和 vec!，所有的这些宏都会展开成相应的代码，
    // 且很可能是长得多的代码。
    println!("---------------------可变参数---------------------");
    // Rust 的函数签名是固定的：定义了两个参数，就必须传入两个参数，多一个少一个都不行，
    // 对于从 JS/TS 过来的同学，这一点其实是有些恼人的。
    // 而宏就可以拥有可变数量的参数，例如可以调用一个参数的 `println!("hello")`，
    // 也可以调用两个参数的 `println!("hello {}", name)`。
    println!("---------------------宏展开---------------------");
    // 由于宏会被展开成其它代码，且这个展开过程是发生在编译器对代码进行解释之前。
    // 因此，宏可以为指定的类型实现某个特征：先将宏展开成实现特征的代码后，再被编译。
    // 而函数就做不到这一点，因为它直到运行时才能被调用，而特征需要在编译期被实现。

    // 声明式宏 macro_rules!
    study_macro_rules();

    // 用过程宏为属性标记生成代码
    study_attribute_macro();
}

fn study_attribute_macro() {
    println!("---------------------用过程宏为属性标记生成代码---------------------");
    // 从形式上来看，过程宏跟函数较为相像，但过程宏是使用源代码作为输入参数，
    // 基于代码进行一系列操作后，再输出一段全新的代码。
    // 注意，过程宏中的 derive 宏输出的代码并不会替换之前的代码，这一点与声明宏有很大的不同！

    // 当创建过程宏时，它的定义必须要放入一个独立的包中，且包的类型也是特殊的，
    // 这么做的原因相当复杂，大家只要知道这种限制在未来可能会有所改变即可。

    // 假设我们要创建一个 `derive` 类型的过程宏：
    // use proc_macro;
    // #[proc_macro_derive(HelloMacro)]
    // pub fn some_name(input: TokenStream) -> TokenStream {}
    // 用于定义过程宏的函数 some_name 使用 TokenStream 作为输入参数，
    // 并且返回的也是同一个类型。TokenStream 是在 proc_macro 包中定义的，
    // 顾名思义，它代表了一个 Token 序列。

    // 自定义 derive 过程宏
    println!("---------------------自定义 derive 过程宏---------------------");
    // 假设我们有一个特征 HelloMacro,
    // 使用过程宏来统一实现该特征，这样用户只需要对类型进行标记即可：#[derive(HelloMacro)]
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Sunfei;

    #[derive(HelloMacro)]
    struct Sunface;

    Sunfei::hello_macro();
    Sunface::hello_macro();
}

fn study_macro_rules() {
    println!("---------------------声明式宏 macro_rules!---------------------");
    // 在 Rust 中使用最广的就是声明式宏，它们也有一些其它的称呼，
    // 例如示例宏( macros by example )、macro_rules! 或干脆直接称呼为宏。
    // 声明式宏允许我们写出类似 match 的代码。match 表达式是一个控制结构，
    // 其接收一个表达式，然后将表达式的结果与多个模式进行匹配，一旦匹配了某个模式，则该模式相关联的代码将被执行。

    // 而宏也是将一个值跟对应的模式进行匹配，且该模式会与特定的代码相关联。
    // 但是与 `match` 不同的是，宏里的值是一段 Rust 源代码(字面量)，
    // 模式用于跟这段源代码的结构相比较，一旦匹配，传入宏的那段源代码将被模式关联的代码所替换，
    // 最终实现宏展开。值得注意的是，所有的这些都是在编译期发生，并没有运行期的性能损耗。

    // 使用 `vec!` 来便捷的初始化一个动态数组:
    let _v = vec![1, 2, 3];
    // 通过 `vec!` 创建的动态数组支持任何元素类型，也并没有限制数组的长度，
    // 如果使用函数，我们是无法做到这一点的。
    // 好在我们有 `macro_rules!`，来看看该如何使用它来实现 `vec!`，以下是一个简化实现：
    // 实际上标准库中的 `vec!` 还包含了预分配内存空间的代码。

    // `#[macro_export]` 注释将宏进行了导出，这样其它的包就可以将该宏引入到当前作用域中，然后才能使用。
    // 使用 `macro_rules!` 进行了宏定义，需要注意的是宏的名称是 vec，而不是 vec!，后者的感叹号只在调用时才需要。

    // vec 的定义结构跟 match 表达式很像，但这里我们只有一个分支，其中包含一个模式 ( $( $x:expr ),* )，
    // 跟模式相关联的代码就在 => 之后。一旦模式成功匹配，那这段相关联的代码就会替换传入的源代码。
    // 由于 vec 宏只有一个模式，因此它只能匹配一种源代码，其它类型的都将导致报错，
    // 而更复杂的宏往往会拥有更多的分支。

    #[macro_export]
    macro_rules! myvec {
    ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let _v = myvec![1, 2, 3];
    // 首先，我们使用圆括号 () 将整个宏模式包裹其中。紧随其后的是 $()，
    // 跟括号中模式相匹配的值(传入的 Rust 源代码)会被捕获，然后用于代码替换。
    // 在这里，模式 $x:expr 会匹配任何 Rust 表达式并给予该模式一个名称：$x。
    // $() 之后的逗号说明在 $() 所匹配的代码的后面会有一个可选的逗号分隔符，
    // 紧随逗号之后的 * 说明 * 之前的模式会被匹配零次或任意多次(类似正则表达式)。
    //  当我们使用 vec![1, 2, 3] 来调用该宏时，$x 模式将被匹配三次，
    //  分别是 1、2、3。为了帮助大家巩固，我们再来一起过一下：
    // 1. $() 中包含的是模式 $x:expr，该模式中的 expr 表示会匹配任何 Rust 表达式，并给予该模式一个名称 $x
    // 2. 因此 $x 模式可以跟整数 1 进行匹配，也可以跟字符串 "hello" 进行匹配: vec!["hello", "world"]
    // 3. $() 之后的逗号，意味着1 和 2 之间可以使用逗号进行分割，也意味着 3 既可以没有逗号，也可以有逗号：vec![1, 2, 3,]
    // 4. * 说明之前的模式可以出现零次也可以任意次，这里出现了三次。

    // `temp_vec.push()` 将根据模式匹配的次数生成对应的代码，当调用 `vec![1, 2, 3]` 时，
    // 下面这段生成的代码将替代传入的源代码，也就是替代 `vec![1, 2, 3]` :
    // let _v = {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }

    #[allow(dead_code)]
    trait ToFloat64 {
        fn to_f64(&self) -> Option<f64>;
    }

    // valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
    // 这里ty表示类型，而expr表示表达式
    macro_rules! impl_to_float64 {
        ($($t:ty),*) => {
            $(
                impl ToFloat64 for $t {
                    fn to_f64(&self) -> Option<f64> {
                        Some(*self as f64)
                    }
                }
            )*
        };
    }

    impl_to_float64!(u8, u16, u32, u64, u128, i32, i64, f64, f32);

    // 未来将被替代的 `macro_rules`
    println!("---------------------未来将被替代的 macro_rules---------------------");
    // 对于 `macro_rules` 来说，它是存在一些问题的，因此，Rust 计划在未来使用新的声明式宏来替换它。
}

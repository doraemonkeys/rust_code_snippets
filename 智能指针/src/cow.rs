pub fn study_cow() {
    println!("---------------------------------study Cow---------------------------------");
    use std::borrow::Cow;
    // Cow是Rust提供的用于实现写时克隆(Clone on write)的智能指针。
    // 从Cow的定义看，它是一个enum，包含一个对类型B的只读引用，或者包含一个拥有类型B的所有权的数据。
    // 使用Cow使我们在返回数据时提供了两种可能:
    // 要么返回一个借用的数据(只读)，要么返回一个拥有所有权的数据(可读写)。
    // 使用Cow主要用来减少内存的分配和复制，因为绝大多数的场景都是读多写少。
    // 使用Cow可以在需要些的时候才做一次内存复制，这样就很大程度减少了内存复制次数。
    fn abs_all(input: &mut Cow<'_, [i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // Clones into a vector if not already owned.
                input.to_mut()[i] = -v;
            }
        }
    }

    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);

    // Another example showing how to keep Cow in a struct:
    struct Items<'a, X>
    where
        [X]: ToOwned<Owned = Vec<X>>,
    {
        values: Cow<'a, [X]>,
    }

    impl<'a, X: Clone + 'a> Items<'a, X>
    where
        [X]: ToOwned<Owned = Vec<X>>,
    {
        fn new(v: Cow<'a, [X]>) -> Self {
            Items { values: v }
        }
    }

    // Creates a container from borrowed values of a slice
    let readonly = [1, 2];
    let borrowed = Items::new(Cow::Borrowed(&readonly[..]));
    match borrowed {
        Items {
            values: Cow::Borrowed(b),
        } => println!("borrowed {b:?}"),
        _ => panic!("expect borrowed value"),
    }

    let mut clone_on_write = borrowed;
    // Mutates the data from slice into owned vec and pushes a new value on top
    clone_on_write.values.to_mut().push(3);
    println!("clone_on_write = {:?}", clone_on_write.values);

    // The data was mutated. Let's check it out.
    match clone_on_write {
        Items {
            values: Cow::Owned(_),
        } => println!("clone_on_write contains owned data"),
        _ => panic!("expect owned data"),
    }

    // 一个例子
    cow_example();
}

fn cow_example() {
    println!("---------------------------------cow_example---------------------------------");
    use std::borrow::Cow;
    // 实现一个字符串敏感词替换函数，从给定的字符串替换掉预制的敏感词。
    const SENSITIVE_WORD: &str = "bad";

    fn remove_sensitive_word<'a>(words: &'a str) -> Cow<'a, str> {
        if words.contains(SENSITIVE_WORD) {
            Cow::Owned(words.replace(SENSITIVE_WORD, ""))
        } else {
            Cow::Borrowed(words)
        }
    }
    fn remove_sensitive_word_old(words: &str) -> String {
        if words.contains(SENSITIVE_WORD) {
            words.replace(SENSITIVE_WORD, "")
        } else {
            words.to_owned()
        }
    }
    // 例子中给出了remove_sensitive_word和remove_sensitive_word_old两种实现，
    // 前者的返回值使用了Cow，后者返回值使用的是String。仔细分析一下，很明显前者的实现效率更高。
    // 因为如果输入的字符串中没有敏感词时，前者Cow::Borrowed(words)不会发生堆内存的分配和拷贝，
    // 后者words.to_owned()会发生一次堆内存的分配和拷贝。

    let words = "I'm a bad boy.";
    let new_words = remove_sensitive_word(words);
    println!("{}", new_words);

    let new_words = remove_sensitive_word_old(words);
    println!("{}", new_words);
}

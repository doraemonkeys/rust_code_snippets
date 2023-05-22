use bytes::{Buf, BufMut};
use bytes::{Bytes, BytesMut};

fn main() {
    // 使用 Vec<u8> 来保存目标数据的时候，有一个问题，
    // 对它进行克隆时会将底层数据也整个复制一份，效率很低
    // Bytes 是一个引用计数类型，跟 Arc 非常类似，或者准确的说，
    // Bytes 就是基于 Arc 实现的，但相比后者Bytes 提供了一些额外的能力。

    println!("-----------------Bytes-----------------");
    // example1
    let mut mem = Bytes::from("Hello world");
    let a = mem.slice(0..5);

    assert_eq!(a, "Hello");

    // 一分为二，返回前半部分，后半部分保留在mem中(底层数据没有复制，仅改变了指针)
    // this is an O(1) operation that just increases the reference count and sets a few indices.
    let b = mem.split_to(6);

    assert_eq!(mem, "world");
    assert_eq!(b, "Hello ");

    println!("-----------------BytesMut-----------------");
    // BytesMut 是 Bytes 的可变版本

    // example1
    // 创建一个容量为1024字节的 BytesMut,超过这个容量时会自动扩容
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    assert_eq!(11, buf.len());
    assert_eq!(1024, buf.capacity());

    // 清空buf,并返回一个新的BytesMut(底层数据没有复制，仅改变了指针)
    // This is an O(1) operation that just increases the reference count and sets a few indices.
    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(1013, buf.capacity()); // 1024 - 11 = 1013

    assert_eq!(other, b"hello world"[..]);
    assert_eq!(11, other.capacity());

    // example2
    let mut buf = BytesMut::with_capacity(64);
    buf.put_u8(b'h');
    buf.put_u8(b'e');
    buf.put(&b"llo"[..]);
    assert_eq!(&buf[..], b"hello");
    // Freeze the buffer so that it can be shared
    // 转换为Bytes,底层数据没有复制，仅改变了指针
    let a = buf.freeze();
    // This does not allocate, instead `b` points to the same memory.
    let b = a.clone();
    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"hello");

    // Buf, BufMut trait
    println!("-----------------Buf, BufMut-----------------");
    // 这两个特征提供了对缓冲区的读写访问，
    // 底层的数据在内存中可能是连续的，也可能是分散的，
    // 例如，Bytes是一个保证内存连续的缓冲区，但rope将字节存储在不相交的块中。
    // Buf和BufMut维护游标，跟踪底层字节存储中的当前位置。在读取或写入字节时，向前移动游标。
    // 乍一看，似乎Buf和BufMut在功能上与std::io::Read和std::io::Write重叠。然而，它们服务于不同的目的。
    // 读写操作可能会执行一个系统调用，这可能会失败。对Buf和BufMut的操作是绝对正确的。

    // 创建一个可变缓冲区
    let mut buf = BytesMut::new();
    // 向缓冲区写入数据
    buf.put_u8(10); //需要实现BufMut trait
    buf.put_slice(b"hello world");

    // 读取数据
    let mut cursor = buf.freeze();

    // 从缓冲区中读取数据并移动游标
    let num = cursor.get_u8(); //需要实现Buf trait
    let message = String::from_utf8_lossy(cursor.chunk());

    println!("Number: {}", num);
    println!("Message: {}", message);
}

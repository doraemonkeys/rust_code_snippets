use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

// `% cat path` 的简单实现
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// `% echo s > path` 的简单实现
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

// `% touch path` 的简单实现（忽略已存在的文件）
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir a`");
    // 创建一个目录，返回 `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => {
            println!("! {:?}", why.kind());
            if why.kind() == io::ErrorKind::AlreadyExists {
                println!("目录已存在");
            }
        }
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    // 前面的匹配可以用 `unwrap_or_else` 方法简化
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // 递归地创建一个目录，返回 `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // 创建一个符号链接，返回 `io::Resutl<()>`
    // if cfg!(target_family = "unix") {
    //     unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
    //         println!("! {:?}", why.kind());
    //     });
    // }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // 读取目录的内容，返回 `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rm a/c/e.txt`");
    // 删除一个文件，返回 `io::Result<()>`
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // 移除一个空目录，返回 `io::Result<()>`
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // 打开文件 open
    println!("-----------------------------打开文件 open-----------------------------");
    // open 静态方法能够以只读模式（read-only mode）打开一个文件。
    // File 拥有资源，即文件描述符（file descriptor），它会在自身被 drop 时关闭文件。
    // 创建指向所需的文件的路径
    let path = Path::new("hello.txt");

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {:?}", path.display(), why),
        Ok(file) => file,
    };
    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", path.display(), why),
        Ok(_) => print!("{} contains:\n{}", path.display(), s),
    }
    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭

    println!("-----------------------------以追加模式打开-----------------------------");
    let mut file = OpenOptions::new()
        .append(true)
        .open("hello.txt")
        .expect("cannot open file");

    file.write_all("www.xxxx.cn".as_bytes())
        .expect("write failed");
    println!("-----------------------------缓冲写入-----------------------------");
    let mut file_buffer = std::io::BufWriter::new(file);
    file_buffer
        .write("www.xxxx.cn".as_bytes())
        .expect("write failed");
    file_buffer.flush().expect("flush failed");

    // 创建文件 create
    println!("-----------------------------创建文件 create-----------------------------");
    static LOREM_IPSUM: &'static str = "xxxxxxxxxxxxxxxxxxxxxxxxx";

    use std::io::prelude::*;

    let path = Path::new("./lorem_ipsum.txt");

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", path.display(), why),
        Ok(file) => file,
    };

    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {:?}", path.display(), why)
        }
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }
}

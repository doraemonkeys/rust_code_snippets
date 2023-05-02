pub fn run(cnf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&cnf.file_path)?;
    let result = if cnf.ignore_case {
        search_case_insensitive(&cnf.query, &contents)
    } else {
        search(&cnf.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // &[String] 是数组切片
    // pub fn build(args: &[String]) -> Result<Config, &str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     // grep How poem.txt -i
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     // 使用环境变量来判断是否忽略大小写。
    //     //  is_ok 方法是 Result 提供的，用于检查是否有值，有就返回 true，没有则返回 false
    //     let ignore_case = std::env::var("IGNORE_CASE").is_ok();
    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }

    // 数组索引会越界，为了安全性和简洁性，使用 Iterator 特征自带的 next 方法是一个更好的选择:
    pub fn build<'a>(args: impl Iterator<Item = &'a String>) -> Result<Config, &'static str> {
        let mut args = args.skip(1);
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //println!("query: {}", query);
    //println!("contents: {}", contents);

    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// in src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

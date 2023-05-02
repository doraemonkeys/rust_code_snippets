// cargo run -- searchstring example-filename.txt

// cargo run -- How poem.txt

// $env:IGNORE_CASE=1;cargo run -- to poem.txt
use minigrep::Config;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query_cnf = Config::build(args.iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    println!("Searching for {}", query_cnf.query);
    println!("In file {}", query_cnf.file_path);

    if let Err(e) = minigrep::run(&query_cnf) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

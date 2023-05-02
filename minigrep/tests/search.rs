#[cfg(test)]
mod tests {

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            minigrep::search(query, contents)
        );
    }
}

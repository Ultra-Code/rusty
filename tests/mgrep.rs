use book::mgrep;

#[test]
fn one_result() {
    let query = "duct";
    // the backslash after the opening double quote tells Rust not to put a
    // newline character at the beginning of the contents of this string
    // literal
    let content = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        mgrep::search(query, content, book::Case::Sensitive)
    );
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        mgrep::search(query, contents, book::Case::Sensitive)
    );
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
        mgrep::search(query, contents, mgrep::Case::Insensitive)
    );
}

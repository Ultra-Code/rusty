pub mod cli;

use std::env;

pub enum Case {
    Sensitive,
    Insensitive,
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn search<'a>(query: &str, content: &'a str, case: Case) -> Vec<&'a str> {
    let mut found = vec![];
    match case {
        Case::Insensitive => {
            let query = query.to_lowercase();

            for line in content.lines() {
                if line.to_lowercase().contains(&query) {
                    found.push(line);
                }
            }
        }
        Case::Sensitive => {
            for line in content.lines() {
                if line.contains(query) {
                    found.push(line);
                }
            }
        }
    };

    found
}

#[must_use]
pub fn case() -> Case {
    env::var("IGNORE_CASE")
        .map_or_else(|_| Case::Sensitive, |_| Case::Insensitive)
}
